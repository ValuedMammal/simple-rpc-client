//! [`Client`].

use bitcoin::{Address, Amount, Transaction, Txid};

use corepc_client::bitcoin;
use crate::Error;
use corepc_client::bitcoin::{Block, BlockHash};
use corepc_client::types::model;
use corepc_client::types::v29::{
    GetBlockFilter, GetBlockHeaderVerbose, GetBlockVerboseOne, GetBlockVerboseZero, GetRawMempool,
    GetRawTransaction, SendToAddress,
};
use jsonrpc::Transport;
use jsonrpc::{serde, serde_json};
use serde_json::json;

// RPC Client.
#[derive(Debug)]
pub struct Client {
    /// The inner JSON-RPC client.
    inner: jsonrpc::Client,
}

impl Client {
    /// Creates a `minreq` HTTP client with `user` and `pass`.
    pub fn new_user_pass(url: &str, user: String, pass: Option<String>) -> Result<Self, Error> {
        let transport = jsonrpc::http::minreq_http::Builder::new()
            .url(url)
            .map_err(|e| Error::InvalidResponse(format!("Invalid URL: {e}")))?
            .timeout(std::time::Duration::from_secs(60))
            .basic_auth(user, pass)
            .build();

        Ok(Self {
            inner: jsonrpc::Client::with_transport(transport),
        })
    }

    /// Creates a `minreq` HTTP client with `cookie` authentication.
    pub fn new_cookie_auth(url: &str, cookie: String) -> Result<Self, Error> {
        let transport = jsonrpc::http::minreq_http::Builder::new()
            .url(url)
            .map_err(|e| Error::InvalidResponse(format!("Invalid URL: {e}")))?
            .timeout(std::time::Duration::from_secs(60))
            .cookie_auth(cookie)
            .build();

        Ok(Self {
            inner: jsonrpc::Client::with_transport(transport),
        })
    }

    /// Creates a client to a bitcoind JSON-RPC server with transport.
    pub fn with_transport<T>(transport: T) -> Self
    where
        T: Transport,
    {
        Self {
            inner: jsonrpc::Client::with_transport(transport),
        }
    }

    /// Calls the RPC `method` with a given `args` list.
    pub fn call<T>(&self, method: &str, args: &[serde_json::Value]) -> Result<T, Error>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let raw = serde_json::value::to_raw_value(args)?;
        let request = self.inner.build_request(method, Some(&*raw));
        let resp = self.inner.send_request(request)?;

        Ok(resp.result()?)
    }
}

// `bitcoind` RPC methods
impl Client {
    /// Get block count.
    pub fn get_block_count(&self) -> Result<u32, Error> {
        let res: i32 = self.call("getblockcount", &[])?;
        Ok(res.try_into()?)
    }

    /// Get best block hash.
    pub fn get_best_block_hash(&self) -> Result<BlockHash, Error> {
        let res: String = self.call("getbestblockhash", &[])?;
        Ok(res.parse()?)
    }

    /// Get block hash by `height`.
    pub fn get_block_hash(&self, height: u32) -> Result<BlockHash, Error> {
        let res: String = self.call("getblockhash", &[json!(height)])?;
        Ok(res.parse()?)
    }

    /// Get block header (verbose).
    pub fn get_block_header_verbose(
        &self,
        hash: &BlockHash,
    ) -> Result<model::GetBlockHeaderVerbose, Error> {
        let res: GetBlockHeaderVerbose = self.call("getblockheader", &[json!(hash)])?;
        Ok(res.into_model()?)
    }

    /// Get block filter.
    pub fn get_block_filter(&self, hash: &BlockHash) -> Result<model::GetBlockFilter, Error> {
        let res: GetBlockFilter = self.call("getblockfilter", &[json!(hash)])?;
        Ok(res.into_model()?)
    }

    /// Get block.
    pub fn get_block(&self, hash: &BlockHash) -> Result<Block, Error> {
        let res: GetBlockVerboseZero = self.call("getblock", &[json!(hash), json!(0)])?;
        Ok(res.block()?)
    }

    /// Get block verbose.
    pub fn get_block_verbose(&self, hash: &BlockHash) -> Result<model::GetBlockVerboseOne, Error> {
        let res: GetBlockVerboseOne = self.call("getblock", &[json!(hash), json!(1)])?;
        Ok(res.into_model()?)
    }

    /// Get raw mempool.
    pub fn get_raw_mempool(&self) -> Result<Vec<Txid>, Error> {
        let res: GetRawMempool = self.call("getrawmempool", &[])?;
        Ok(res.into_model()?.0)
    }

    /// Send to address.
    pub fn send_to_address(&self, address: &Address, amount: Amount) -> Result<Txid, Error> {
        let res: SendToAddress =
            self.call("sendtoaddress", &[json!(address), json!(amount.to_btc())])?;
        Ok(res.txid()?)
    }

    /// Get raw transaction.
    pub fn get_raw_transaction(&self, txid: &Txid) -> Result<Transaction, Error> {
        let res: GetRawTransaction = self.call("getrawtransaction", &[json!(txid)])?;
        Ok(res.into_model()?.0)
    }
}
