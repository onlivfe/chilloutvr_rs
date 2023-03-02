#![cfg(feature = "http_client")]

use chilloutvr::{api_client::ApiClient, id, model::SearchResults};
mod common;

#[tokio::test]
#[ignore]
async fn search() -> Result<(), chilloutvr::api_client::ApiError> {
	let api_client = common::api_client();

	let query = chilloutvr::query::Search { term: "club".to_string() };
	let results: SearchResults = api_client.query(query).await?.data;

	dbg!(&results);

	assert!(results.0.len() >= 2);

	let first_result = match results.0.first() {
		Some(result) => result,
		None => panic!("expected response to contain at least a single world"),
	};

	assert!(!id::Any::from(first_result.id.clone()).as_ref().is_empty());

	Ok(())
}
