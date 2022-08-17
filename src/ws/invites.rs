use super::WorldDetails;
use crate::UserBaseInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Invite {
	pub id: String,
	pub user: UserBaseInfo,
	pub world: WorldDetails,
	pub instance_id: String,
	pub receiver_id: String,
	pub instance_name: String,
}
