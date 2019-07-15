// Copyright (c) The XPeer Core Contributors
// SPDX-License-Identifier: Apache-2.0

//! This module defines error types used by [`XPeerDB`](crate::XPeerDB).

use failure::Fail;

/// This enum defines errors commonly used among [`XPeerDB`](crate::XPeerDB) APIs.
#[derive(Debug, Fail)]
pub enum XPeerDbError {
    /// A requested item is not found.
    #[fail(display = "{} not found.", _0)]
    NotFound(String),
    /// Requested too many items.
    #[fail(display = "Too many items requested: {}, max is {}", _0, _1)]
    TooManyRequested(u64, u64),
}
