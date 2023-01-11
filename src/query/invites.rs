#[cfg(feature = "ws")]
use super::{RequestType, Requestable};
use serde::Deserialize;
#[cfg(feature = "ws")]
use serde::Serialize;

/// Invite an user to the currently logged in user's instance
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
	/// The ID that the invite is for
	pub id: crate::model::id::User,
}

#[cfg(feature = "ws")]
impl Requestable for Invite {
	fn request_type(&self) -> RequestType {
		RequestType::InviteSend
	}
}

/// Requests an invite from an user
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InviteRequest {
	/// The ID that the request is for
	pub id: crate::model::id::User,
}

#[cfg(feature = "ws")]
impl Requestable for InviteRequest {
	fn request_type(&self) -> RequestType {
		RequestType::RequestInvite
	}
}

/// Accepts an invite request,
/// giving the requester an invite to the current user's instance
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptInviteRequest {
	/// The ID of the invite that this is a response to
	pub id: crate::model::id::Invite,
}

#[cfg(feature = "ws")]
impl Requestable for AcceptInviteRequest {
	fn request_type(&self) -> RequestType {
		RequestType::RequestInviteAccept
	}
}

/// Declines an invite request
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclineInviteRequest {
	/// The ID of the invite that this is a response to
	pub id: crate::model::id::Invite,
}

#[cfg(feature = "ws")]
impl Requestable for DeclineInviteRequest {
	fn request_type(&self) -> RequestType {
		RequestType::RequestInviteDecline
	}
}

/// Marks an invite as expired
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpireInvite {
	/// The ID of the invite to mark as expired
	pub id: crate::model::id::Invite,
}

#[cfg(feature = "ws")]
impl Requestable for ExpireInvite {
	fn request_type(&self) -> RequestType {
		RequestType::InviteExpire
	}
}
