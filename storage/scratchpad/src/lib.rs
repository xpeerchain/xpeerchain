// Copyright (c) The XPeer Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! This crate provides in-memory representation of XPeer core data structures used by the executor.

mod accumulator;
mod sparse_merkle;

pub use crate::{
    accumulator::Accumulator,
    sparse_merkle::{AccountState, ProofRead, SparseMerkleTree},
};
