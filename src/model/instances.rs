#[cfg(feature = "http")]
use crate::model::{AssetBase, AssetBaseWithTags, UserBase, UserDetails};
use serde::{Deserialize, Serialize};

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InstancePrivacy {
	Public,
	FriendsOfFriends,
	Friends,
	Group,
	EveryoneCanInvite,
	OwnerMustInvite,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[non_exhaustive]
pub enum InstanceRegion {
	#[serde(rename = "eu")]
	Europe,
	#[serde(rename = "us")]
	UnitedStates,
	#[serde(rename = "as")]
	Asia,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde_with::serde_as]
#[serde(rename_all = "camelCase")]
pub struct InstanceDetails {
	pub id: crate::model::id::Instance,
	pub name: String,
	pub region: InstanceRegion,
	pub game_mode_id: String,
	pub game_mode_name: String,
	pub world: AssetBase,
	pub max_players: u32,
	pub current_player_count: u32,
	#[serde(default)]
	#[cfg_attr(not(feature = "strict"), serde_as(as = "serde_with::VecSkipError<_>"))]
	pub members: Vec<UserBase>,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedInstanceDetails {
	#[serde(flatten)]
	pub base: InstanceDetails,
	pub instance_setting_privacy: InstancePrivacy,
	pub author: UserBase,
	pub owner: UserDetails,
	pub world: AssetBaseWithTags,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceHost {
	pub fqdn: String,
	pub port: u32,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceJoinResponse {
	pub host: InstanceHost,
	pub jwt: String,
	pub world: AssetBase,
}
