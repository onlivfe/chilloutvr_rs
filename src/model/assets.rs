use crate::model::UserBase;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssetBase {
	pub id: String,
	pub name: String,
	pub image_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssetBaseWithTags {
	pub id: String,
	pub name: String,
	pub image_url: String,
	#[serde(default)]
	pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssetBaseWithCategories {
	pub id: String,
	pub name: String,
	pub image_url: String,
	#[serde(default)]
	pub categories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssetFile {
	pub asset: AssetBaseWithTags,
	#[serde(rename = "FileId")]
	pub id: String,
	#[serde(rename = "FileSize")]
	pub size: String,
	#[serde(rename = "FileKey")]
	pub key: String,
	#[serde(rename = "FileHash")]
	pub hash: String,
	#[serde(rename = "FileLocation")]
	pub location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvatarDetails {
	pub id: String,
	pub name: String,
	pub description: String,
	pub image_url: String,
	pub author_guid: String,
	#[serde(default)]
	pub categories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvatarAssetDetails {
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
