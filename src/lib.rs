//! `simple_rpc_client`.

mod client;
pub use client::*;
mod error;
pub use error::Error;

pub use corepc_client;
pub use jsonrpc;
