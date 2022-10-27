use crate::{AssetBase, FeaturedItem, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserBase {
	pub id: String,
	pub name: String,
	pub image_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserDetails {
	#[serde(flatten)]
	pub base: UserBase,
	pub rank: String,
	pub featured_badge: FeaturedItem,
	pub featured_group: FeaturedItem,
	pub avatar: AssetBase,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserDetailsQuery {
	pub user_id: String,
}

impl Queryable for UserDetailsQuery {
	type ResponseType = UserDetails;
	fn url(&self) -> String {
		format!("{}/users/{}", crate::API_V1_HTTP_URL, &self.user_id)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserAuth {
	pub username: String,
	pub access_key: String,
	pub user_id: String,
	pub current_avatar: String,
	pub current_home_world: String,
	pub video_url_resolver_executable: String,
	pub video_url_resolver_hashes: String,
	pub blocked_users: Vec<String>,
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
