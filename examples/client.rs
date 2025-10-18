use std::str::FromStr;

use simple_rpc_client::Client;

/// RPC url.
const URL: &str = "http://127.0.0.1:38332";
const COOKIE_FILE: &str = ".bitcoin/signet/.cookie";

fn main() {
    let cookie_file = std::env::var("RPC_COOKIE").unwrap_or(COOKIE_FILE.to_string());
    let path = std::path::PathBuf::from_str(&cookie_file).unwrap();
    let cookie = std::fs::read_to_string(path).unwrap();

    let client = Client::new_cookie_auth(URL, cookie);
    let res = client.get_best_block_hash().unwrap();

    println!("{:#?}", res);
}
