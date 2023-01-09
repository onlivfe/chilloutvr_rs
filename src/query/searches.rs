use crate::{model::SearchResults, Queryable};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Search {
	pub term: String,
}

impl Queryable for Search {
	type ResponseType = SearchResults;
	fn url(&self) -> String {
		format!("{}/search/{}", crate::API_V1_HTTP_URL, &self.term)
	}
}
