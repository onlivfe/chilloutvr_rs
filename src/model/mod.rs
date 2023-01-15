//! Models of the API responses to queries.

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};

pub mod id;

mod invites;
pub use invites::*;

mod users;
pub use users::*;

mod worlds;
pub use worlds::*;

mod instances;
pub use instances::*;

mod featureds;
pub use featureds::*;

mod assets;
pub use assets::*;

mod searches;
pub use searches::*;

/// Seems like a lot if not all of the API calls are wrapped
/// in a generic data/message struct.
#[cfg(any(feature = "http"))]
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDataWrapper<T> {
	/// Non-empty string, otherwise deserialized to none
	#[serde(default)]
	#[serde_as(as = "NoneAsEmptyString")]
	pub message: Option<String>,
	/// The actual data
	pub data: T,
}

#[cfg(feature = "ws")]
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Deserialize,
	strum::Display,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[non_exhaustive]
#[serde(tag = "responseType", content = "data")]
pub enum WsResponseData {
	#[serde(rename = "0")]
	MenuPopup(serde_json::Value),
	#[serde(rename = "1")]
	HudMessage(serde_json::Value),
	#[serde(rename = "2")]
	PushNotification(serde_json::Value),
	#[serde(rename = "10")]
	OnlineFriends(Friends),
	#[serde(rename = "15")]
	Invites(Invites),
	#[serde(rename = "20")]
	RequestInvites(InviteRequest),
	#[serde(rename = "25")]
	FriendRequest(FriendRequests),
}

#[cfg(any(feature = "http"))]
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsResponse {
	/// Non-empty string, otherwise deserialized to none
	#[serde(default)]
	#[serde_as(as = "NoneAsEmptyString")]
	pub message: Option<String>,
	/// The actual data
	#[serde(flatten)]
	pub data: WsResponseData,
}
