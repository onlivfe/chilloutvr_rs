#![cfg(feature = "api_client")]

use chilloutvr::model::ResponseDataWrapper;
mod common;

#[test]
#[ignore]
fn get_worlds() -> Result<(), chilloutvr::api_client::ApiError> {
	let query =
		chilloutvr::query::WorldListQuery { category: "Official Worlds".to_string() };
	let result = tokio_test::block_on(common::api_client().query(query))?;

	let worlds = match result {
		ResponseDataWrapper::Data(v) => v,
		_ => panic!("expected data, got {:?}", result),
	};

	let first_world = match worlds.first() {
		Some(world) => world,
		None => panic!("expected response to contain at least a single world"),
	};

	assert!(!first_world.id.is_empty());

	Ok(())
}
