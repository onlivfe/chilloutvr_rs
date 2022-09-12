use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldInstance {
	#[serde(default)]
	pub id: String,
	#[serde(default)]
	pub name: String,
	pub player_count: u32,
	pub max_player_count: u32,
	pub region: String,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldInstances {
	#[serde_as(as = "serde_with::VecSkipError<_>")]
	pub instances: Vec<WorldInstance>,
}
