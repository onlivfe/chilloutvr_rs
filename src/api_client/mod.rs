//! An optional API client feature using `reqwest`

use std::num::NonZeroU32;

use governor::{
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota,
	RateLimiter,
};
use reqwest::{header::HeaderMap, Client};

use crate::{
	model::{ApiAuth, ResponseDataWrapper},
	query::Queryable,
};

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
	pub async fn query<T: Queryable<impl Into<SupportedApiStates>> + Send + Sync>(
		&self,
		queryable: T,
	) -> Result<T::ResponseType, ApiError> {
		let mut request = self.client.get(queryable.url());
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
			let val: T::ResponseType = if queryable.wrapped_response() {
				serde_json::from_str::<ResponseDataWrapper<T::ResponseType>>(&text)?.data
			} else {
				serde_json::from_str(&text)?
			};
			Ok(val)
		}
		#[cfg(not(feature = "debug"))]
		{
			let val: T::ResponseType = if queryable.wrapped_response() {
				response.json::<ResponseDataWrapper<T::ResponseType>>().await?.data
			} else {
				response.json().await?
			};

			Ok(val)
		}
	}
}
