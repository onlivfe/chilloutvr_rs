#![cfg(feature = "api_client")]

use chilloutvr::model::id;
mod common;

#[test]
#[ignore]
fn user() -> Result<(), chilloutvr::api_client::ApiError> {
	let user_id = id::User::try_from("81c652f6-f2e9-6d48-fff9-1584fc6ac95d").unwrap();
	let query = chilloutvr::query::UserDetails { user_id: user_id.clone() };
	let user = tokio_test::block_on(common::api_client().query(query))?;

	dbg!(&user);

	assert_eq!(user.base.id, user_id);
	assert!(!user.avatar.name.is_empty());
	assert!(!user.featured_badge.name.is_empty());
	assert!(!user.featured_group.image.is_empty());

	Ok(())
}
