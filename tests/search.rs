#![cfg(feature = "api_client")]

use chilloutvr::model::id;
mod common;

#[test]
#[ignore]
fn search() -> Result<(), chilloutvr::api_client::ApiError> {
	let query = chilloutvr::query::Search { term: "club".to_string() };
	let results = tokio_test::block_on(common::api_client().query(query))?;

	dbg!(&results);

	assert!(results.0.len() >= 2);

	let first_result = match results.0.first() {
		Some(result) => result,
		None => panic!("expected response to contain at least a single world"),
	};

	assert!(!id::Any::from(first_result.id.clone()).as_ref().is_empty());

	Ok(())
}
