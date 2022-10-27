use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::Queryable;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct SearchResult {
	#[serde(rename = "ResultId")]
	pub id: String,
	#[serde(rename = "ResultName")]
	pub name: String,
	#[serde(rename = "ResultImageUrl")]
	pub image_url: String,
	#[serde(rename = "ResultDate")]
	pub date: OffsetDateTime,
	#[serde(rename = "ResultType")]
	pub r#type: String,
	#[serde(rename = "ResultIsMine")]
	pub is_mine: bool,
	#[serde(rename = "ResultIsShared")]
	pub is_shared: bool,
	#[serde(rename = "ResultIsPublic")]
	pub is_public: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SearchQuery {
	pub term: String,
}

impl Queryable for SearchQuery {
	type ResponseType = SearchResult;
	fn url(&self) -> String {
		format!("{}/search/{}", crate::API_V1_HTTP_URL, &self.term)
	}
}
