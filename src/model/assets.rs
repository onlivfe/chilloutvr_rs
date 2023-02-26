use crate::model::UserBase;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[cfg(feature = "http")]
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	strum::Display,
	strum::EnumString,
	Deserialize,
	Serialize,
)]
#[serde(rename_all = "lowercase", from = "String")]
#[allow(missing_docs)]
/// A tag of an asset
pub enum AssetTag {
	Gore,
	Horror,
	Jumpscare,
	Nudity,
	Suggestive,
	Violence,
	ContainsMusic,
	ExtremelyBright,
	ExtremelyHuge,
	ExtremelySmall,
	FlashingColors,
	FlashingLights,
	LoudAudio,
	ParticleSystems,
	ScreenEffects,
	SpawnAudio,
	LongRangeAudio,
	#[strum(disabled)]
	/// A new unsupported or nonstandard or etc tag
	Other(String),
}

impl From<String> for AssetTag {
	fn from(value: String) -> Self {
		use std::str::FromStr;
		Self::from_str(&value).unwrap_or(Self::Other(value))
	}
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Very minimal details of an asset such as a world or an avatar
pub struct AssetBase {
	/// The ID of the asset
	pub id: crate::id::Asset,
	/// The display name of the asset
	pub name: String,
	/// The preview image's URL
	pub image_url: String,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde_with::serde_as]
#[serde(rename_all = "camelCase")]
/// Details of an asset, including possible tags
pub struct AssetBaseWithTags {
	#[serde(flatten)]
	/// Base details of the asset
	pub base: AssetBase,
	#[serde(default)]
	#[cfg_attr(not(feature = "debug"), serde_as(as = "serde_with::VecSkipError<_>"))]
	/// The tags of the asset
	pub tags: Vec<AssetTag>,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde_with::serde_as]
#[serde(rename_all = "camelCase")]
/// Details of an asset, including possible categories
pub struct AssetBaseWithCategories {
	#[serde(flatten)]
	/// Base details of the asset
	pub base: AssetBase,
	#[serde(default)]
	#[cfg_attr(not(feature = "debug"), serde_as(as = "serde_with::VecSkipError<_>"))]
	/// The categories of the asset
	pub categories: Vec<String>,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details of an asset, including the actual file's details
pub struct AssetFile {
	/// Base details of the asset
	pub asset: AssetBaseWithTags,
	#[serde(rename = "fileId")]
	/// The ID of the actual backing file
	pub file_id: crate::id::File,
	#[serde(rename = "fileSize")]
	// TODO: Better representation via a file size type
	/// The size of the file
	pub size: String,
	#[serde(rename = "fileKey")]
	/// The base64 encoded key used for the crc32 decoding ("decryption") of the
	/// file.
	pub key: String,
	#[serde(rename = "fileHash")]
	/// The MD5 hash of the file, which can/should be used for integrity checks
	pub hash: String,
	#[serde(rename = "fileLocation")]
	/// The location of the file
	pub url: String,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde_with::serde_as]
#[serde(rename_all = "camelCase")]
/// Details about an avatar
pub struct AvatarDetails {
	#[serde(flatten)]
	/// Base details of the asset
	pub base: AssetBaseWithCategories,
	/// The description of the avatar
	pub description: String,
	#[serde(rename = "authorGuid")]
	/// The avatar uploader's ID
	pub author_id: crate::id::User,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Extended details of an avatar
pub struct AvatarAssetDetails {
	#[serde(flatten)]
	/// Base details of the asset
	pub base: AssetBaseWithTags,
	/// The description of the avatar
	pub description: String,
	/// Details of the uploader
	pub user: UserBase,
	/// When the avatar was first uploaded at
	pub uploaded_at: OffsetDateTime,
	/// When the avatar was last uploaded at
	pub updated_at: OffsetDateTime,
	/// If the currently authenticated user is allowed to try to switch to it
	pub switch_permitted: bool,
	/// If the avatar is public
	pub is_published: bool,
	#[serde(default)]
	/// The categories of the avatar
	pub categories: Vec<String>,
	/// How large the avatar is
	pub filesize: u64,
}
