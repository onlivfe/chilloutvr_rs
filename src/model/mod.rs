use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, strum::EnumDiscriminants,
)]
#[serde(rename_all = "PascalCase")]
pub enum ResponseDataWrapper<T> {
	Message(String),
	Data(T),
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
