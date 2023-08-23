#[cfg(feature = "http")]
use racal::Queryable;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ws")]
use super::{RequestType, Requestable};
#[cfg(feature = "http")]
use crate::{
	model::{ResponseDataWrapper, UserAuth},
	query::NoAuthentication,
};

/// Gets details about a specific user
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UserDetails {
	/// The ID of the user to get more information about
	pub user_id: crate::id::User,
}

#[cfg(feature = "http")]
impl Queryable<NoAuthentication, ResponseDataWrapper<crate::model::UserDetails>>
	for UserDetails
{
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/users/{}", crate::API_V1_HTTP_URL, &self.user_id)
	}
}

/// Credentials to try to login and fetch an access token with
#[cfg(feature = "http")]
#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginCredentials {
	/// The email address, which the API calls username
	#[serde(rename = "username")]
	pub email: String,
	/// The password of the account
	pub password: String,
}

#[cfg(feature = "http")]
impl std::fmt::Debug for LoginCredentials {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("LoginCredentials")
			.field("email", &self.email)
			.field("password", &"*****")
			.finish()
	}
}

/// Saved access key and username which can also be used to login
#[derive(Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedLoginCredentials {
	/// The username the access key is for
	pub username: String,
	/// The access key to authenticate API requests with
	#[serde(rename = "password")]
	pub access_key: String,
}

impl std::fmt::Debug for SavedLoginCredentials {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SavedLoginCredentials")
			.field("username", &self.username)
			.field("token", &"*****")
			.finish()
	}
}

#[cfg(feature = "http")]
impl From<UserAuth> for SavedLoginCredentials {
	fn from(auth: UserAuth) -> Self {
		Self { access_key: auth.access_key, username: auth.username }
	}
}

impl From<&Self> for SavedLoginCredentials {
	fn from(value: &Self) -> Self {
		Self {
			access_key: value.access_key.clone(),
			username: value.username.clone(),
		}
	}
}

/// Authentication to login to CVR
#[cfg(feature = "http")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "auth_type")]
pub enum AuthType {
	/// Authentication using the login info
	LoginProfile(LoginCredentials),
	/// Authentication from a previous login
	LoginCredentials(SavedLoginCredentials),
}

impl From<LoginCredentials> for AuthType {
	fn from(value: LoginCredentials) -> Self { Self::LoginProfile(value) }
}
impl From<SavedLoginCredentials> for AuthType {
	fn from(value: SavedLoginCredentials) -> Self {
		Self::LoginCredentials(value)
	}
}

#[cfg(feature = "http")]
impl Queryable<NoAuthentication, ResponseDataWrapper<UserAuth>> for AuthType {
	fn url(&self, _: &NoAuthentication) -> String {
		format!("{}/users/auth", crate::API_V1_HTTP_URL)
	}

	fn body(&self, _: &NoAuthentication) -> Option<serde_json::Result<Vec<u8>>> {
		Some(serde_json::to_vec(self))
	}

	fn method(&self, _state: &NoAuthentication) -> racal::RequestMethod {
		racal::RequestMethod::Post
	}
}

/// Adds an user to the blocked users list
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockUser {
	/// The ID of the user to add to the blocked users list
	pub id: crate::id::Invite,
}

#[cfg(feature = "ws")]
impl Requestable for BlockUser {
	fn request_type(&self) -> RequestType { RequestType::BlockUser }
}

/// Removes an user to the blocked users list
#[cfg(feature = "ws")]
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnBlockUser {
	/// The ID of the user to remove to the blocked users list
	pub id: crate::id::Invite,
}

#[cfg(feature = "ws")]
impl Requestable for UnBlockUser {
	fn request_type(&self) -> RequestType { RequestType::UnBlockUser }
}
