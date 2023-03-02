#![cfg(feature = "http_client")]

use chilloutvr::{
	api_client::ApiClient,
	model::{FriendRequests, Friends},
};

mod common;

#[tokio::test]
#[ignore]
async fn friend_requests() -> Result<(), chilloutvr::api_client::ApiError> {
	let api_client = common::api_client();

	let query = chilloutvr::query::FriendRequests {};
	let results: FriendRequests = api_client.query(query).await?.data;
	// To run this test, you should have at least 1 friend request, oh no for you
	// ~

	dbg!(&results);

	assert!(!results.0.is_empty());

	let first_request = match results.0.first() {
		Some(result) => result,
		None => panic!("expected response to contain at least a single world"),
	};

	assert!(!first_request.id.as_ref().is_empty());
	assert!(!first_request.name.is_empty());
	assert!(!first_request.image_url.is_empty());

	Ok(())
}

#[tokio::test]
#[ignore]
async fn friends() -> Result<(), chilloutvr::api_client::ApiError> {
	let api_client = common::api_client();

	let query = chilloutvr::query::FriendList {};
	let results: Friends = api_client.query(query).await?.data;
	// To run this test, you should have at least 1 friend, oh no for you ~

	dbg!(&results);

	assert!(!results.0.is_empty());

	let first_friend = match results.0.first() {
		Some(result) => result,
		None => panic!("expected response to contain at least a single world"),
	};

	assert!(!first_friend.base.id.as_ref().is_empty());
	assert!(!first_friend.base.name.is_empty());
	assert!(!first_friend.base.image_url.is_empty());

	Ok(())
}
