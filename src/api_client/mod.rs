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
//! > Requires the `Username` and `AccessKey` headers in addition to the rate
//! > limiting.
//!
//! The WebSocket API client is more messy, in this implementation the
//! connection is opened lazily (on first use) and never manually closed again
//! afterwards.

#[cfg(feature = "http_client")]
use std::num::NonZeroU32;

#[cfg(feature = "http_client")]
use governor::{
	Quota,
	RateLimiter,
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
};
use http::{HeaderName, HeaderValue, header::InvalidHeaderValue};
#[cfg(feature = "http_client")]
pub use racal::reqwest::ApiClient;
#[cfg(feature = "http_client")]
use reqwest::{Client, RequestBuilder, header::HeaderMap};

#[cfg(feature = "http_client")]
use crate::query::NoAuthentication;
use crate::query::SavedLoginCredentials;

#[cfg(feature = "ws_client")]
mod ws;

/// Configuration for the API client
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct ApiConfiguration {
	/// The user agent of the API client
	pub user_agent: String,
	/// If the API client should indicate it has the mature access DLC enabled
	pub mature_content_enabled: bool,
	/// What platform the API client should indicate it's using (almost always
	/// should remain `pc_standalone`)
	pub platform: String,
	/// A comma separated string of compatible (API?) versions
	pub compatible_versions: String,
}

impl ApiConfiguration {
	/// Creates a new API client configuration
	#[must_use]
	pub fn new(user_agent: String) -> Self {
		Self {
			user_agent,
			mature_content_enabled: false,
			platform: "pc_standalone".to_string(),
			compatible_versions: "0,1,2".to_string(),
		}
	}

	fn to_headers(
		&self,
	) -> Result<Vec<(HeaderName, HeaderValue)>, InvalidHeaderValue> {
		Ok(vec![
			(http::header::USER_AGENT, HeaderValue::try_from(&self.user_agent)?),
			(
				"MatureContentDlc".parse().unwrap(),
				HeaderValue::try_from(self.mature_content_enabled.to_string())?,
			),
			("Platform".parse().unwrap(), HeaderValue::try_from(&self.platform)?),
			(
				"CompatibleVersions".parse().unwrap(),
				HeaderValue::try_from(&self.compatible_versions)?,
			),
		])
	}
}

impl SavedLoginCredentials {
	fn to_headers(
		&self,
	) -> Result<Vec<(HeaderName, HeaderValue)>, InvalidHeaderValue> {
		Ok(vec![
			("Username".parse().unwrap(), self.username.parse()?),
			("AccessKey".parse().unwrap(), self.access_key.parse()?),
		])
	}
}

/// An error that may happen with an API query
#[derive(Debug)]
pub enum ApiError {
	/// An error happened with serialization
	Serde(serde_json::Error),
	/// An error happened with the HTTPS request
	#[cfg(feature = "http_client")]
	Http(reqwest::Error),
	/// An error happened with the WS connection
	#[cfg(feature = "ws_client")]
	WebSocket(ezsockets::Error),
}

impl From<serde_json::Error> for ApiError {
	fn from(err: serde_json::Error) -> Self { Self::Serde(err) }
}

#[cfg(feature = "http_client")]
impl From<reqwest::Error> for ApiError {
	fn from(err: reqwest::Error) -> Self { Self::Http(err) }
}

#[cfg(feature = "http_client")]
impl From<racal::reqwest::ApiError> for ApiError {
	fn from(err: racal::reqwest::ApiError) -> Self {
		match err {
			racal::reqwest::ApiError::Reqwest(e) => Self::Http(e),
			racal::reqwest::ApiError::Serde(e) => Self::Serde(e),
		}
	}
}

