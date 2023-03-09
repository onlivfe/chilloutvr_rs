#![cfg(feature = "ws_client")]

use chilloutvr::{api_client::ApiError, query::Online};
use tokio_stream::StreamExt;
mod common;

#[tokio::test]
#[ignore]
async fn online() -> Result<(), ApiError> {
	let api_client = common::api_client();
	api_client.send(Online).await?;

	Ok(())
}

#[tokio::test]
#[ignore]
async fn open_ws() -> Result<(), ApiError> {
	let api_client = common::api_client();

	let listener = api_client.listen().await.unwrap();
	let mut listener_lock = (*listener).lock().await;
	let next = listener_lock
		.next()
		.await
		.expect("WS listener to have next item")
		.expect("next WS item to not be err");

	dbg!(&next);

	Ok(())
}
