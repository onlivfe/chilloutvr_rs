#[cfg(feature = "http")]
use crate::{
	model::{ResponseDataWrapper, WorldDetailsResponse, WorldInstance},
	query::NoAuthentication,
};
#[cfg(feature = "http")]
use racal::Queryable;
use serde::Serialize;

/// Gets details about a specific world
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct WorldDetailsQuery {
	/// The ID of the world to get more information about
	pub world_id: crate::model::id::Asset,
}

#[cfg(feature = "http")]
impl Queryable<NoAuthentication, ResponseDataWrapper<WorldDetailsResponse>>
	for WorldDetailsQuery
{
	fn url(&self) -> String {
		format!("{}/worlds/{}", crate::API_V1_HTTP_URL, &self.world_id)
	}
}

/// Lists worlds
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct WorldListQuery {
	// TODO: Enum-ify
	/// The category of the worlds to list in
	pub category: String,
}

#[cfg(feature = "http")]
impl Queryable<NoAuthentication, ResponseDataWrapper<Vec<WorldInstance>>>
	for WorldListQuery
{
	fn url(&self) -> String {
		format!("{}/worlds/list/{}", crate::API_V1_HTTP_URL, &self.category)
	}
}
