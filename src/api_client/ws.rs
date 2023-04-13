use async_trait::async_trait;
use serde::Serialize;
use tokio::{sync::mpsc::UnboundedSender, task::JoinHandle};
use tokio_stream::wrappers::UnboundedReceiverStream;

use super::{ApiConfiguration, ApiError};
use crate::{model::WsResponse, query::SavedLoginCredentials};

type WsListenItem = Result<WsResponse, ApiError>;
pub type ReceiverContainer =
	std::sync::Arc<tokio::sync::Mutex<UnboundedReceiverStream<WsListenItem>>>;

pub struct Client {
	receive: ReceiverContainer,
	handle: JoinHandle<()>,
	internal_client: ezsockets::Client<InternalClientExt>,
}

struct InternalClientExt {
	received_sender: UnboundedSender<WsListenItem>,
}

impl InternalClientExt {
	/// Turns a WS receiving channel to an async streams
	fn send_ws_msg(&self, bytes: &[u8]) {
		let res: WsListenItem =
			serde_json::from_slice::<WsResponse>(bytes).map_err(ApiError::from);
		self.received_sender.send(res).ok();
	}
}
#[async_trait]
impl ezsockets::ClientExt for InternalClientExt {
	type Call = ();

	async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
		self.send_ws_msg(text.as_bytes());
		Ok(())
	}

	async fn on_binary(
		&mut self, bytes: Vec<u8>,
	) -> Result<(), ezsockets::Error> {
		self.send_ws_msg(&bytes);
		Ok(())
	}

	async fn on_call(
		&mut self, _params: Self::Call,
	) -> Result<(), ezsockets::Error> {
		Ok(())
	}
}

impl Client {
	pub async fn new(
		config: &ApiConfiguration, auth: &SavedLoginCredentials,
	) -> Result<Self, ApiError> {
		use serde::ser::Error;

		let mut headers = config.to_headers().map_err(|e| {
			serde_json::Error::custom(
				"Couldn't parse config into headers: ".to_string() + &e.to_string(),
			)
		})?;
		headers.append(&mut auth.to_headers().map_err(|e| {
			serde_json::Error::custom(
				"Couldn't parse auth into headers: ".to_string() + &e.to_string(),
			)
		})?);

		let mut ws_config = ezsockets::ClientConfig::new(crate::API_V1_WS_URL);
		for (header_name, header_value) in headers {
			ws_config = ws_config.header(header_name, header_value);
		}

		let (received_sender, received_receiver) =
			tokio::sync::mpsc::unbounded_channel::<WsListenItem>();

		let (internal_client, future) = ezsockets::connect(
			|_client| InternalClientExt { received_sender },
			ws_config,
		)
		.await;

		let handle = tokio::spawn(async move {
			future.await.unwrap();
		});

		internal_client.call(());

		let ws_client = Self {
			internal_client,
			handle,
			receive: std::sync::Arc::new(tokio::sync::Mutex::new(
				UnboundedReceiverStream::from(received_receiver),
			)),
		};

		Ok(ws_client)
	}

	/// Sends a WS message to the CVR API.
	///
	/// # Errors
	///
	/// If something with the request failed.
	pub fn send(
		&self, requestable: impl crate::query::Requestable + Serialize + Send,
	) -> Result<(), ApiError> {
		let data = crate::query::RequestWrapper {
			request_type: requestable.request_type(),
			data: requestable,
		};
		let data = serde_json::to_vec(&data)?;
		self.internal_client.binary(data);

		Ok(())
	}

	pub fn listen(&self) -> ReceiverContainer { self.receive.clone() }
}

impl Drop for Client {
	fn drop(&mut self) { self.handle.abort(); }
}
