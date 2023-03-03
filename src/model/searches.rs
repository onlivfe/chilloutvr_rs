use serde::{Deserialize, Serialize};

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "type", content = "id")]
#[serde(rename_all = "camelCase")]
/// The ID of a searches' result, which also tells the type of the search result
pub enum SearchResultId {
	/// An user
	User(crate::id::User),
	/// An avatar
	Avatar(crate::id::Asset),
	/// A prop
	Prop(crate::id::Asset),
	/// A world
	World(crate::id::Asset),
}

#[cfg(feature = "http")]
impl From<SearchResultId> for crate::id::Any {
	fn from(value: SearchResultId) -> Self {
		match value {
			SearchResultId::User(v) => v.into(),
			SearchResultId::Avatar(v)
			| SearchResultId::Prop(v)
			| SearchResultId::World(v) => v.into(),
		}
	}
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A search result
pub struct SearchResult {
	#[serde(flatten)]
	/// The ID of the search result, which also tells the type of the result
	pub id: SearchResultId,
	/// The display name of the search result
	pub name: String,
	/// An URL to the preview image
	pub image_url: String,
}

#[cfg(feature = "http")]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// Results for a search
pub struct SearchResults(
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
	pub Vec<SearchResult>,
);
