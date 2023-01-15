use super::ApiError;
use crate::{model::WsResponse, query::SavedLoginCredentials};
use futures_util::{SinkExt, TryStream};
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_stream::{
	wrappers::{ReceiverStream, UnboundedReceiverStream},
	StreamExt,
};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub type WsStreamReturn = Box<
	dyn TryStream<Ok = WsResponse, Error = ApiError, Item = Result<WsResponse, ApiError>>,
>;

pub struct WsClient {
	send: tokio::sync::mpsc::Sender<Message>,
	receive: UnboundedReceiverStream<Message>,
}

enum WsMultiplexMessage {
	Send(Message),
	Receive(Result<Message, ApiError>),
}

impl WsClient {
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
			.header("Host", crate::API_V1_WS_HOST)
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

		let (mut recv_sender, recv_receiver) =
			tokio::sync::mpsc::unbounded_channel::<Message>();
		let (send_sender, send_receiver) = tokio::sync::mpsc::channel::<Message>(1);

		let ws_client = WsClient {
			receive: UnboundedReceiverStream::from(recv_receiver),
			send: send_sender,
		};

		// Convert the channels to a `Stream`.

		// TODO: listening to websocket requests
		let (mut client, _) = tokio_tungstenite::connect_async(request).await?;
		tokio::spawn(async move {
			let mut multiplexing_stream = client
				.map(|rec| {
					WsMultiplexMessage::Receive(rec.map_err(|e| ApiError::Tungstenite(e)))
				})
				.merge(
					ReceiverStream::from(send_receiver)
						.map(|send| WsMultiplexMessage::Send(send)),
				);

			while let Some(msg) = multiplexing_stream.next().await {
				match msg {
					WsMultiplexMessage::Receive(recv) => {
						if let Ok(message) = recv {
							recv_sender.send(message).ok();
						}
					}
					WsMultiplexMessage::Send(send) => {
						client.send(send).await;
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

		self.send.send(data).await;

		Ok(())
	}

	pub async fn listen(&self) -> Result<WsStreamReturn, ApiError> {
		todo!();
	}

	/// Turns a WS receiving channel to an async stream
	fn ws_recv_channel_stream(client: WsStream) -> WsStreamReturn {
		let stream = client.map(|res| match res {
			Ok(res) => Ok(serde_json::from_slice::<WsResponse>(&res.into_data())?),
			Err(err) => Err(ApiError::Tungstenite(err)),
		});
		Box::new(stream)
	}
}
