#[cfg(feature = "http")]
use racal::Queryable;
use serde::Deserialize;

#[cfg(feature = "http")]
use crate::{
	model::{ExtendedInstanceDetails, ResponseDataWrapper},
	query::NoAuthentication,
};

/// Gets details about a specific instance
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
	/// The ID of the instance to get more information about
	pub instance_id: crate::id::Instance,
}

#[cfg(feature = "http")]
impl Queryable<NoAuthentication, ResponseDataWrapper<ExtendedInstanceDetails>>
	for Instance
{
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/instances/{}", crate::API_V1_HTTP_URL, &self.instance_id)
	}
}
