use crate::{
	model::{ApiAuth, Friends},
	query::Queryable,
};

#[derive(Default, Debug, Clone)]
pub struct FriendList();

impl Queryable<ApiAuth> for FriendList {
	type ResponseType = Friends;
	fn url(&self) -> String {
		format!("{}/friends", crate::API_V1_HTTP_URL)
	}
}

#[derive(Default, Debug, Clone)]
pub struct FriendRequests();

impl Queryable<ApiAuth> for FriendRequests {
	type ResponseType = crate::model::FriendRequests;
	fn url(&self) -> String {
		format!("{}/friends/requests", crate::API_V1_HTTP_URL)
	}
}
