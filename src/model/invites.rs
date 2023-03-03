use serde::{Deserialize, Serialize};

#[cfg(feature = "ws")]
use crate::model::{UserBase, WorldDisplayDetails};

/// A request from the sender to be invited by the receiver
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteRequest {
	/// The ID of the invite
	pub id: crate::id::Invite,
	/// The sender of the invite request
	pub sender: crate::model::UserBase,
	/// The receiver's ID
	pub receiver_id: crate::id::User,
}

/// An invite from someone to join their instance
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
	/// The ID of the invite
	pub id: crate::id::Invite,
	/// The user that this invite is from
	pub user: UserBase,
	/// The world that this invite is to
	pub world: WorldDisplayDetails,
	/// The ID of the instance that this invite is to
	pub instance_id: crate::id::Instance,
	/// The receiving user's ID
	pub receiver_id: crate::id::User,
	/// The instance's name that this invite is to
	pub instance_name: String,
}

#[cfg(feature = "ws")]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A list of invites
pub struct Invites(
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
	pub Vec<Invite>,
);

#[cfg(feature = "ws")]
#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// A list of invite requests
pub struct InviteRequests(
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
	pub Vec<InviteRequest>,
);
