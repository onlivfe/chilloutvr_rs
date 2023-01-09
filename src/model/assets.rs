use crate::model::UserBase;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBase {
	pub id: crate::model::id::Asset,
	pub name: String,
	pub image_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBaseWithTags {
	pub id: crate::model::id::Asset,
	pub name: String,
	pub image_url: String,
	#[serde(default)]
	pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetBaseWithCategories {
	pub id: crate::model::id::Asset,
	pub name: String,
	pub image_url: String,
	#[serde(default)]
	pub categories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetFile {
	pub asset: AssetBaseWithTags,
	#[serde(rename = "fileId")]
	pub id: crate::model::id::Asset,
	#[serde(rename = "fileSize")]
	pub size: String,
	#[serde(rename = "fileKey")]
	pub key: String,
	#[serde(rename = "fileHash")]
	pub hash: String,
	#[serde(rename = "fileLocation")]
	pub location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvatarDetails {
	pub id: crate::model::id::Asset,
	pub name: String,
	pub description: String,
	pub image_url: String,
	#[serde(rename = "authorGuid")]
	pub author_id: crate::model::id::User,
	#[serde(default)]
	pub categories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
