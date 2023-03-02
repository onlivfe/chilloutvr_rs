//! An optional API client feature using `reqwest`
//!
//! Besides using this, you could instead easily implement your own client using
//! a different HTTP library with the [`racal::Queryable`](racal::Queryable)
//! trait. Though this does additionally support unwrapping the message/data of
//! the CVR API responses.
//!
//! If you're implementing your own API client, you need to implement two
//! possible API states:
//!
//! 1. [`chilloutvr::query::NoAuthentication`](crate::query::NoAuthentication)
//!
//! > Doesn't require authentication but still needs to be rate limited
//! > properly.
//!
//! 2. [`chilloutvr::query::SavedLoginCredentials`](crate::model::SavedLoginCredentials)
//!
//! > Requires the `Username` and `AccessKey` headers
//! in addition to the rate limiting.
//!
//! The WebSocket API client is more messy, in this implementation the
//! connection is opened lazily (on first use) and never manually closed again
//! afterwards.

#[cfg(feature = "http_client")]
use std::num::NonZeroU32;

#[cfg(feature = "http_client")]
use governor::{
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota,
	RateLimiter,
};
#[cfg(feature = "http_client")]
pub use racal::reqwest::ApiClient;
#[cfg(feature = "http_client")]
use reqwest::RequestBuilder;
#[cfg(feature = "http_client")]
use reqwest::{header::HeaderMap, Client};
use serde::ser::Serialize;

use crate::query::{NoAuthentication, SavedLoginCredentials};

#[cfg(feature = "ws_client")]
mod ws;

/// An error that may happen with an API query
#[derive(Debug)]
pub enum ApiError {
	/// An error happened with serialization
	Serde(serde_json::Error),
	/// An error happened with the request itself
	#[cfg(feature = "http_client")]
	Reqwest(reqwest::Error),
	/// An error happened with the WS connection
	#[cfg(feature = "ws_client")]
	Tungstenite(tokio_tungstenite::tungstenite::Error),
}

impl From<serde_json::Error> for ApiError {
	fn from(err: serde_json::Error) -> Self { Self::Serde(err) }
}

#[cfg(feature = "http_client")]
impl From<reqwest::Error> for ApiError {
	fn from(err: reqwest::Error) -> Self { Self::Reqwest(err) }
}

#[cfg(feature = "http_client")]
impl From<racal::reqwest::ApiError> for ApiError {
	fn from(err: racal::reqwest::ApiError) -> Self {
		match err {
			racal::reqwest::ApiError::Reqwest(e) => Self::Reqwest(e),
			racal::reqwest::ApiError::Serde(e) => Self::Serde(e),
		}
	}
}

#[cfg(feature = "ws_client")]
impl From<tokio_tungstenite::tungstenite::Error> for ApiError {
	fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
		Self::Tungstenite(err)
	}
}

#[cfg(feature = "http_client")]
type NormalRateLimiter =
	RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>;

#[cfg(feature = "http_client")]
#[must_use]
fn http_rate_limiter() -> NormalRateLimiter {
	// ~5 seconds per request sustained over one minute, allowing up to a request
	// per second in bursts.
	RateLimiter::direct(
		Quota::per_minute(NonZeroU32::try_from(12).unwrap())
			.allow_burst(NonZeroU32::try_from(5).unwrap()),
	)
}

/// The main API client without authentication
#[cfg(feature = "http_client")]
pub struct UnauthenticatedCVR {
	user_agent: String,
	http: Client,
	http_rate_limiter: NormalRateLimiter,
}

#[cfg(feature = "http_client")]
#[async_trait::async_trait]
impl racal::reqwest::ApiClient<NoAuthentication> for UnauthenticatedCVR {
	fn state(&self) -> &NoAuthentication { &NoAuthentication {} }

	fn client(&self) -> &reqwest::Client { &self.http }

	async fn before_request(
		&self, req: RequestBuilder,
	) -> Result<RequestBuilder, racal::reqwest::ApiError> {
		self.http_rate_limiter.until_ready().await;
		Ok(req)
	}
}

/// The main API client with authentication
pub struct AuthenticatedCVR {
	user_agent: String,
	auth: SavedLoginCredentials,
	#[cfg(feature = "http_client")]
	http: Client,
	#[cfg(feature = "http_client")]
	http_rate_limiter: NormalRateLimiter,
	#[cfg(feature = "ws_client")]
	ws: tokio::sync::RwLock<Option<ws::Client>>,
}

#[cfg(feature = "http_client")]
#[async_trait::async_trait]
impl racal::reqwest::ApiClient<SavedLoginCredentials> for AuthenticatedCVR {
	fn state(&self) -> &SavedLoginCredentials { &self.auth }

	fn client(&self) -> &reqwest::Client { &self.http }

	async fn before_request(
		&self, req: RequestBuilder,
	) -> Result<RequestBuilder, racal::reqwest::ApiError> {
		self.http_rate_limiter.until_ready().await;
		Ok(req)
	}
}

impl AuthenticatedCVR {
	/// Creates an API client
	#[cfg(feature = "http_client")]
	fn http_client(
		user_agent: &str, auth: &SavedLoginCredentials,
	) -> Result<Client, ApiError> {
		use serde::ser::Error;

		let builder = Client::builder();
		let mut headers = HeaderMap::new();

		headers.insert(
			"Username",
			auth.username.parse().map_err(|_| {
				serde_json::Error::custom("Couldn't turn username into a header")
			})?,
		);
		headers.insert(
			"AccessKey",
			auth.access_key.parse().map_err(|_| {
				serde_json::Error::custom("Couldn't turn access_key into a header")
			})?,
		);

		Ok(builder.user_agent(user_agent).default_headers(headers).build()?)
	}

