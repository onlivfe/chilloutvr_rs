use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseDataWrapper<T> {
	/// Non-empty string, otherwise deserialized to none
	#[serde(default)]
	#[serde_as(as = "NoneAsEmptyString")]
	pub message: Option<String>,
	pub data: T,
}

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
