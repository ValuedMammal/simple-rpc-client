use corepc_client::bitcoin;
use std::str::FromStr;

// Setup RPC client.
lazy_static::lazy_static! {
    static ref CLIENT: simple_rpc_client::Client = {
        let url = std::env::var("RPC_URL").unwrap();
        let path = std::env::var("RPC_COOKIE").unwrap();
        let auth = std::fs::read_to_string(path).unwrap();
        let transport = jsonrpc::http::simple_http::Builder::new()
            .url(&url)
            .unwrap()
            .cookie_auth(auth)
            .build();

        simple_rpc_client::Client::with_transport(transport)
    };
}

#[test]
fn test_get_block_count() {
    let res = CLIENT.get_block_count().unwrap();
    dbg!(res);
}

#[test]
fn test_get_block_hash() {
    let res = CLIENT.get_block_hash(0).unwrap();
    dbg!(res);
}

#[test]
fn test_get_best_block_hash() {
    let res = CLIENT.get_best_block_hash().unwrap();
    dbg!(res);
}

#[test]
fn test_get_block_header() {
    let hash = CLIENT.get_best_block_hash().unwrap();
    let res = CLIENT.get_block_header_verbose(&hash).unwrap();
    dbg!(res);
}

#[test]
fn test_get_block_filter() {
    let hash = CLIENT.get_best_block_hash().unwrap();
    let res = CLIENT.get_block_filter(&hash).unwrap();
    dbg!(res);
}

#[test]
fn test_get_block() {
    let hash = CLIENT.get_best_block_hash().unwrap();
    let res = CLIENT.get_block(&hash).unwrap();
    dbg!(res);
}

#[test]
fn test_get_block_verbose() {
    let hash = CLIENT.get_best_block_hash().unwrap();
    let res = CLIENT.get_block_verbose(&hash).unwrap();
    dbg!(res);
}

#[test]
fn test_send_to_address() {
    use bitcoin::{Address, Amount};
    let addr = Address::from_str("bcrt1qrvyf5r90k9ky7ennhp3grtus22q0zvzx8qdsad")
        .unwrap()
        .assume_checked();
    let res = CLIENT.send_to_address(&addr, Amount::ONE_BTC).unwrap();
    dbg!(res);
}

#[test]
fn test_get_raw_mempool() {
    let res = CLIENT.get_raw_mempool().unwrap();
    dbg!(&res);
}

#[test]
fn test_get_raw_transaction() {
    let hash = CLIENT.get_best_block_hash().unwrap();
    let block = CLIENT.get_block(&hash).unwrap();
    let txid = block.txdata.first().unwrap().compute_txid();
    let res = CLIENT.get_raw_transaction(&txid).unwrap();
    dbg!(res);
}
