use corepc_client::bitcoin;
use std::str::FromStr;

// Setup RPC client.
lazy_static::lazy_static! {
    static ref CLIENT: simplerpc::Client = {
        let url = std::env::var("RPC_URL").expect("missing RPC_URL");
        let cookie_file = std::env::var("RPC_COOKIE").expect("missing RPC_COOKIE");
        let auth = simplerpc::Auth::CookieFile(cookie_file.into());
        simplerpc::Client::new(&url, auth).expect("failed to create RPC Client")
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

#[test]
fn test_get_descriptor_info() {
    let desc_str = "tr(0a1975ec57d1dafe78c6941e56cbc6b3972a12c2339d2c199ba88a32d290bdbb)#asff8576";
    let res = CLIENT.get_descriptor_info(desc_str).unwrap();
    dbg!(res);
}

#[test]
fn test_import_descriptors() {
    use simplerpc::types::ImportDescriptorsRequest;
    let request = ImportDescriptorsRequest {
        desc: "tr(cUZeQ1QvqskvASz81VeyRRrs7YfGfQbMqtyU55efWegJzkP6hpTP)#jqscf30c".to_string(),
        timestamp: std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
        ..Default::default()
    };
    let resp = &CLIENT.import_descriptors(&[request]).unwrap()[0];
    dbg!(resp);
    assert!(resp.success);
}
