#[cfg(feature = "http")]
use racal::Queryable;
use serde::{Deserialize, Serialize};

#[cfg(feature = "http")]
use crate::{
	model::{ResponseDataWrapper, SearchResults},
	query::SavedLoginCredentials,
};

/// Search for things using a search term
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Search {
	/// The search term to use
	pub term: String,
}

#[cfg(feature = "http")]
impl Queryable<SavedLoginCredentials, ResponseDataWrapper<SearchResults>>
	for Search
{
	fn url(&self, _: &SavedLoginCredentials) -> String {
		format!("{}/search/{}", crate::API_V1_HTTP_URL, &self.term)
	}
}
