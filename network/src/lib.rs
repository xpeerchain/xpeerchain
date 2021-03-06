// Copyright (c) The XPeer Core Contributors
// SPDX-License-Identifier: Apache-2.0

// The nightly features that are commonly needed with async / await
// Lets turn these on so that we can experiment a little bit
#![feature(async_await)]
// <Black magic>
// Increase recursion limit to allow for use of select! macro.
#![recursion_limit = "1024"]
// </Black magic>

// Public exports
pub use common::NetworkPublicKeys;
pub use interface::NetworkProvider;

pub mod interface;
pub mod proto;
pub mod protocols;
pub mod validator_network;

mod common;
mod connectivity_manager;
mod counters;
mod error;
mod peer_manager;
mod sink;
mod transport;
mod utils;

/// Type for unique identifier associated with each network protocol
pub type ProtocolId = bytes::Bytes;
