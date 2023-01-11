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
#[cfg(any(feature = "http", feature = "ws"))]
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
	Hash,
	Deserialize,
	Serialize,
	strum::Display,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[non_exhaustive]
pub enum ResponseType {
	MenuPopup = 0,
	HudMessage = 1,
	PushNotification = 2,
	OnlineFriends = 10,
	Invites = 15,
	RequestInvites = 20,
	FriendRequest = 25,
}

/// Data for a WS response
#[cfg(feature = "ws")]
pub trait Listenable {
	const RESPONSE_TYPE: ResponseType;
}

#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ResponseWrapper<T> {
	pub request_type: ResponseType,
	pub message: Option<String>,
	#[serde(flatten)]
	pub data: ResponseDataWrapper<T>,
}
