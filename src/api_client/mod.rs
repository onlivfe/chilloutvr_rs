//! An optional API client feature using `reqwest`
//!
//! Besides using this, you could instead easily implement your own client using
//! a different HTTP library with the `chilloutvr::query::Queryable` trait.

use std::num::NonZeroU32;

use governor::{
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota,
	RateLimiter,
};
use racal::{Queryable, RequestMethod};
use reqwest::{header::HeaderMap, Client};
use serde::de::DeserializeOwned;

use crate::{model::ApiAuth, query::CvrApiUnwrapping};

#[derive(Debug)]
pub enum ApiError {
	Serde(serde_json::Error),
	Reqwest(reqwest::Error),
}

impl From<serde_json::Error> for ApiError {
	fn from(err: serde_json::Error) -> Self {
		Self::Serde(err)
	}
}

impl From<reqwest::Error> for ApiError {
	fn from(err: reqwest::Error) -> Self {
		Self::Reqwest(err)
	}
}

#[derive(Debug)]
pub struct CVR {
	client: Client,
	rate_limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
}

pub enum SupportedApiStates {
	Unauthenticated,
	Authenticated(ApiAuth),
}

impl From<()> for SupportedApiStates {
	fn from(_: ()) -> Self {
		SupportedApiStates::Unauthenticated
	}
}

impl From<ApiAuth> for SupportedApiStates {
	fn from(auth: ApiAuth) -> Self {
		SupportedApiStates::Authenticated(auth)
	}
}

impl CVR {
	/// Creates an API client
	#[must_use]
	fn client(user_agent: String, auth: Option<ApiAuth>) -> Client {
		let builder = Client::builder();
		let mut headers = HeaderMap::new();

		if let Some(auth) = auth {
			headers.append("Username", auth.username.parse().unwrap());
			headers.append("AccessKey", auth.access_key.parse().unwrap());
		}

		builder.user_agent(user_agent).default_headers(headers).build().unwrap()
	}

	/// Creates a new CVR API client
	///
	/// # Panics
	///
	/// If the user agent is invalid for use in the header
	#[must_use]
	pub fn new(user_agent: String, auth: impl Into<Option<ApiAuth>>) -> Self {
		Self {
			client: CVR::client(user_agent, auth.into()),
			// ~5 seconds per request sustained over one minute, allowing up to a request
			// per second in bursts.
			rate_limiter: RateLimiter::direct(
				Quota::per_minute(NonZeroU32::try_from(12).unwrap())
					.allow_burst(NonZeroU32::try_from(5).unwrap()),
			),
		}
	}

	/// Sends a query to the CVR API
	///
	/// # Errors
	///
	/// If something with the request failed.
	pub async fn query<ReturnType, WrappedType, FromState, T>(
		&self,
		queryable: T,
	) -> Result<ReturnType, ApiError>
	where
		WrappedType: CvrApiUnwrapping<ReturnType> + DeserializeOwned,
		FromState: Into<SupportedApiStates>,
		T: Queryable<FromState, WrappedType> + Send + Sync,
	{
		let mut request = self.client.request(
			match queryable.method() {
				RequestMethod::Get => reqwest::Method::GET,
				RequestMethod::Head => reqwest::Method::HEAD,
				RequestMethod::Patch => reqwest::Method::PATCH,
				RequestMethod::Post => reqwest::Method::POST,
				RequestMethod::Put => reqwest::Method::PUT,
				RequestMethod::Delete => reqwest::Method::DELETE,
			},
			queryable.url(),
		);
		if let Some(body) = queryable.body() {
			request = request.body(body?);
		}

		self.rate_limiter.until_ready().await;
		let response = request.send().await?.error_for_status()?;
		// TODO: Figure out if there are any extra rate limit headers to respect

		#[cfg(feature = "debug")]
		{
			let text = response.text().await?;
			dbg!(&text);
			Ok(serde_json::from_str::<WrappedType>(&text)?.unwrap_data())
		}
		#[cfg(not(feature = "debug"))]
		{
			Ok(response.json::<WrappedType>().await?.unwrap_data())
		}
	}
}
