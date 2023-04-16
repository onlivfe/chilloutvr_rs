//! Data about queries which can be used with your HTTP client of choice.
//!
//! An example implementation is provided with reqwest,
//! from `chilloutvr::api_client` if you enabled the `api_client` feature.

use serde::{Deserialize, Serialize};

mod categories;
pub use categories::*;
mod friends;
pub use friends::*;
mod instances;
pub use instances::*;
mod users;
pub use users::*;
mod searches;
pub use searches::*;
mod worlds;
pub use worlds::*;
mod invites;
pub use invites::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
/// [`racal::Queryable`](racal::Queryable)'s `RequiredApiState`.
///
/// Even unauthenticated requests to CVR's HTTP API should take rate limits
/// into account, thus not using `()` for the API state.
pub struct NoAuthentication {}

impl racal::FromApiState<Self> for NoAuthentication {
	fn from_state(state: &Self) -> &Self { state }
}

impl racal::FromApiState<SavedLoginCredentials> for NoAuthentication {
	fn from_state(_: &SavedLoginCredentials) -> &Self { &Self {} }
}

impl racal::FromApiState<Self> for SavedLoginCredentials {
	fn from_state(state: &Self) -> &Self { state }
}

/// A WS message going from the client to the CVR server
#[cfg(feature = "ws")]
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Deserialize,
	Serialize,
	strum::Display,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum RequestType {
	/// Should be sent as a keepalive to keep an account marked as online.
	///
	/// Can for example be sent every 30 seconds.
	SelfOnline = 0,
	FriendRequestSend = 5,
	FriendRequestAccept = 6,
	FriendRequestDecline = 7,
	UnFriend = 8,
	InviteSend = 10,
	InviteExpire = 11,
	RequestInvite = 15,
	RequestInviteAccept = 16,
	RequestInviteDecline = 17,
	BlockUser = 30,
	UnBlockUser = 31,
	GlobalMessage = 100,
	UserMessage = 101,
	SystemGlobalAnnouncement = 110,
	SystemUserAnnouncement = 111,
}

/// Data for a WS request
#[cfg(feature = "ws")]
pub trait Requestable {
	/// The type of the request
	fn request_type(&self) -> RequestType;
}

/// The WS API message base structure
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestWrapper<T> {
	/// An enum that indicates the WS message's type
	pub request_type: RequestType,
	/// The actual data of the message
	pub data: T,
}

/// Mark the connected user account as being online
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Online;

#[cfg(feature = "ws")]
impl Requestable for Online {
	fn request_type(&self) -> RequestType { RequestType::SelfOnline }
}
