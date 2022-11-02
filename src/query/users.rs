use serde::Serialize;

use crate::{model::UserAuth, Queryable};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserDetails {
	pub user_id: String,
}

impl Queryable for UserDetails {
	type ResponseType = crate::model::UserDetails;
	fn url(&self) -> String {
		format!("{}/users/{}", crate::API_V1_HTTP_URL, &self.user_id)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserAuthRequest {
	pub username: String,
	pub password: String,
	pub auth_type: String,
}

impl Queryable for UserAuthRequest {
	type ResponseType = UserAuth;
	fn url(&self) -> String {
		format!("{}/users/auth", crate::API_V1_HTTP_URL)
	}

	fn body(&self) -> Option<serde_json::Result<Vec<u8>>> {
		Some(serde_json::to_vec(self))
	}
}
