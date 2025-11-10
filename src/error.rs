//! Error enum for the RPC client.

use std::fmt;

use corepc_client::types::v29::{GetBlockFilterError, GetBlockHeaderVerboseError};
use jsonrpc::serde_json;

#[derive(Debug)]
pub enum Error {
    /// Bitcoin address/hash parsing error.
    BitcoinHex(corepc_client::bitcoin::hex::HexToArrayError),

    /// From hex conversion error
    FromHex(corepc_client::bitcoin::consensus::encode::FromHexError),

    /// JSON-RPC transport or protocol error
    JsonRpc(jsonrpc::Error),

    /// GetBlockHeaderVerboseError
    CorepcHeaderVerbose(GetBlockHeaderVerboseError),

    /// Get Block Filter Error
    CorepcFilter(GetBlockFilterError),

    /// Invalid response type
    InvalidResponse(String),

    /// JSON serialization/deserialization error.
    Json(serde_json::Error),

    /// Integer conversion error.
    IntConversion(std::num::TryFromIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BitcoinHex(e) => write!(f, "Parsing error: {e}"),
            Error::FromHex(e) => write!(f, "From hex conversion error: {e}"),
            Error::JsonRpc(e) => write!(f, "JSON-RPC error: {e}"),
            Error::CorepcHeaderVerbose(e) => write!(f, "Block Header Verbose error: {e}"),
            Error::CorepcFilter(e) => write!(f, "Block Filter error: {e}"),
            Error::InvalidResponse(msg) => write!(f, "Invalid response: {msg}"),
            Error::Json(e) => write!(f, "Json error: {e}"),
            Error::IntConversion(e) => write!(f, "Integer conversion error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::BitcoinHex(e) => Some(e),
            Error::FromHex(e) => Some(e),
            Error::JsonRpc(e) => Some(e),
            Error::CorepcHeaderVerbose(e) => Some(e),
            Error::CorepcFilter(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::IntConversion(e) => Some(e),
            _ => None,
        }
    }
}

impl From<jsonrpc::Error> for Error {
    fn from(e: jsonrpc::Error) -> Self {
        Error::JsonRpc(e)
    }
}

impl From<corepc_client::bitcoin::hex::HexToArrayError> for Error {
    fn from(e: corepc_client::bitcoin::hex::HexToArrayError) -> Self {
        Error::BitcoinHex(e)
    }
}

impl From<corepc_client::bitcoin::consensus::encode::FromHexError> for Error {
    fn from(e: corepc_client::bitcoin::consensus::encode::FromHexError) -> Self {
        Error::FromHex(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

impl From<GetBlockHeaderVerboseError> for Error {
    fn from(e: GetBlockHeaderVerboseError) -> Self {
        Error::CorepcHeaderVerbose(e)
    }
}

impl From<GetBlockFilterError> for Error {
    fn from(e: GetBlockFilterError) -> Self {
        Error::CorepcFilter(e)
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(e: std::num::TryFromIntError) -> Self {
        Error::IntConversion(e)
    }
}
