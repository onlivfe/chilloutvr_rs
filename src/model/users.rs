use serde::{Deserialize, Serialize};

#[cfg(feature = "http")]
use super::AssetBaseWithCategories;
#[cfg(feature = "http")]
use crate::model::{AssetBase, FeaturedItem};

#[cfg(any(feature = "http", feature = "ws"))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Basic details about an user
pub struct UserBase {
	/// The ID of the user
	pub id: crate::id::User,
	/// The display name of the user
	pub name: String,
	/// The user's icon
	pub image_url: String,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Standard details about an user
pub struct UserDetails {
	#[serde(flatten)]
	/// The basic details of the user
	pub base: UserBase,
	/// The users rank
	pub rank: String,
	/// The users featured badge
	pub featured_badge: FeaturedItem,
	/// The users featured group
	pub featured_group: FeaturedItem,
	/// The users current avatar
	pub avatar: AssetBase,
}

#[cfg(feature = "http")]
#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Authentication success response
pub struct UserAuth {
	/// The username that the authentication is for
	pub username: String,
	/// The access key to authenticate future requests with
	pub access_key: String,
	/// The ID of the user that the authentication is for
	pub user_id: crate::id::User,
	/// The ID of the avatar that the user had selected at the time of the
	/// response
	pub current_avatar: crate::id::Asset,
	/// The ID of the home world that the user had selected at the time of the
	/// response
	pub current_home_world: crate::id::Asset,
	/// An URL to a video downloader executable
	pub video_url_resolver_executable: String,
	/// An URL to a hash file for the video downloader executable
	pub video_url_resolver_hashes: String,
	#[serde(default)]
	/// Users that were blocked by the user at the time of the response
	pub blocked_users: Vec<String>,
}

#[cfg(feature = "http")]
impl std::fmt::Debug for UserAuth {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("UserAuth")
			.field("username", &self.username)
			.field("access_key", &"*****")
			.field("user_id", &self.user_id)
			.field("current_avatar", &self.current_avatar)
			.field("current_home_world", &self.current_home_world)
			.field(
				"video_url_resolver_executable",
				&self.video_url_resolver_executable,
			)
			.field("video_url_resolver_hashes", &self.video_url_resolver_hashes)
			.field("blocked_users", &self.blocked_users)
			.finish()
	}
}

#[cfg(feature = "http")]
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
/// A list of friends
pub struct Friends(
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
	pub Vec<AssetBaseWithCategories>,
);

#[cfg(feature = "http")]
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
/// A list of friend requests
pub struct FriendRequests(
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
	pub Vec<AssetBase>,
);

#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A change event of an user's status
pub struct UserOnlineStatusChange {
	/// The ID of the user
	pub id: crate::id::User,
	/// If the user is now online or offline
	pub is_online: bool,
}

#[cfg(feature = "ws")]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A list of user online status change events
pub struct OnlineUserStatusChanges(
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
	pub Vec<UserOnlineStatusChange>,
);
