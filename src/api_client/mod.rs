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

#[cfg(feature = "http_client")]
use governor::{
	clock::DefaultClock,
	middleware::NoOpMiddleware,
	state::{InMemoryState, NotKeyed},
	Quota,
	RateLimiter,
};
#[cfg(feature = "http_client")]
use racal::{Queryable, RequestMethod};
#[cfg(feature = "http_client")]
use reqwest::{header::HeaderMap, Client};
use serde::{de::DeserializeOwned, ser::Serialize};
#[cfg(feature = "http_client")]
use std::num::NonZeroU32;
#[cfg(feature = "ws_client")]
use tokio::net::TcpStream;
#[cfg(feature = "ws_client")]
use tokio::sync::RwLock;

#[cfg(feature = "ws_client")]
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::query::{CvrApiUnwrapping, NoAuthentication, SavedLoginCredentials};

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
	fn from(err: serde_json::Error) -> Self {
		Self::Serde(err)
	}
}

#[cfg(feature = "http_client")]
impl From<reqwest::Error> for ApiError {
	fn from(err: reqwest::Error) -> Self {
		Self::Reqwest(err)
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
#[cfg(feature = "ws_client")]
type WsClient = RwLock<WebSocketStream<MaybeTlsStream<TcpStream>>>;

/// The main API client
#[derive(Debug)]
pub struct CVR {
	#[cfg(feature = "http_client")]
	http: Client,
	#[cfg(feature = "http_client")]
	http_rate_limiter: NormalRateLimiter,
	#[cfg(feature = "ws_client")]
	ws: WsClient,
}

#[doc(hidden)]
#[cfg(feature = "http_client")]
pub enum SupportedApiStates {
	Unauthenticated,
	Authenticated(SavedLoginCredentials),
}

#[cfg(feature = "http_client")]
impl From<NoAuthentication> for SupportedApiStates {
	fn from(_: NoAuthentication) -> Self {
		Self::Unauthenticated
	}
}

#[cfg(feature = "http_client")]
impl From<SavedLoginCredentials> for SupportedApiStates {
	fn from(auth: SavedLoginCredentials) -> Self {
		Self::Authenticated(auth)
	}
}

impl CVR {
	/// Creates an API client
	#[cfg(feature = "http_client")]
	fn http_client(
		user_agent: &str,
		auth: &Option<SavedLoginCredentials>,
	) -> Result<Client, ApiError> {
		use serde::ser::Error;

		let builder = Client::builder();
		let mut headers = HeaderMap::new();

		if let Some(auth) = auth {
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
		}

		Ok(builder.user_agent(user_agent).default_headers(headers).build()?)
	}

	/// Creates the rate limiter for the HTTP API
	#[cfg(feature = "http_client")]
	#[must_use]
	fn http_rate_limiter() -> NormalRateLimiter {
		RateLimiter::direct(
			Quota::per_minute(NonZeroU32::try_from(12).unwrap())
				.allow_burst(NonZeroU32::try_from(5).unwrap()),
		)
	}

	/// Creates an API client
	#[cfg(feature = "ws_client")]
	async fn ws_client(
		user_agent: &str,
		auth: &Option<SavedLoginCredentials>,
	) -> Result<WsClient, ApiError> {
		use serde::ser::Error;

		let mut request: http::Request<()> =
			http::Request::connect(crate::API_V1_WS_URL).body(()).unwrap();
		request.headers_mut().insert(
			"User-Agent",
			user_agent.parse().map_err(|_| {
				serde_json::Error::custom("Couldn't turn user_agent a header")
			})?,
		);
		if let Some(auth) = auth {
			request.headers_mut().insert(
				"Username",
				auth.username.parse().map_err(|_| {
					serde_json::Error::custom("Couldn't turn username into a header")
				})?,
			);
			request.headers_mut().insert(
				"AccessKey",
				auth.access_key.parse().map_err(|_| {
					serde_json::Error::custom("Couldn't turn access_key into a header")
				})?,
			);
		}

		let (client, _) =
			tokio_tungstenite::connect_async_tls_with_config(request, None, None).await?;

		Ok(RwLock::new(client))
	}

	/// Creates a new CVR API client
	///
	/// # Errors
	///
	/// If deserializing user agent into a header fails,
	/// or if WS API is enabled & the connection establishment fails.
	pub async fn new(
		user_agent: &str,
		auth: impl Into<Option<SavedLoginCredentials>> + Send,
	) -> Result<Self, ApiError> {
		let auth = auth.into();
		Ok(Self {
			#[cfg(feature = "http_client")]
			http: Self::http_client(user_agent, &auth)?,
			// ~5 seconds per request sustained over one minute, allowing up to a request
			// per second in bursts.
			#[cfg(feature = "http_client")]
			http_rate_limiter: Self::http_rate_limiter(),
			#[cfg(feature = "ws_client")]
			ws: Self::ws_client(user_agent, &auth).await?,
		})
	}

	/// Sends a query to the CVR API
	///
	/// Also automatically unwraps the data field, discarding the message.
	/// Use [`cvr.query_without_unwrapping`](Self::query_without_unwrapping) if
	/// you want to access the message field too.
	///
	/// # Errors
	///
	/// If something with the request failed.
	#[cfg(feature = "http_client")]
	pub async fn query<ReturnType, WrappedType, FromState, T>(
		&self,
		queryable: T,
	) -> Result<ReturnType, ApiError>
	where
		WrappedType: CvrApiUnwrapping<ReturnType> + DeserializeOwned,
		FromState: Into<SupportedApiStates>,
		T: Queryable<FromState, WrappedType> + Send + Sync,
	{
		Ok(self.query_without_unwrapping(queryable).await?.unwrap_data())
	}

	/// Sends a query to the CVR API
	///
	/// # Errors
	///
	/// If something with the request failed.
	#[cfg(feature = "http_client")]
	pub async fn query_without_unwrapping<R, FromState, T>(
		&self,
		queryable: T,
	) -> Result<R, ApiError>
	where
		R: DeserializeOwned,
		FromState: Into<SupportedApiStates>,
		T: Queryable<FromState, R> + Send + Sync,
	{
		let mut request = self.http.request(
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

		self.http_rate_limiter.until_ready().await;
		let response = request.send().await?.error_for_status()?;
		// TODO: Figure out if there are any extra rate limit headers to respect

		#[cfg(feature = "debug")]
		{
			let text = response.text().await?;
			dbg!(&text);
			Ok(serde_json::from_str::<R>(&text)?)
		}
		#[cfg(not(feature = "debug"))]
		{
			Ok(response.json::<R>().await?)
		}
	}

	/// Sends a WS message to the CVR API.
	///
	/// # Errors
	///
	/// If something with the request failed.
	#[cfg(feature = "ws_client")]
	pub async fn send(
		&self,
		requestable: impl crate::query::Requestable + Serialize + Send,
	) -> Result<(), ApiError> {
		use futures_util::SinkExt;

		let data = crate::query::RequestWrapper {
			request_type: requestable.request_type(),
			data: requestable,
		};
		let data = serde_json::to_vec(&data)?;
		let data = tokio_tungstenite::tungstenite::Message::binary(data);

		self.ws.write().await.send(data).await?;

		Ok(())
	}

	// TODO: listening to websocket requests
}
