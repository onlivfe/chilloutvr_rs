use std::num::NonZeroU32;

use governor::{
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota,
	RateLimiter,
};
use reqwest::Client;

use crate::Queryable;

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

pub struct CVR {
	client: Client,
	rate_limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
}

impl CVR {
	/// Creates a new CVR API client
	///
	/// # Panics
	///
	/// If the user agent is invalid for use in the header
	#[must_use]
	pub fn new(user_agent: String) -> Self {
		Self {
			client: Client::builder().user_agent(user_agent).build().unwrap(),
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
	pub async fn query<T: Queryable + Send>(
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

		let val: T::ResponseType = response.json().await?;

		Ok(val)
	}
}
