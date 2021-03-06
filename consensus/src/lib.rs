// Copyright (c) The XPeer Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! Consensus for the XPeer Core blockchain
//!
//! Encapsulates public consensus traits and any implementations of those traits.
//! Currently, the only consensus protocol supported is XPeerBFT (based on
//! [HotStuff](https://arxiv.org/pdf/1803.05069.pdf)).

#![deny(missing_docs)]
#![feature(async_await, slice_patterns)]
#![feature(drain_filter)]
#![feature(checked_duration_since)]
#![feature(crate_visibility_modifier)]
#![recursion_limit = "128"]
#[macro_use]
extern crate failure;

mod chained_bft;
mod util;

/// Defines the public consensus provider traits to implement for
/// use in the XPeer Core blockchain.
pub mod consensus_provider;

mod counters;

mod state_computer;
mod state_replication;
mod state_synchronizer;
mod txn_manager;
