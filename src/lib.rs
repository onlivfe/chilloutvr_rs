//! Typed model predictions for [`ChilloutVR`](https://store.steampowered.com/app/661130/ChilloutVR/)'s upcoming API.
//!
//! As of writing this, the API shouldn't be used yet by third party clients.
//! The purpose of this library is to have models to base client applications on
//! before the API gets more stable & public.
//!
//! Please contact someone from the ABI team & ask for their permission before
//! using the API, as it seems to be internal only as of writing this and very
//! "don't touch". I've not tried what happens if you do contact the API, so
//! don't complaining if you get punished for touching it.

#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(clippy::all)]
#![deny(unsafe_code)]
#![deny(clippy::cargo)]
//#![warn(missing_docs)]
#![deny(rustdoc::invalid_html_tags)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// My project my choice, tabs are literally made for indentation, spaces not.
#![allow(clippy::tabs_in_doc_comments)]

// ...Yes, mixed `/1/` and `/v1/`....
// ABI I love you, but plz, this is next level sinning with your API
pub const API_V1_HTTP_URL: &str = "https://api.abinteractive.net/1";
pub const API_V1_WS_URL: &str = "https://api.abinteractive.net/1/users/ws";
pub const API_V1_GAME_DATA: &str = "https://gateway.abi.network/v1/IGameData";

pub mod model;
pub mod query;
pub mod ws;

#[cfg(feature = "api_client")]
#[cfg_attr(nightly, doc(cfg(feature = "api_client")))]
pub mod api_client;
