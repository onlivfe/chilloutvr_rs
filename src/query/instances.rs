use serde::{Deserialize, Serialize};

use crate::{model::ExtendedInstanceDetails, Queryable};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
	pub instance_id: crate::model::id::Instance,
}

impl Queryable for Instance {
	type ResponseType = ExtendedInstanceDetails;
	fn url(&self) -> String {
		format!("{}/instances/{}", crate::API_V1_HTTP_URL, &self.instance_id)
	}
}
