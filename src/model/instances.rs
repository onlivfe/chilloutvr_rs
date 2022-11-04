use serde::{Deserialize, Serialize};

use crate::model::{AssetBase, AssetBaseWithTags, UserBase, UserDetails};

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstanceBase {
	pub id: String,
	pub name: String,
	pub region: InstanceRegion,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstanceDetails {
	#[serde(flatten)]
	pub base: InstanceBase,
	pub game_mode_id: String,
	pub game_mode_name: String,
	pub world: AssetBase,
	pub max_players: u32,
	pub current_player_count: u32,
	#[serde(default)]
	pub members: Vec<UserBase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtendedInstanceDetails {
	#[serde(flatten)]
	pub base: InstanceDetails,
	pub instance_setting_privacy: InstancePrivacy,
	pub author: UserBase,
	pub owner: UserDetails,
	pub world: AssetBaseWithTags,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstanceHost {
	pub fqdn: String,
	pub port: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstanceJoinResponse {
	pub host: InstanceHost,
	pub jqt: String,
	pub world: AssetBase,
}
