//! Data about queries which can be used by your HTTP client of choice.
//!
//! An example implementation is provided with reqwest,
//! from `chilloutvr::api_client` if you enabled the `api_client` feature.

use serde::de::DeserializeOwned;

/// Data for a HTTP request & response
pub trait Queryable<RequiredApiState> {
	/// The type of the expected OK response
	type ResponseType: DeserializeOwned;

	/// The URL of the request
	fn url(&self) -> String;
	/// Creates a body for the request
	fn body(&self) -> Option<serde_json::Result<Vec<u8>>> {
		None
	}

	/// If the response type is wrapped in
	/// [`chilloutvr::model::ResponseDataWrapper`]
	fn wrapped_response(&self) -> bool {
		true
	}
}

mod friends;
pub use friends::*;
mod instances;
pub use instances::*;
mod users;
pub use users::*;
mod searches;
pub use searches::*;
mod worlds;
pub use worlds::*;
