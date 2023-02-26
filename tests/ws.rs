#![cfg(feature = "ws_client")]

use chilloutvr::{api_client::ApiError, query::Online};
use tokio_stream::StreamExt;
mod common;

#[tokio::test]
#[ignore]
async fn online() -> Result<(), chilloutvr::api_client::ApiError> {
	let api_client = common::api_client();
	api_client.send(Online).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn open_ws() -> Result<(), chilloutvr::api_client::ApiError> {
	let api_client = common::api_client();

	let listener = match api_client.listen().await {
		Ok(listener) => listener,
		Err(ApiError::Tungstenite(tokio_tungstenite::tungstenite::Error::Http(
			mut resp,
		))) => {
			// Turn the body into human readable from the bytes
			let mut body = None;
			std::mem::swap(resp.body_mut(), &mut body);
			panic!(
				"Creating WS connection gave HTTP status code {} with body: {:?}\nMore info: {:?}",
				resp.status(),
				body.as_ref().map(|bytes| String::from_utf8_lossy(bytes)),
				resp
			);
		}
		Err(err) => panic!("Creating WS connection failed, {:?}", err),
	};
	let mut listener_lock = (*listener).lock().await;
	let next = listener_lock
		.next()
		.await
		.expect("WS listener to have next item")
		.expect("next WS item to not be err");

	dbg!(&next);

	Ok(())
}
