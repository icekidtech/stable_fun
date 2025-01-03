use dotenv::dotenv;
use std::env;

pub fn get_rpc_url() -> String {
    dotenv().ok();
    env::var("RPC_URL").unwrap_or_else(|_| "https://api.testnet.solana.com".to_string())
}