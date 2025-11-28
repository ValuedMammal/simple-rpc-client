//! Client methods which support Bitcoin Core version 28.0.

use bitcoin::BlockHash;
use corepc_client::bitcoin;
use corepc_client::client_sync::Error;
use corepc_client::types::model::{GetBlockHeaderVerbose, GetBlockVerboseOne};
use corepc_client::types::v28;

use super::Client;
use super::json;

impl Client {
    /// Get block header (verbose).
    pub fn get_block_header_verbose(
        &self,
        hash: &BlockHash,
    ) -> Result<GetBlockHeaderVerbose, Error> {
        let res: v28::GetBlockHeaderVerbose = self.call("getblockheader", &[json!(hash)])?;
        Ok(res.into_model().unwrap())
    }

    /// Get block (verbose).
    pub fn get_block_verbose(&self, hash: &BlockHash) -> Result<GetBlockVerboseOne, Error> {
        let res: v28::GetBlockVerboseOne = self.call("getblock", &[json!(hash), json!(1)])?;
        Ok(res.into_model().unwrap())
    }
}
