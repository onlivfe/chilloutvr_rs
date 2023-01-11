use racal::Queryable;
use serde::{Deserialize, Serialize};

use crate::model::{ExtendedInstanceDetails, ResponseDataWrapper};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
	pub instance_id: crate::model::id::Instance,
}

impl Queryable<(), ResponseDataWrapper<ExtendedInstanceDetails>> for Instance {
	fn url(&self) -> String {
		format!("{}/instances/{}", crate::API_V1_HTTP_URL, &self.instance_id)
	}
}
