use racal::Queryable;
use serde::Serialize;

use crate::model::{ResponseDataWrapper, UserAuth};

use super::NoAuthentication;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserDetails {
	pub user_id: crate::model::id::User,
}

impl Queryable<NoAuthentication, ResponseDataWrapper<crate::model::UserDetails>>
	for UserDetails
{
	fn url(&self) -> String {
		format!("{}/users/{}", crate::API_V1_HTTP_URL, &self.user_id)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AuthType {
	LoginProfile = 1,
	LoginCredentials,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAuthRequest {
	/// Actually an email
	pub username: String,
	pub password: String,
	pub auth_type: AuthType,
}

impl std::fmt::Debug for UserAuthRequest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("UserAuthRequest")
			.field("username", &self.username)
			.field("password", &"*****")
			.field("auth_type", &self.auth_type)
			.finish()
	}
}

impl Queryable<NoAuthentication, ResponseDataWrapper<UserAuth>> for UserAuthRequest {
	fn url(&self) -> String {
		format!("{}/users/auth", crate::API_V1_HTTP_URL)
	}

	fn body(&self) -> Option<serde_json::Result<Vec<u8>>> {
		Some(serde_json::to_vec(self))
	}
}
