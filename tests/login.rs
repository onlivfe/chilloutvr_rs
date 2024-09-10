#![cfg(feature = "http_client")]

use chilloutvr::{
	api_client::{ApiClient, ApiError},
	query::AuthType,
};

mod common;

#[tokio::test]
#[ignore]
async fn login() -> Result<(), ApiError> {
	let client = common::unauthenticated_api_client();

	/*
	example credentials file:

	{
			"auth_type": "loginProfile",
			"username": "example@example.com",
			"password": "hunter2"
	}

	*/

	let credentials = serde_json::from_slice::<AuthType>(
		&std::fs::read("user-credentials.json").expect(
			"must have a prepared `user-credentials.json` file for live API testing",
		),
	)
	.expect("`user-credentials.json` file to parse into auth type");

	let results = client.query(credentials).await?.data;

	dbg!(&results);

	assert!(!results.access_key.is_empty());

	Ok(())
}
