use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::model::UserBase;

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
/// A tag of an asset, see [the CCK's Content Tags page](https://developers.abinteractive.net/cck/content-tags/)
///
/// The docs are copied here for convenience, but they might get out of date,
/// so check the source if you're unsure.
pub enum AssetTag {
	/// If your content contains gore of any kind.
	///
	/// # Examples
	///
	/// - Blood from a wound or cut
	/// - Excessive violence
	/// - Excessive detail about e.g. an injury
	///
	/// # Tag Locked
	///
	/// This tag is locked behind the [Mature Content Access DLC](https://developers.abinteractive.net/chilloutvr/faq/mature-content-access-dlc/) which is free on steam.
	Gore,
	/// Use this tag if your avatar contains violence.
	///
	/// # Examples
	///
	/// - Injuries
	/// - Damaging or destroying an object/character
	/// - Usage of firearms
	Violence,
	/// Use this tag if your content contains Horror elements.
	///
	/// # Examples
	///
	/// - Scary visual effect
	/// - Scary sound effects
	/// - Jump scares
	Horror,
	/// A jump scare is a technique often used in horror films and video games,
	/// intended to scare the audience by surprising them with an abrupt change
	/// in image or event, usually co-occurring with a loud, frightening sound.
	Jumpscare,
	/// Use this tag if your avatar or spawnable/prop is very small.
	ExtremelySmall,
	/// Just like [Excessively Small](Self::ExtremelySmall),
	/// but the other way around.
	/// If your avatar or spawnable/prop is very huge.
	ExtremelyHuge,
	/// The Suggestive Tag must be used whenever there is large amounts of
	/// visible skin or a sexually suggestive pose or animation.
	///
	/// # Examples
	///
	/// - Bikini or Swimsuit outfits
	Suggestive,
	/// The Nudity tag is to be applied to all content not to be seen by
	/// children. It is mainly meant for sexual appealing content.
	///
	/// # Examples
	///
	/// - Genitals
	/// - Nipples
	/// - Ass without pants
	/// - Sex toys
	///
	/// # Tag Locked
	///
	/// This tag is locked behind the [Mature Content Access DLC](https://developers.abinteractive.net/chilloutvr/faq/mature-content-access-dlc/) which is free on steam.
	Nudity,
	/// Extremely bright is pretty much self-explanatory,
	/// use it, when you have a lot of bright materials on your content.
	///
	/// # Examples
	///
	/// - High emissive values on materials
	/// - High amount of bloom
	ExtremelyBright,
	/// Use this tag, if your content contains rapidly changing/flashing colors.
	FlashingColors,
	/// This tag is similar to [Flashing Colors](Self::FlashingColors), but is
	/// not limited to color.
	///
	/// # Examples
	///
	/// - Lights
	/// - Materials
	/// - Textures
	/// - Colors
	///
	/// # Tip
	///
	/// Flashing Colors and Flashing Lights is commonly used together.
	/// The most important reason for this tag's existence is due to health
	/// conditions players might have. So make sure to use those tags whenever
	/// something is rapidly changing or flashing.
	FlashingLights,
	/// Use this tag, if your content contains particle systems.
	ParticleSystems,
	/// If your content contains any kind of screen effects use this tag.
	///
	/// # Examples
	///
	/// - Screen space shader effects
	/// - Screen space animation
	/// - Flashy animations
	ScreenEffects,
	/// Use this tag if your created content, specifically avatars and
	/// spawnables/props, contains music.
	/// This tag was not made because of copyright or any other legal reasons,
	/// this tag, including all others as well,
	/// does not protect your content from getting copyright claimed and/or
	/// taken down because of legal reasons.
	///
	/// # Examples
	///
	/// - User playing music on his avatar
	/// - User playing music using a prop
	ContainsMusic,
	/// The Loud Audio Tag is used for content which contains loud audio.
	/// Loud audio can mean, but is not limited to,
	/// vastly louder sound effects or music playing.
	LoudAudio,
	/// This tag is specifically made for audio sources which immediately play
	/// after the content is loaded, aka. spawn audios.
	SpawnAudio,
	/// This tag is used for audio sources which have a long distance fall off.
	/// This means that you can hear them in game, even while being far or very
	/// far away from its origin.
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
pub struct AssetBase<Id = crate::id::Asset> {
	/// The ID of the asset
	pub id: Id,
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
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
	/// The tags of the asset
	pub tags: Vec<AssetTag>,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde_with::serde_as]
#[serde(rename_all = "camelCase")]
/// Details of an asset, including possible categories
pub struct AssetBaseWithCategories<Id = crate::id::Asset> {
	#[serde(flatten)]
	/// Base details of the asset
	pub base: AssetBase<Id>,
	#[serde(default)]
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
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
