#![cfg(feature = "http_client")]

use chilloutvr::{
	api_client::{ApiClient, ApiError},
	model::WorldListItem,
};
mod common;

#[tokio::test]
#[ignore]
async fn active_worlds() -> Result<(), ApiError> {
	let api_client = common::api_client();

	let query = chilloutvr::query::WorldListQuery {
		category: "wrldactive".try_into().unwrap(),
	};
	let world_instances: Vec<WorldListItem> = api_client.query(query).await?.data;

	dbg!(&world_instances);
	assert!(world_instances.len() > 1);

	let first = world_instances.first().unwrap();

	assert!(!first.base.name.is_empty());
	assert!(!first.player_count > 0);

	Ok(())
}
