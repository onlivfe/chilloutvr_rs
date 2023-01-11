use racal::Queryable;
use serde::Serialize;

use crate::model::{ResponseDataWrapper, WorldDetailsResponse, WorldInstance};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct WorldDetailsQuery {
	pub world_id: crate::model::id::Asset,
}

impl Queryable<(), ResponseDataWrapper<WorldDetailsResponse>> for WorldDetailsQuery {
	fn url(&self) -> String {
		format!("{}/worlds/{}", crate::API_V1_HTTP_URL, &self.world_id)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct WorldListQuery {
	pub category: String,
}

impl Queryable<(), ResponseDataWrapper<Vec<WorldInstance>>> for WorldListQuery {
	fn url(&self) -> String {
		format!("{}/worlds/list/{}", crate::API_V1_HTTP_URL, &self.category)
	}
}
