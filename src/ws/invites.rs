//! Models for the currently unsupported WS API

use super::{Listenable, ResponseType, WorldDetails};
use crate::model::{InviteRequest, UserBase};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Invite {
	pub id: String,
	pub user: UserBase,
	pub world: WorldDetails,
	pub instance_id: String,
	pub receiver_id: String,
	pub instance_name: String,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkipInviteErrors(#[serde_as(as = "serde_with::VecSkipError<_>")] Vec<Invite>);

impl Listenable for Vec<Invite> {
	const RESPONSE_TYPE: ResponseType = ResponseType::OnlineFriends;
}

impl Listenable for SkipInviteErrors {
	const RESPONSE_TYPE: ResponseType = ResponseType::OnlineFriends;
}

impl Listenable for Vec<InviteRequest> {
	const RESPONSE_TYPE: ResponseType = ResponseType::OnlineFriends;
}