	/// Removes authentication to the API client
	///
	/// # Errors
	///
	/// If deserializing user agent fails.
	#[cfg(feature = "http_client")]
	pub fn downgrade(self) -> Result<UnauthenticatedCVR, ApiError> {
		Ok(UnauthenticatedCVR {
			http: UnauthenticatedCVR::http_client(&self.user_agent)?,
			http_rate_limiter: self.http_rate_limiter,
			user_agent: self.user_agent,
		})
	}

	/// Creates a new authenticated CVR API client
	///
	/// # Errors
	///
	/// If deserializing user agent into a header fails
	pub fn new(
		user_agent: String, auth: impl Into<SavedLoginCredentials> + Send,
	) -> Result<Self, ApiError> {
		let auth = auth.into();
		Ok(Self {
			#[cfg(feature = "http_client")]
			http: Self::http_client(&user_agent, &auth)?,
			#[cfg(feature = "http_client")]
			http_rate_limiter: http_rate_limiter(),
			#[cfg(feature = "ws_client")]
			ws: tokio::sync::RwLock::new(None),
			auth,
			user_agent,
		})
	}

	/// Opens the WebSocket connection if it wasn't already open
	///
	/// # Errors
	///
	/// If opening the WS connection fails
	#[cfg(feature = "ws_client")]
	pub async fn ws_connect(&self) -> Result<(), ApiError> {
		{
			let lock = self.ws.read().await;
			if let Some(ws_client) = &*lock {
				if ws_client.is_ok() {
					return Ok(());
				}
			}
		}

		#[cfg(feature = "http_client")]
		self.http_rate_limiter.until_ready().await;
		let client =
			ws::Client::new(self.user_agent.clone(), self.auth.clone()).await?;
		let mut lock = self.ws.write().await;
		*lock = Some(client);

		Ok(())
	}

	/// Closes the WebSocket connection if it is open
	#[cfg(feature = "ws_client")]
	pub async fn ws_disconnect(&self) {
		{
			let mut lock = self.ws.write().await;
			*lock = None;
		}
	}

	/// If the WS client is connected
	#[cfg(feature = "ws_client")]
	pub async fn ws_is_connected(&self) -> bool {
		let lock = self.ws.read().await;
		if let Some(ws_client) = &*lock {
			if ws_client.is_ok() {
				return true;
			}
		}
		false
	}

	/// Sends a WS message to the CVR API.
	///
	/// # Errors
	///
	/// If something with the request failed,
	/// or if the WS connection wasn't already open and creating it failed.
	#[cfg(feature = "ws_client")]
	pub async fn send(
		&self, requestable: impl crate::query::Requestable + Serialize + Send,
	) -> Result<(), ApiError> {
		{
			let lock = self.ws.read().await;
			if let Some(ws_client) = &*lock {
				if ws_client.is_ok() {
					return ws_client.send(requestable).await;
				}
			}
		}

		#[cfg(feature = "http_client")]
		self.http_rate_limiter.until_ready().await;
		let client =
			ws::Client::new(self.user_agent.clone(), self.auth.clone()).await?;
		let mut lock = self.ws.write().await;
		*lock = Some(client);
		let lock = lock.downgrade();
		(*lock)
			.as_ref()
			.expect("client should exist as lock was never dropped")
			.send(requestable)
			.await
	}

	/// Listens to events from the WS connection
	///
	/// # Errors
	///
	/// If creating the client fails,
	/// or if the WS connection wasn't already open and creating it failed.
	#[cfg(feature = "ws_client")]
	pub async fn listen(&self) -> Result<ws::ReceiverContainer, ApiError> {
		{
			let lock = self.ws.read().await;
			if let Some(ws_client) = &*lock {
				if ws_client.is_ok() {
					return Ok(ws_client.listen());
				}
			}
		}

		#[cfg(feature = "http_client")]
		self.http_rate_limiter.until_ready().await;
		let client =
			ws::Client::new(self.user_agent.clone(), self.auth.clone()).await?;
		let mut lock = self.ws.write().await;
		*lock = Some(client);
		let lock = lock.downgrade();
		Ok(
			(*lock)
				.as_ref()
				.expect("client should exist as lock was never dropped")
				.listen(),
		)
	}
}

impl UnauthenticatedCVR {
	/// Creates an unauthenticated API client
	#[cfg(feature = "http_client")]
	fn http_client(user_agent: &str) -> Result<Client, ApiError> {
		Ok(Client::builder().user_agent(user_agent).build()?)
	}

	/// Adds authentication to the API client
	///
	/// # Errors
	///
	/// If deserializing user agent or authentication fails.
	pub fn upgrade(
		self, auth: impl Into<SavedLoginCredentials> + Send,
	) -> Result<AuthenticatedCVR, ApiError> {
		let auth = auth.into();
		Ok(AuthenticatedCVR {
			#[cfg(feature = "http_client")]
			http: AuthenticatedCVR::http_client(&self.user_agent, &auth)?,
			#[cfg(feature = "http_client")]
			http_rate_limiter: self.http_rate_limiter,
			#[cfg(feature = "ws_client")]
			ws: tokio::sync::RwLock::new(None),
			#[cfg(feature = "ws_client")]
			auth,
			user_agent: self.user_agent,
		})
	}

	/// Creates a new CVR API client
	///
	/// # Errors
	///
	/// If deserializing user agent into a header fails,
	/// or if WS API is enabled & the connection establishment fails.
	#[cfg(feature = "http_client")]
	pub fn new(user_agent: String) -> Result<Self, ApiError> {
		Ok(Self {
			http: Self::http_client(&user_agent)?,
			http_rate_limiter: http_rate_limiter(),
			user_agent,
		})
	}
}
