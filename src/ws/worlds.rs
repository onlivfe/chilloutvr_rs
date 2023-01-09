use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldDetails {
	pub id: crate::model::id::Asset,
	pub name: String,
	pub image_url: String,
}
