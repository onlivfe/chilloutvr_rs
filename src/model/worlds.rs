use serde::{Deserialize, Serialize};
#[cfg(feature = "http")]
use time::OffsetDateTime;

#[cfg(feature = "http")]
use crate::model::{AssetBaseWithTags, UserBase};

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldDetails {
	#[serde(flatten)]
	pub base: AssetBaseWithTags,
	#[serde(default)]
	pub description: String,
	pub user: UserBase,
	pub uploaded_at: OffsetDateTime,
	pub updated_at: OffsetDateTime,
	pub switch_permitted: bool,
	pub is_published: bool,
	#[serde(default)]
	pub categories: Vec<String>,
	pub filesize: u64,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldInstance {
	pub id: crate::model::id::Instance,
	#[serde(default)]
	pub name: String,
	pub player_count: u32,
	pub max_player_count: u32,
	pub region: super::InstanceRegion,
}

#[cfg(feature = "http")]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldDetailsResponse {
	#[serde(default)]
	#[cfg_attr(not(feature = "strict"), serde_as(as = "serde_with::VecSkipError<_>"))]
	pub instances: Vec<WorldInstance>,
	#[serde(flatten)]
	pub world: WorldDetails,
}

#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldDisplayDetails {
	pub id: crate::model::id::Asset,
	pub name: String,
	pub image_url: String,
}
