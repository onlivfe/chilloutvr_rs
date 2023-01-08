use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDataWrapper<T> {
	#[serde(default)]
	pub message: String,
	pub data: T,
}

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
