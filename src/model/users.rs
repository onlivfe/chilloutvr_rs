use crate::model::{AssetBase, FeaturedItem};
use serde::{Deserialize, Serialize};

use super::AssetBaseWithCategories;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBase {
	pub id: crate::model::id::User,
	pub name: String,
	pub image_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetails {
	#[serde(flatten)]
	pub base: UserBase,
	pub rank: String,
	pub featured_badge: FeaturedItem,
	pub featured_group: FeaturedItem,
	pub avatar: AssetBase,
}

#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAuth {
	pub username: String,
	pub access_key: String,
	pub user_id: crate::model::id::User,
	pub current_avatar: crate::model::id::Asset,
	pub current_home_world: crate::model::id::Asset,
	pub video_url_resolver_executable: String,
	pub video_url_resolver_hashes: String,
	#[serde(default)]
	pub blocked_users: Vec<String>,
}

impl std::fmt::Debug for UserAuth {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("UserAuth")
			.field("username", &self.username)
			.field("access_key", &"*****")
			.field("user_id", &self.user_id)
			.field("current_avatar", &self.current_avatar)
			.field("current_home_world", &self.current_home_world)
			.field("video_url_resolver_executable", &self.video_url_resolver_executable)
			.field("video_url_resolver_hashes", &self.video_url_resolver_hashes)
			.field("blocked_users", &self.blocked_users)
			.finish()
	}
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct Friends(
	#[cfg_attr(not(feature = "strict"), serde_as(as = "serde_with::VecSkipError<_>"))]
	pub Vec<AssetBaseWithCategories>,
);

#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct FriendRequests(
	#[cfg_attr(not(feature = "strict"), serde_as(as = "serde_with::VecSkipError<_>"))]
	pub Vec<AssetBase>,
);
