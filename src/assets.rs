use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Asset {
	pub id: String,
	pub name: String,
	pub image_url: String,
}
