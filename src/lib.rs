//! [`ChilloutVR`](https://store.steampowered.com/app/661130/ChilloutVR/)'s API models in rust.
//!
//! This is fully unofficial and in no way affiliated, endorsed, supported,
//! or created by Alpha Blend Interactive, the creators of `ChilloutVR`.
//!
//! The crate has models of the responses, with proper serde support.
//! It also definitions for the requests, using [`racal`](https://docs.rs/racal/latest/racal/) for the HTTP parts,
//! and big request/response enums for `WebSocket`s,
//! meaning that there's no lock-in to a single API client.
//! An example API client is provided for convenience though using [`reqwest`](https://crates.io/crates/reqwest).
//!
//! The API technically isn't public yet, so proceed with your own discretion.
//! That also means there is no official API documentation.
//! Which means it's possible that some things are wrong
//! and/or will change a lot in the future.

#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![deny(clippy::cargo)]
#![warn(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]
// Not much can be done about it :/
#![allow(clippy::multiple_crate_versions)]

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

pub mod id;
pub mod model;
pub mod query;

#[cfg(any(feature = "http_client", feature = "ws_client"))]
pub mod api_client;
