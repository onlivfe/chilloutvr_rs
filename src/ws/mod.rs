use serde::{Deserialize, Serialize};

mod types;
pub use types::*;

mod users;
pub use users::*;

mod worlds;
pub use worlds::*;

mod invites;
pub use invites::*;

// TODO: Figure out how to bend the type system so that the Request / Response types
// are conveniently mapped to the corresponding rust types.

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request {
	pub request_type: RequestType,
	pub data: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
	pub request_type: RequestType,
	pub message: Option<String>,
	pub data: Option<String>,
}
