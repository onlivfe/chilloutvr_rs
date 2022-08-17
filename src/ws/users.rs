use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserOnlineStatusChange {
	pub id: String,
	pub is_online: bool,
}
