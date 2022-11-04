use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::model::{AssetBaseWithTags, UserBase};

use super::InstanceRegion;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldDetails {
	#[serde(flatten)]
	pub base: AssetBaseWithTags,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldInstance {
	#[serde(default)]
	pub id: String,
	#[serde(default)]
	pub name: String,
	pub player_count: u32,
	pub max_player_count: u32,
	pub region: InstanceRegion,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldDetailsResponse {
	#[serde(default)]
	#[serde_as(as = "serde_with::VecSkipError<_>")]
	pub instances: Vec<WorldInstance>,
	#[serde(flatten)]
	pub world: WorldDetails,
}
