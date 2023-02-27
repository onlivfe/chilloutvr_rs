use serde::{Deserialize, Serialize};

#[cfg(feature = "http")]
use crate::model::{AssetBase, AssetBaseWithTags, UserBase, UserDetails};

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
/// Who can join the instance
pub enum InstancePrivacy {
	/// Anyone can join
	Public,
	/// Any friend of anyone in the instance can join
	FriendsOfFriends,
	/// Friends of the instance owner can join
	Friends,
	/// Members of the instance's group can join
	Group,
	/// Anyone in the instance can invite, and joining needs an invite
	EveryoneCanInvite,
	/// The owner can invite, and joining needs an invite
	OwnerMustInvite,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[non_exhaustive]
/// The region  of the instance
pub enum InstanceRegion {
	#[serde(rename = "eu")]
	/// The instance is `eu`
	Europe,
	#[serde(rename = "us")]
	/// The instance is `us`
	UnitedStates,
	#[serde(rename = "as")]
	/// The instance is `as`
	Asia,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde_with::serde_as]
#[serde(rename_all = "camelCase")]
/// Details about an instance
pub struct InstanceDetails {
	/// The instance's ID
	pub id: crate::id::Instance,
	/// The name of the instance
	pub name: String,
	/// Where around the world is the instance hosted
	pub region: InstanceRegion,
	/// The ID of the game mode of the instance, not really useful as of writing
	pub game_mode_id: String,
	/// The name of the game mode of the instance, not really useful as of
	/// writing
	pub game_mode_name: String,
	/// The world that the instance is on
	pub world: AssetBase,
	/// How many players can fit into the instance
	pub max_players: u32,
	/// How many players are currently in the instance
	pub current_player_count: u32,
	#[serde(default)]
	#[cfg_attr(
		not(feature = "debug"),
		serde_as(as = "serde_with::VecSkipError<_>")
	)]
	/// A list of the users currently in the instance
	pub members: Vec<UserBase>,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Full details about an instance
pub struct ExtendedInstanceDetails {
	#[serde(flatten)]
	/// Base details about an instance
	pub base: InstanceDetails,
	/// Who can join the instance
	pub instance_setting_privacy: InstancePrivacy,
	/// Who created the instance
	pub author: UserBase,
	/// Who is the current owner of the instance
	pub owner: UserDetails,
	/// What world is the instance on
	pub world: AssetBaseWithTags,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details about the host of an instance
pub struct InstanceHost {
	/// The fully qualified domain name of the host
	pub fqdn: String,
	/// The port to use for joining the instance
	pub port: u32,
}

#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Details for joining an instance
pub struct InstanceJoinResponse {
	/// The instance host details
	pub host: InstanceHost,
	/// An authentication token for joining the instance
	pub jwt: String,
	/// Details of the instance's world
	pub world: AssetBase,
}
