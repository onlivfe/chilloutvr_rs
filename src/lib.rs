//! Typed models for [`ChilloutVR`](https://store.steampowered.com/app/661130/ChilloutVR/)'s API.
//!
//! There is no documentation for the API, so some things might be a bit off.
//!
//! The ABI team hasn't publicly stated that the API can be used,
//! so it'd be better to ask them before actually using this.
//! You've been warned, so don't complain if you get punished for it.

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]

// ...Yes, mixed `/1/` and `/v1/`....
// ABI I love you, but plz, this is next level sinning with your API

/// The main API endpoint
pub const API_V1_HTTP_URL: &str = "https://api.abinteractive.net/1";
/// The API host name
pub const API_V1_HOSTNAME: &str = "api.abinteractive.net";
/// The WS API endpoint
pub const API_V1_WS_URL: &str = "wss://api.abinteractive.net/1/users/ws";
// The gateway endpoint... Website/old API probably?
//pub const API_V1_GAME_DATA: &str = "https://gateway.abi.network/v1/IGameData";

// TODO: Complete docs
#[allow(missing_docs)]
pub mod model;
pub mod query;

#[cfg(any(feature = "http_client", feature = "ws_client"))]
pub mod api_client;
