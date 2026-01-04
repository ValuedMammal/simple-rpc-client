//! [`Client`].

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use bitcoin::{Address, Amount, Block, BlockHash, FeeRate, Transaction, Txid};

use corepc_client::bitcoin;
use corepc_client::client_sync::Error;
use corepc_client::types::model::{
    GetBlockHeaderVerbose, GetBlockVerboseOne, GetBlockchainInfo, MempoolEntry,
};
use corepc_client::types::v29;
use jsonrpc::Transport;
use jsonrpc::{serde, serde_json};
use serde_json::json;

use crate::types::{GetBlockFilter, ImportDescriptorsRequest, ImportDescriptorsResponse};

#[cfg(feature = "28_0")]
pub mod v28;

// RPC Client.
#[derive(Debug)]
pub struct Client {
    /// The inner JSON-RPC client.
    inner: jsonrpc::Client,
}

/// The way of authenticating to the JSON-RPC server.
#[derive(Debug, Clone)]
pub enum Auth {
    /// User and password
    UserPass(String, String),
    /// Path to cookie file
    CookieFile(PathBuf),
}

impl Auth {
    /// Get the user:pass credentials from this [`Auth`].
    fn get_user_pass(self) -> Result<(String, String), Error> {
        match self {
            Auth::UserPass(user, pass) => Ok((user, pass)),
            Auth::CookieFile(path) => {
                let line = BufReader::new(File::open(path)?)
                    .lines()
                    .next()
                    .ok_or(Error::InvalidCookieFile)??;
                let colon = line.find(':').ok_or(Error::InvalidCookieFile)?;

                Ok((line[..colon].to_string(), line[colon + 1..].to_string()))
            }
        }
    }
}

impl Client {
    /// Creates a `simple_http` client with `url` and `auth`.
    ///
    /// This can fail if we are unable to read the configured [`Auth::CookieFile`].
    pub fn new(url: &str, auth: Auth) -> Result<Self, Error> {
        let (user, pass) = auth.get_user_pass()?;
        Ok(Self::new_user_pass(url, user, Some(pass)))
    }

    /// Creates a `simple_http` client with `user` and `pass`.
    pub fn new_user_pass(url: &str, user: String, pass: Option<String>) -> Self {
        let transport = jsonrpc::simple_http::Builder::new()
            .url(url)
            .expect("URL check failed")
            .timeout(std::time::Duration::from_secs(60))
            .auth(user, pass)
            .build();

        Self {
            inner: jsonrpc::Client::with_transport(transport),
        }
    }

    /// Creates a `simple_http` client with `cookie` authentication.
    pub fn new_cookie_auth(url: &str, cookie: String) -> Self {
        let transport = jsonrpc::simple_http::Builder::new()
            .url(url)
            .expect("URL check failed")
            .timeout(std::time::Duration::from_secs(60))
            .cookie_auth(cookie)
            .build();

        Self {
            inner: jsonrpc::Client::with_transport(transport),
        }
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
        Ok(res.try_into().unwrap())
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

    /// Get block filter.
    pub fn get_block_filter(&self, hash: &BlockHash) -> Result<GetBlockFilter, Error> {
        use crate::types::GetBlockFilterResponse;
        let res: GetBlockFilterResponse = self.call("getblockfilter", &[json!(hash)])?;
        Ok(res.into_model().unwrap())
    }

    /// Get a serialized, hex-encoded raw [`Block`] by given `hash`.
    pub fn get_block_raw(&self, hash: &BlockHash) -> Result<String, Error> {
        use v29::GetBlockVerboseZero as GetBlock;
        let res: GetBlock = self.call("getblock", &[json!(hash), json!(0)])?;
        Ok(res.0)
    }

    /// Get a bitcoin [`Block`] by `hash`.
    pub fn get_block(&self, hash: &BlockHash) -> Result<Block, Error> {
        use v29::GetBlockVerboseZero as GetBlock;
        let res: GetBlock = self.call("getblock", &[json!(hash), json!(0)])?;
        Ok(res.block().unwrap())
    }

    /// Get raw mempool.
    pub fn get_raw_mempool(&self) -> Result<Vec<Txid>, Error> {
        let res: v29::GetRawMempool = self.call("getrawmempool", &[])?;
        Ok(res.into_model().unwrap().0)
    }

    /// Get raw mempool (verbose).
    pub fn get_raw_mempool_verbose(&self) -> Result<BTreeMap<Txid, MempoolEntry>, Error> {
        let res: v29::GetRawMempoolVerbose = self.call("getrawmempool", &[json!(true)])?;
        Ok(res.into_model().unwrap().0)
    }

    /// Send to address.
    pub fn send_to_address(&self, address: &Address, amount: Amount) -> Result<Txid, Error> {
        let res: v29::SendToAddress =
            self.call("sendtoaddress", &[json!(address), json!(amount.to_btc())])?;
        Ok(res.txid()?)
    }

    /// Get raw transaction.
    pub fn get_raw_transaction(&self, txid: &Txid) -> Result<Transaction, Error> {
        let res: v29::GetRawTransaction = self.call("getrawtransaction", &[json!(txid)])?;
        Ok(res.into_model().unwrap().0)
    }

    /// Import descriptors.
    pub fn import_descriptors(
        &self,
        requests: &[ImportDescriptorsRequest],
    ) -> Result<Vec<ImportDescriptorsResponse>, Error> {
        self.call("importdescriptors", &[json!(requests)])
    }
}

#[cfg(not(feature = "28_0"))]
impl Client {
    /// Get blockchain info.
    pub fn get_blockchain_info(&self) -> Result<GetBlockchainInfo, Error> {
        let res: v29::GetBlockchainInfo = self.call("getblockchaininfo", &[])?;
        Ok(res.into_model().unwrap())
    }

    /// Get block header (verbose).
    pub fn get_block_header_verbose(
        &self,
        hash: &BlockHash,
    ) -> Result<GetBlockHeaderVerbose, Error> {
        let res: v29::GetBlockHeaderVerbose = self.call("getblockheader", &[json!(hash)])?;
        Ok(res.into_model().unwrap())
    }

    /// Get block verbose.
    pub fn get_block_verbose(&self, hash: &BlockHash) -> Result<GetBlockVerboseOne, Error> {
        let res: v29::GetBlockVerboseOne = self.call("getblock", &[json!(hash), json!(1)])?;
        Ok(res.into_model().unwrap())
    }

    /// Get descriptor info.
    pub fn get_descriptor_info(&self, descriptor: &str) -> Result<v29::GetDescriptorInfo, Error> {
        self.call("getdescriptorinfo", &[json!(descriptor)])
    }
}
