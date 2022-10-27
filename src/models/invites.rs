use crate::UserBase;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InviteRequest {
	pub id: String,
	pub sender: UserBase,
	pub receiver_id: String,
}
