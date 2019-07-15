// Copyright (c) The XPeer Core Contributors
// SPDX-License-Identifier: Apache-2.0

use failure;
use xpeer_crypto::hkdf::HkdfError;
use std::{convert, error::Error, fmt, io};

/// We define our own Result type in order to not have to import the xpeer/common/failture_ext
pub type Result<T> = ::std::result::Result<T, WalletError>;

/// XPeer Wallet Error is a convenience enum for generating arbitarary WalletErrors. Currently, only
/// the XPeerWalletGeneric error is being used, but there are plans to add more specific errors as
/// XPeerWallet matures
pub enum WalletError {
    /// generic error message
    XPeerWalletGeneric(String),
}

impl Error for WalletError {
    fn description(&self) -> &str {
        match *self {
            WalletError::XPeerWalletGeneric(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            WalletError::XPeerWalletGeneric(_) => None,
        }
    }
}

impl fmt::Display for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WalletError::XPeerWalletGeneric(ref s) => write!(f, "XPeerWalletGeneric: {}", s),
        }
    }
}

impl fmt::Debug for WalletError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}

impl convert::From<WalletError> for io::Error {
    fn from(_err: WalletError) -> io::Error {
        match _err {
            WalletError::XPeerWalletGeneric(s) => io::Error::new(io::ErrorKind::Other, s),
        }
    }
}

impl convert::From<io::Error> for WalletError {
    fn from(err: io::Error) -> WalletError {
        WalletError::XPeerWalletGeneric(err.description().to_string())
    }
}

impl convert::From<failure::prelude::Error> for WalletError {
    fn from(err: failure::prelude::Error) -> WalletError {
        WalletError::XPeerWalletGeneric(format!("{}", err))
    }
}

impl convert::From<protobuf::error::ProtobufError> for WalletError {
    fn from(err: protobuf::error::ProtobufError) -> WalletError {
        WalletError::XPeerWalletGeneric(err.description().to_string())
    }
}

impl convert::From<ed25519_dalek::SignatureError> for WalletError {
    fn from(err: ed25519_dalek::SignatureError) -> WalletError {
        WalletError::XPeerWalletGeneric(format!("{}", err))
    }
}

impl convert::From<HkdfError> for WalletError {
    fn from(err: HkdfError) -> WalletError {
        WalletError::XPeerWalletGeneric(format!("{}", err))
    }
}
