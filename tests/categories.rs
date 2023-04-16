#![cfg(feature = "http_client")]

use chilloutvr::{
	api_client::{ApiClient, ApiError},
	model::Categories,
};
mod common;

#[tokio::test]
#[ignore]
async fn categories() -> Result<(), ApiError> {
	let api_client = common::api_client();

	let query = chilloutvr::query::Categories;
	let categories: Categories = api_client.query(query).await?.data;

	dbg!(&categories);
	assert!(categories.avatars.len() > 1);
	assert!(categories.friends.len() > 1);
	assert!(categories.spawnables.len() > 1);
	assert!(categories.worlds.len() > 5);

	let first_avi = categories.avatars.first().unwrap();

	assert!(!first_avi.name.is_empty());

	Ok(())
}
