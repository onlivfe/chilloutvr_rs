#[cfg(feature = "http")]
use racal::Queryable;
#[cfg(feature = "http")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "http")]
use crate::{model::ResponseDataWrapper, query::SavedLoginCredentials};

/// Gets your categories
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Categories;

#[cfg(feature = "http")]
impl
	Queryable<
		SavedLoginCredentials,
		ResponseDataWrapper<crate::model::Categories>,
	> for Categories
{
	fn url(&self, _: &SavedLoginCredentials) -> String {
		format!("{}/categories", crate::API_V1_HTTP_URL)
	}
}
