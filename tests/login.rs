#![cfg(feature = "http_client")]

use chilloutvr::{
	api_client::{ApiClient, ApiError, ApiConfiguration},
	query::{LoginCredentials, AuthType},
};

mod common;

#[tokio::test]
#[ignore]
async fn login() -> Result<(), ApiError> {
	let config = ApiConfiguration::new(common::USER_AGENT.to_owned());
	let client = chilloutvr::api_client::UnauthenticatedCVR::new(config).expect("Failed to create unauthenticated client.");

	let query = AuthType::LoginProfile(LoginCredentials {
		email: "email@Address".to_owned(),
		password: "pa$$word".to_owned(),
	});

	let results = client.query(query).await?.data;

	dbg!(&results);

	assert!(!results.access_key.is_empty());

	Ok(())
}