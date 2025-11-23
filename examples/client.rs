/// RPC url.
const URL: &str = "http://127.0.0.1:38332";
/// Path to bitcoind cookie file.
const COOKIE_FILE: &str = ".bitcoin/signet/.cookie";

fn main() -> anyhow::Result<()> {
    let cookie_file = std::env::var("RPC_COOKIE").unwrap_or(COOKIE_FILE.to_string());
    let client = simplerpc::Client::new(URL, simplerpc::Auth::CookieFile(cookie_file.into()))?;

    let res = client.get_best_block_hash()?;
    println!("{:#?}", res);

    Ok(())
}
