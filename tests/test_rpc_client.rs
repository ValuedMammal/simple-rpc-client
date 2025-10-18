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
    let res = CLIENT.get_best_block_hash().unwrap();
    dbg!(res);
}
