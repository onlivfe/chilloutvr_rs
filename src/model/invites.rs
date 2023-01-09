use crate::model::UserBase;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteRequest {
	pub id: crate::model::id::Invite,
	pub sender: UserBase,
	pub receiver_id: crate::model::id::User,
}
