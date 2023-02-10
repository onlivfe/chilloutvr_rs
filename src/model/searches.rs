use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[cfg(feature = "http")]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(tag = "type", content = "id")]
#[serde(rename_all = "camelCase")]
pub enum SearchResultId {
	/// An user
	User(crate::model::id::User),
	/// An avatar
	Avatar(crate::model::id::Asset),
	/// A prop
	Prop(crate::model::id::Asset),
	/// A world
	World(crate::model::id::Asset),
}

#[cfg(feature = "http")]
impl From<SearchResultId> for crate::model::id::Any {
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
pub struct SearchResult {
	#[serde(flatten)]
	pub id: SearchResultId,
	pub name: String,
	pub image_url: String,
}

#[cfg(feature = "http")]
#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResults(
	#[cfg_attr(not(feature = "debug"), serde_as(as = "serde_with::VecSkipError<_>"))]
	pub Vec<SearchResult>,
);
