use super::ApiError;
use crate::{model::WsResponse, query::SavedLoginCredentials};
use futures_util::SinkExt;
use serde::Serialize;
use tokio_stream::{
	wrappers::{ReceiverStream, UnboundedReceiverStream},
	StreamExt,
};
use tokio_tungstenite::tungstenite::Message;

type WsListenItem = Result<WsResponse, ApiError>;
pub type ReceiverContainer =
	std::sync::Arc<tokio::sync::Mutex<UnboundedReceiverStream<WsListenItem>>>;

pub struct Client {
	send: tokio::sync::mpsc::Sender<Message>,
	receive: ReceiverContainer,
}

enum WsMultiplexMessage {
	Send(Message),
	Receive(Result<Message, ApiError>),
}

impl Client {
	pub async fn new(
		user_agent: String,
		auth: SavedLoginCredentials,
	) -> Result<Self, ApiError> {
		use base64::Engine as _;

		use serde::ser::Error;

		// WebSocket protocol is dumb, just why.... this is useless
		// And yes it needs to be exactly 16 bytes after decoding
		let rand_base_64 =
			base64::engine::general_purpose::URL_SAFE.encode(rand::random::<[u8; 16]>());

		// This is pain, need to follow
		// https://github.com/snapview/tungstenite-rs/issues/327
		let request: http::Request<()> = http::Request::get(crate::API_V1_WS_URL)
			.header("User-Agent", user_agent.clone())
			.header("Username", auth.username.clone())
			.header("AccessKey", auth.access_key.clone())
			.header("Host", crate::API_V1_HOSTNAME)
			.header("Connection", "Upgrade")
			.header("Upgrade", "websocket")
			.header("Sec-WebSocket-Key", rand_base_64)
			.header("Sec-WebSocket-Version", "13")
			.body(())
			.map_err(|_| {
				serde_json::Error::custom(
					"Couldn't create the first request to upgrade to WS, suggesting a bad user agent",
				)
			})?;

		let (recv_sender, recv_receiver) =
			tokio::sync::mpsc::unbounded_channel::<WsListenItem>();
		let (send_sender, send_receiver) = tokio::sync::mpsc::channel::<Message>(1);

		let ws_client = Self {
			receive: std::sync::Arc::new(tokio::sync::Mutex::new(
				UnboundedReceiverStream::from(recv_receiver),
			)),
			send: send_sender,
		};

		// Convert the channels to a `Stream`.

		// TODO: listening to websocket requests
		let (client, _) = tokio_tungstenite::connect_async(request).await?;
		tokio::spawn(async move {
			let (mut ws_sender, ws_receiver) = {
				use futures_util::StreamExt;

				client.split()
			};
			let mut bidirectional_stream = ws_receiver
				.map(|rec| {
					WsMultiplexMessage::Receive(rec.map_err(ApiError::Tungstenite))
				})
				.merge(ReceiverStream::from(send_receiver).map(WsMultiplexMessage::Send));

			while let Some(msg) = bidirectional_stream.next().await {
				match msg {
					WsMultiplexMessage::Receive(recv) => {
						if let Ok(message) = recv {
							if message.is_close() {
								recv_sender.closed().await;
							} else {
								recv_sender.send(Self::ws_map_message(message)).ok();
							}
						} else {
							recv_sender.closed().await;
						}
					}
					WsMultiplexMessage::Send(send) => {
						ws_sender.send(send).await.ok();
					}
				}
			}
		});

		Ok(ws_client)
	}

	pub fn is_ok(&self) -> bool {
		!self.send.is_closed()
	}

	/// Sends a WS message to the CVR API.
	///
	/// # Errors
	///
	/// If something with the request failed.
	pub async fn send(
		&self,
		requestable: impl crate::query::Requestable + Serialize + Send,
	) -> Result<(), ApiError> {
		let data = crate::query::RequestWrapper {
			request_type: requestable.request_type(),
			data: requestable,
		};
		let data = serde_json::to_vec(&data)?;
		let data = tokio_tungstenite::tungstenite::Message::binary(data);

		self.send.send(data).await.map_err(|_| {
			ApiError::Tungstenite(tokio_tungstenite::tungstenite::Error::AlreadyClosed)
		})?;

		Ok(())
	}

	pub fn listen(&self) -> ReceiverContainer {
		self.receive.clone()
	}

	/// Turns a WS receiving channel to an async stream
	fn ws_map_message(msg: Message) -> WsListenItem {
		Ok(serde_json::from_slice::<WsResponse>(&msg.into_data())?)
	}
}
