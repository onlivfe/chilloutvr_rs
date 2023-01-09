use serde::{Deserialize, Serialize};

use super::{Listenable, ResponseType};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOnlineStatusChange {
	pub id: crate::model::id::User,
	pub is_online: bool,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkipErrors(
	#[serde_as(as = "serde_with::VecSkipError<_>")] Vec<UserOnlineStatusChange>,
);

impl Listenable for Vec<UserOnlineStatusChange> {
	const RESPONSE_TYPE: ResponseType = ResponseType::OnlineFriends;
}

impl Listenable for SkipErrors {
	const RESPONSE_TYPE: ResponseType = ResponseType::OnlineFriends;
}
