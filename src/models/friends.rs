use crate::{AssetBase, AssetBaseWithCategories, Queryable};

#[derive(Default, Debug, Clone)]
pub struct FriendListQuery();

impl Queryable for FriendListQuery {
	type ResponseType = AssetBaseWithCategories;
	fn url(&self) -> String {
		format!("{}/friends", crate::API_V1_HTTP_URL)
	}
}

#[derive(Default, Debug, Clone)]
pub struct FriendRequestsQuery();

impl Queryable for FriendRequestsQuery {
	type ResponseType = AssetBase;
	fn url(&self) -> String {
		format!("{}/friends/requests", crate::API_V1_HTTP_URL)
	}
}
