//! `simple_https_client`.

mod client;
pub use client::*;

pub use corepc_client;
pub use jsonrpc;

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
