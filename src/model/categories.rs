#[cfg(feature = "http")]
use serde::{Deserialize, Serialize};

/// A category
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
	/// The ID of the category
	pub id: crate::id::Category,
	/// The name of the category
	pub name: String,
}

/// Different categories
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Categories {
	/// Avatar categories
	pub avatars: Vec<Category>,
	/// World categories
	pub worlds: Vec<Category>,
	/// Friend categories
	pub friends: Vec<Category>,
	/// Prop categories
	pub spawnables: Vec<Category>,
}
