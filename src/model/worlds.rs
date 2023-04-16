use serde::{Deserialize, Serialize};
#[cfg(feature = "http")]
use time::OffsetDateTime;

#[cfg(feature = "http")]
use crate::model::{AssetBaseWithTags, UserBase};

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details of the world that include player count
pub struct WorldListItem {
	#[serde(flatten)]
	/// The base details of the world asset
	pub base: crate::model::AssetBase,
	/// How many players are in instances of the world
	pub player_count: u32,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about a world
pub struct WorldDetails {
	#[serde(flatten)]
	/// Basic details of the asset
	pub base: AssetBaseWithTags,
	#[serde(default)]
	/// A description of the world
	pub description: String,
	/// The uploader user's details
	pub user: UserBase,
	/// When the world was first uploaded at
	pub uploaded_at: OffsetDateTime,
	/// When the world was last uploaded at
	pub updated_at: OffsetDateTime,
	/// If the currently authenticated user can join the world
	pub switch_permitted: bool,
	/// If the world is public
	pub is_published: bool,
	#[serde(default)]
	/// Possible categories of the world
	pub categories: Vec<String>,
	/// The world asset file's size
	pub filesize: u64,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// An instance of the world
pub struct WorldInstance {
	/// The ID of the instance
	pub id: crate::id::Instance,
	#[serde(default)]
	/// The name of the instance
	pub name: String,
	/// How many players are in the instance
	pub player_count: u32,
	/// How many players the instance can support
	pub max_player_count: u32,
	/// What region the instance is in
	pub region: super::InstanceRegion,
}

#[cfg(feature = "http")]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details of a world's current status
pub struct WorldDetailsResponse {
	#[serde(default)]
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
	/// Details of instances that are using the world
	pub instances: Vec<WorldInstance>,
	#[serde(flatten)]
	/// Details of the actual world
	pub world: WorldDetails,
}

#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Very minimal details of a world
pub struct WorldDisplayDetails {
	/// The ID of the world
	pub id: crate::id::Asset,
	/// The name of the world
	pub name: String,
	/// The preview image of the world
	pub image_url: String,
}
