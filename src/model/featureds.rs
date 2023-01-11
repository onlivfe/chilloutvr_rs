use serde::{Deserialize, Serialize};

/// A featured item showcased on an user's profile
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedItem {
	/// The title to display of the item
	pub name: String,
	/// An URL of the item's image
	pub image: String,
}
