#[cfg(feature = "http")]
use racal::Queryable;
use serde::{Deserialize, Serialize};

#[cfg(feature = "http")]
use crate::{
	model::{ResponseDataWrapper, WorldDetailsResponse, WorldListItem},
	query::NoAuthentication,
};

/// Gets details about a specific world
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct WorldDetailsQuery {
	/// The ID of the world to get more information about
	pub world_id: crate::id::Asset,
}

#[cfg(feature = "http")]
impl Queryable<NoAuthentication, ResponseDataWrapper<WorldDetailsResponse>>
	for WorldDetailsQuery
{
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/worlds/{}", crate::API_V1_HTTP_URL, &self.world_id)
	}
}

/// Lists worlds
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct WorldListQuery {
	/// The category of the worlds to list
	pub category: crate::id::Category,
}

#[cfg(feature = "http")]
impl Queryable<NoAuthentication, ResponseDataWrapper<Vec<WorldListItem>>>
	for WorldListQuery
{
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/worlds/list/{}", crate::API_V1_HTTP_URL, &self.category)
	}
}
