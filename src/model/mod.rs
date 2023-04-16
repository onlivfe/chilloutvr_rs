//! Models of the API responses to queries.

use serde::{de::Error, Deserialize, Serialize};
use serde_with::serde_as;

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

mod categories;
pub use categories::*;

/// Seems like a lot if not all of the API calls are wrapped
/// in a generic data/message struct.
#[cfg(any(feature = "http"))]
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDataWrapper<T> {
	/// Deserialized to empty string if missing or null
	#[serde(default)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	pub message: String,
	/// The actual data
	pub data: T,
}

#[cfg(feature = "ws")]
#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	strum::Display,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[non_exhaustive]
/// The actual response data of an incoming WebSocket message
pub enum WsResponseData {
	/// Some sorta alert/notification most likely?
	MenuPopup(serde_json::Value),
	/// Some sorta alert/notification most likely?
	HudMessage(serde_json::Value),
	/// Some sorta alert/notification most likely?
	PushNotification(serde_json::Value),
	/// Update of the status of online friends
	OnlineFriends(Friends),
	/// Update of current invites
	Invites(Invites),
	/// Update of current invite requests
	RequestInvites(InviteRequest),
	/// Update of current friend requests
	FriendRequest(FriendRequests),
}

// Auto derives don't seem to support this, see https://github.com/serde-rs/serde/issues/745
impl<'de> serde::Deserialize<'de> for WsResponseData {
	fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
		use serde_json::Value;
		let mut value = Value::deserialize(d)?;
		let resp_type = value
			.get("responseType")
			.ok_or_else(|| D::Error::missing_field("responseType"))?
			.as_u64()
			.ok_or_else(|| {
				D::Error::custom("responseType was not an unsigned int")
			})?;
		let data = value
			.get_mut("data")
			.ok_or_else(|| D::Error::missing_field("data"))?
			.take();

		Ok(match resp_type {
			0 => Self::MenuPopup(data),
			1 => Self::HudMessage(data),
			2 => Self::PushNotification(data),
			10 => Self::OnlineFriends(serde_json::from_value(data).map_err(|e| {
				D::Error::custom(format!(
					"deserializing OnlineFriends data failed: {e:?}"
				))
			})?),
			15 => Self::Invites(serde_json::from_value(data).map_err(|e| {
				D::Error::custom(format!("deserializing Invites data failed: {e:?}"))
			})?),
			20 => {
				Self::RequestInvites(serde_json::from_value(data).map_err(|e| {
					D::Error::custom(format!(
						"deserializing RequestInvites data failed: {e:?}"
					))
				})?)
			}
			25 => Self::FriendRequest(serde_json::from_value(data).map_err(|e| {
				D::Error::custom(format!(
					"deserializing FriendRequest data failed: {e:?}"
				))
			})?),
			type_ => {
				return Err(D::Error::invalid_value(
					serde::de::Unexpected::Unsigned(type_),
					&"a supported WS message responseType",
				));
			}
		})
	}
}

#[cfg(any(feature = "http"))]
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A WebSocket response
pub struct WsResponse {
	/// Deserialized to empty string if missing or null
	#[serde(default)]
	#[serde_as(deserialize_as = "serde_with::DefaultOnNull")]
	pub message: String,
	/// The actual data
	#[serde(flatten)]
	pub data: WsResponseData,
}
