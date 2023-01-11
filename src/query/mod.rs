//! Data about queries which can be used by your HTTP client of choice.
//!
//! An example implementation is provided with reqwest,
//! from `chilloutvr::api_client` if you enabled the `api_client` feature.

pub trait CvrApiUnwrapping<UnwrappedType>: DeserializeOwned {
	fn unwrap_data(self) -> UnwrappedType;
}

impl<T: DeserializeOwned> CvrApiUnwrapping<T> for T {
	fn unwrap_data(self) -> T {
		self
	}
}

impl<T: DeserializeOwned> CvrApiUnwrapping<T> for ResponseDataWrapper<T> {
	fn unwrap_data(self) -> T {
		self.data
	}
}

mod friends;
pub use friends::*;
mod instances;
pub use instances::*;
mod users;
use serde::de::DeserializeOwned;
pub use users::*;
mod searches;
pub use searches::*;
mod worlds;
pub use worlds::*;

use crate::model::ResponseDataWrapper;
