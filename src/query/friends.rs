use crate::{model::Friends, Queryable};

#[derive(Default, Debug, Clone)]
pub struct FriendList();

impl Queryable for FriendList {
	type ResponseType = Friends;
	fn url(&self) -> String {
		format!("{}/friends", crate::API_V1_HTTP_URL)
	}
}

#[derive(Default, Debug, Clone)]
pub struct FriendRequests();

impl Queryable for FriendRequests {
	type ResponseType = crate::model::FriendRequests;
	fn url(&self) -> String {
		format!("{}/friends/requests", crate::API_V1_HTTP_URL)
	}
}
