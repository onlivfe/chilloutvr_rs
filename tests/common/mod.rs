#![cfg(any(feature = "http_client", feature = "ws_client"))]
// Something's funky with checking if these are used or not.
#![allow(dead_code)]
use chilloutvr::{
	api_client::{ApiConfiguration, AuthenticatedCVR},
	model::{ResponseDataWrapper, UserAuth},
	query::SavedLoginCredentials,
};
use once_cell::sync::Lazy;

pub const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	") - tests",
);

pub static USER_AUTH: Lazy<UserAuth> = Lazy::new(|| {
	let user_auth: UserAuth =
		serde_json::from_slice::<ResponseDataWrapper<UserAuth>>(
			&std::fs::read("user-auth.json").expect(
				"must have a prepared `user-auth.json` file for live API testing",
			),
		)
		.expect("`user-auth.json` file to parse into user auth")
		.data;

	assert!(!user_auth.username.is_empty());
	assert!(user_auth.access_key.len() > 20);

	user_auth
});

pub fn api_client() -> AuthenticatedCVR {
	AuthenticatedCVR::new(
		ApiConfiguration::new(USER_AGENT.to_owned()),
		SavedLoginCredentials::from(&USER_AUTH.clone().into()),
	)
	.unwrap()
}
