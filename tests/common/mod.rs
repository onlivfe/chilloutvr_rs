#![cfg(feature = "http_client")]
// Something's funky with checking if these are used or not.
#![allow(dead_code)]
use chilloutvr::{
	api_client::CVR,
	model::{ResponseDataWrapper, UserAuth},
	query::SavedLoginCredentials,
};

const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_REPOSITORY"),
	") - tests",
);

pub fn api_client() -> CVR {
	let user_auth: UserAuth = serde_json::from_slice::<ResponseDataWrapper<UserAuth>>(
		&std::fs::read("user-auth.json")
			.expect("must have a prepared `user-auth.json` file for live API testing"),
	)
	.expect("`user-auth.json` file to parse into user auth")
	.data;

	assert!(!user_auth.username.is_empty());
	assert!(user_auth.access_key.len() > 20);

	CVR::new(USER_AGENT.to_owned(), SavedLoginCredentials::from(user_auth)).unwrap()
}
