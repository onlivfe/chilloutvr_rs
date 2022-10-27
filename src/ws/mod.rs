use serde::{Deserialize, Serialize};

mod types;
pub use types::*;

mod users;
pub use users::*;

mod worlds;
pub use worlds::*;

mod invites;
pub use invites::*;

pub enum Error {
	RequestError,
	Serde,
}

/// Data for a WS request
pub trait Requestable {
	const REQUEST_TYPE: RequestType;
}

/// Data for a WS response
pub trait Listenable {
	const RESPONSE_TYPE: ResponseType;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct RequestWrapper<T> {
	pub request_type: RequestType,
	pub data: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
enum ResponseDataWrapper<T> {
	Message(String),
	Data(T),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct ResponseWrapper<T> {
	pub request_type: RequestType,
	pub message: Option<String>,
	#[serde(flatten)]
	pub data: ResponseDataWrapper<T>,
}
