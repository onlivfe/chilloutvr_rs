use serde::Deserialize;

#[cfg(feature = "ws")]
use crate::model::{UserBase, WorldDisplayDetails};

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
pub struct Invites(
	#[cfg_attr(not(feature = "strict"), serde_as(as = "serde_with::VecSkipError<_>"))]
	Vec<Invite>,
);

#[cfg(feature = "ws")]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteRequests(
	#[cfg_attr(not(feature = "strict"), serde_as(as = "serde_with::VecSkipError<_>"))]
	Vec<InviteRequest>,
);
