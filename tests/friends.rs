#![cfg(feature = "api_client")]

mod common;

#[test]
#[ignore]
fn friend_requests() -> Result<(), chilloutvr::api_client::ApiError> {
	let query = chilloutvr::query::FriendRequests {};
	let results = tokio_test::block_on(common::api_client().query(query))?;
	// To run this test, you should have at least 1 pending friend request :/

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

#[test]
#[ignore]
fn friends() -> Result<(), chilloutvr::api_client::ApiError> {
	let query = chilloutvr::query::FriendList {};
	let results = tokio_test::block_on(common::api_client().query(query))?;
	// To run this test, you should have at least 1 friend :/

	dbg!(&results);

	assert!(!results.0.is_empty());

	let first_friend = match results.0.first() {
		Some(result) => result,
		None => panic!("expected response to contain at least a single world"),
	};

	assert!(!first_friend.id.as_ref().is_empty());
	assert!(!first_friend.name.is_empty());
	assert!(!first_friend.image_url.is_empty());

	Ok(())
}
