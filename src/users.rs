use serde::{Deserialize, Serialize};

use crate::{Asset, FeaturedItem};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserBaseInfo {
	pub id: String,
	pub name: String,
	pub image_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserDetails {
	#[serde(flatten)]
	pub base: UserBaseInfo,
	pub rank: String,
	pub featured_badge: FeaturedItem,
	pub featured_group: FeaturedItem,
	pub avatar: Asset,
}
