use serde::{Deserialize, Serialize};

use super::UserAuth;

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAuth {
	pub username: String,
	pub access_key: String,
}

impl From<UserAuth> for ApiAuth {
	fn from(user_auth: UserAuth) -> Self {
		ApiAuth { access_key: user_auth.access_key, username: user_auth.username }
	}
}

impl std::fmt::Debug for ApiAuth {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ApiAuth")
			.field("username", &self.username)
			.field("access_key", &"*****")
			.finish()
	}
}
