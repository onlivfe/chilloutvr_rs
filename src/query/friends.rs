#[cfg(feature = "ws")]
use super::{RequestType, Requestable};
#[cfg(feature = "http")]
use crate::{
	model::{Friends, ResponseDataWrapper},
	query::SavedLoginCredentials,
};
#[cfg(feature = "http")]
use racal::Queryable;

/// Gets your friends list
#[cfg(feature = "http")]
#[derive(Default, Debug, Clone)]
pub struct FriendList();

#[cfg(feature = "http")]
impl Queryable<SavedLoginCredentials, ResponseDataWrapper<Friends>> for FriendList {
	fn url(&self) -> String {
		format!("{}/friends", crate::API_V1_HTTP_URL)
	}
}

/// Gets friend requests
#[cfg(feature = "http")]
#[derive(Default, Debug, Clone)]
pub struct FriendRequests();

#[cfg(feature = "http")]
impl Queryable<SavedLoginCredentials, ResponseDataWrapper<crate::model::FriendRequests>>
	for FriendRequests
{
	fn url(&self) -> String {
		format!("{}/friends/requests", crate::API_V1_HTTP_URL)
	}
}

/// Requests an user to be the current user's friend
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendRequest {
	/// The ID of the user to request to be friends with
	pub id: crate::model::id::Invite,
}

#[cfg(feature = "ws")]
impl Requestable for FriendRequest {
	fn request_type(&self) -> RequestType {
		RequestType::FriendRequestSend
	}
}

/// Accepts another user's request to be friends
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptFriendRequest {
	/// The ID of the user that this response is to
	pub id: crate::model::id::Invite,
}

#[cfg(feature = "ws")]
impl Requestable for AcceptFriendRequest {
	fn request_type(&self) -> RequestType {
		RequestType::FriendRequestAccept
	}
}

/// Decline another user's request to be friends
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclineFriendRequest {
	/// The ID of the user that this response is to
	pub id: crate::model::id::Invite,
}

#[cfg(feature = "ws")]
impl Requestable for DeclineFriendRequest {
	fn request_type(&self) -> RequestType {
		RequestType::FriendRequestDecline
	}
}

/// Removes an user from the current user's friends list
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnFriend {
	/// The ID of the user to remove from the friends list
	pub id: crate::model::id::Invite,
}

#[cfg(feature = "ws")]
impl Requestable for UnFriend {
	fn request_type(&self) -> RequestType {
		RequestType::UnFriend
	}
}
