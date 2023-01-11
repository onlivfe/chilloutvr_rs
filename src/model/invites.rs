use serde::Deserialize;

#[cfg(feature = "ws")]
use crate::model::{Listenable, ResponseType, UserBase, WorldDisplayDetails};

/// A request from the sender to be invited by the receiver
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteRequest {
	/// The ID of the invite
	pub id: crate::model::id::Invite,
	/// The sender of the invite
	pub sender: crate::model::UserBase,
	/// The receiver's ID
	pub receiver_id: crate::model::id::User,
}

/// An invite from someone to join their instance
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
	pub id: crate::model::id::Invite,
	pub user: UserBase,
	pub world: WorldDisplayDetails,
	pub instance_id: crate::model::id::Instance,
	pub receiver_id: crate::model::id::User,
	pub instance_name: String,
}

#[cfg(feature = "ws")]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invites(#[serde_as(as = "serde_with::VecSkipError<_>")] Vec<Invite>);

#[cfg(feature = "ws")]
impl Listenable for Vec<Invite> {
	const RESPONSE_TYPE: ResponseType = ResponseType::OnlineFriends;
}

#[cfg(feature = "ws")]
impl Listenable for Invites {
	const RESPONSE_TYPE: ResponseType = ResponseType::OnlineFriends;
}

#[cfg(feature = "ws")]
impl Listenable for Vec<InviteRequest> {
	const RESPONSE_TYPE: ResponseType = ResponseType::OnlineFriends;
}
