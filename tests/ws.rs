#![cfg(feature = "ws_client")]

use chilloutvr::query::Online;
mod common;

#[test]
#[ignore]
fn online() -> Result<(), chilloutvr::api_client::ApiError> {
	let api_client = common::api_client();
	tokio_test::block_on(api_client.send(Online))?;

	Ok(())
}

#[test]
#[ignore]
fn open_ws() -> Result<(), chilloutvr::api_client::ApiError> {
	use tokio_stream::StreamExt;

	let api_client = common::api_client();
	let listener = tokio_test::block_on(api_client.listen()).unwrap();
	let mut listener_lock = tokio_test::block_on((*listener).lock());
	let next = tokio_test::block_on(listener_lock.next()).unwrap().unwrap();

	dbg!(&next);

	Ok(())
}
