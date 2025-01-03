use dotenv::dotenv;
use std::env;

pub struct Config {
    pub server_address: String,
    pub rpc_url: String,
}

pub fn load_config() -> Config {
    dotenv().ok();
    Config {
        server_address: env::var("SERVER_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
        rpc_url: env::var("RPC_URL").unwrap_or_else(|_| "https://api.testnet.solana.com".to_string()),
    }
}