#[cfg(feature = "ws_client")]
impl From<ezsockets::Error> for ApiError {
	fn from(err: ezsockets::Error) -> Self { Self::WebSocket(err) }
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
	config: ApiConfiguration,
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
	config: ApiConfiguration,
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
		config: &ApiConfiguration, auth: &SavedLoginCredentials,
	) -> Result<Client, ApiError> {
		use serde::ser::Error;

		let builder = Client::builder();
		let mut headers: Vec<(HeaderName, HeaderValue)> =
			config.to_headers().map_err(|e| {
				serde_json::Error::custom(
					"Couldn't parse config into headers: ".to_string() + &e.to_string(),
				)
			})?;
		headers.append(&mut auth.to_headers().map_err(|e| {
			serde_json::Error::custom(
				"Couldn't parse auth into headers: ".to_string() + &e.to_string(),
			)
		})?);

		let headers = HeaderMap::from_iter(headers);

		Ok(builder.default_headers(headers).build()?)
	}

	/// Removes authentication to the API client
	///
	/// # Errors
	///
	/// If deserializing user agent fails.# Panics
	///
	/// If there's an internal programming error, aka should never panic.ails
	pub fn new(
		config: ApiConfiguration, auth: impl Into<SavedLoginCredentials> + Send,
	) -> Result<Self, ApiError> {
		let auth = auth.into();
		Ok(Self {
			#[cfg(feature = "http_client")]
			http: Self::http_client(&config, &auth)?,
			#[cfg(feature = "http_client")]
			http_rate_limiter: http_rate_limiter(),
			#[cfg(feature = "ws_client")]
			ws: tokio::sync::RwLock::new(None),
			auth,
			config,
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
			if lock.is_some() {
				return Ok(());
			}
		}

		#[cfg(feature = "http_client")]
		self.http_rate_limiter.until_ready().await;
		let client = ws::Client::new(&self.config, &self.auth).await?;
		{
			let mut lock = self.ws.write().await;
			*lock = Some(client);
		}

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
		lock.is_some()
	}

	// Clippy seems to be wrong, or at least haven't been able to figure out how
	// this could be cleaned up more...
	#[allow(clippy::significant_drop_tightening)]
	/// Sends a WS message to the CVR API.
	///
	/// # Errors
	///
	/// If something with the request failed,
	/// or if the WS connection wasn't already open and creating it failed.
	///
	/// # Panics
	///
	/// If there's an internal programming error, aka should never panic.
	#[cfg(feature = "ws_client")]
	pub async fn send(
		&self,
		requestable: impl crate::query::Requestable + serde::ser::Serialize + Send,
	) -> Result<(), ApiError> {
		{
			let lock = self.ws.read().await;
			if let Some(ws_client) = &*lock {
				return ws_client.send(requestable);
			}
		}

		#[cfg(feature = "http_client")]
		self.http_rate_limiter.until_ready().await;
		let client = ws::Client::new(&self.config, &self.auth).await?;
		let mut lock = self.ws.write().await;
		*lock = Some(client);
		let lock = lock.downgrade();
		(*lock)
			.as_ref()
			.expect("client should exist as lock was never dropped")
			.send(requestable)
	}

	// Clippy seems to be wrong, or at least haven't been able to figure out how
	// this could be cleaned up more...
	#[allow(clippy::significant_drop_tightening)]
	/// Listens to events from the WS connection
	///
	/// # Errors
	///
	/// If creating the client fails,
	/// or if the WS connection wasn't already open and creating it failed.
	///
	/// # Panics
	///
	/// If there's an internal programming error, aka should never panic.
	#[cfg(feature = "ws_client")]
	pub async fn listen(&self) -> Result<ws::ReceiverContainer, ApiError> {
		{
			let lock = self.ws.read().await;
			if let Some(ws_client) = &*lock {
				return Ok(ws_client.listen());
			}
		}

		#[cfg(feature = "http_client")]
		self.http_rate_limiter.until_ready().await;
		let client = ws::Client::new(&self.config, &self.auth).await?;
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

	/// Removes the authentication from the API client
	///
	/// # Errors
	///
	/// If locking fails
	pub async fn downgrade(self) -> Result<UnauthenticatedCVR, ApiError> {
		{
			let mut lock = self.ws.write().await;
			*lock = None;
		}
		let http = UnauthenticatedCVR::http_client(&self.config.user_agent)?;
		Ok(UnauthenticatedCVR {
			config: self.config,
			http,
			http_rate_limiter: self.http_rate_limiter,
		})
	}
}

#[cfg(feature = "http_client")]
impl UnauthenticatedCVR {
	/// Creates an unauthenticated API client
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
			http: AuthenticatedCVR::http_client(&self.config, &auth)?,
			http_rate_limiter: self.http_rate_limiter,
			#[cfg(feature = "ws_client")]
			ws: tokio::sync::RwLock::new(None),
			auth,
			config: self.config,
		})
	}

	/// Creates a new CVR API client
	///
	/// # Errors
	///
	/// If deserializing user agent into a header fails,
	/// or if WS API is enabled & the connection establishment fails.
	pub fn new(config: ApiConfiguration) -> Result<Self, ApiError> {
		Ok(Self {
			http: Self::http_client(&config.user_agent)?,
			http_rate_limiter: http_rate_limiter(),
			config,
		})
	}
}
