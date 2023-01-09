use serde::Serialize;

use crate::{
	model::{WorldDetailsResponse, WorldInstance},
	Queryable,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct WorldDetailsQuery {
	pub world_id: crate::model::id::Asset,
}

impl Queryable for WorldDetailsQuery {
	type ResponseType = WorldDetailsResponse;
	fn url(&self) -> String {
		format!("{}/worlds/{}", crate::API_V1_HTTP_URL, &self.world_id)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct WorldListQuery {
	pub category: String,
}

impl Queryable for WorldListQuery {
	type ResponseType = Vec<WorldInstance>;
	fn url(&self) -> String {
		format!("{}/worlds/list/{}", crate::API_V1_HTTP_URL, &self.category)
	}
}
