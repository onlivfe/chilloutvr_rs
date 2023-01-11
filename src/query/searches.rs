use racal::Queryable;

use crate::model::{ApiAuth, ResponseDataWrapper, SearchResults};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Search {
	pub term: String,
}

impl Queryable<ApiAuth, ResponseDataWrapper<SearchResults>> for Search {
	fn url(&self) -> String {
		format!("{}/search/{}", crate::API_V1_HTTP_URL, &self.term)
	}
}
