#[cfg(feature = "http")]
use crate::{
	model::{ResponseDataWrapper, SearchResults},
	query::SavedLoginCredentials,
};

#[cfg(feature = "http")]
use racal::Queryable;

/// Search for things using a search term
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Search {
	/// The search term to use
	pub term: String,
}

#[cfg(feature = "http")]
impl Queryable<SavedLoginCredentials, ResponseDataWrapper<SearchResults>> for Search {
	fn url(&self) -> String {
		format!("{}/search/{}", crate::API_V1_HTTP_URL, &self.term)
	}
}
