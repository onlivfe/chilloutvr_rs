use racal::Queryable;

use crate::model::{ApiAuth, Friends, ResponseDataWrapper};

#[derive(Default, Debug, Clone)]
pub struct FriendList();

impl Queryable<ApiAuth, ResponseDataWrapper<Friends>> for FriendList {
	fn url(&self) -> String {
		format!("{}/friends", crate::API_V1_HTTP_URL)
	}
}

#[derive(Default, Debug, Clone)]
pub struct FriendRequests();

impl Queryable<ApiAuth, ResponseDataWrapper<crate::model::FriendRequests>>
	for FriendRequests
{
	fn url(&self) -> String {
		format!("{}/friends/requests", crate::API_V1_HTTP_URL)
	}
}
