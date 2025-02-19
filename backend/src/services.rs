use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    transaction::Transaction,
    signer::keypair::read_keypair_file, // Import the read_keypair_file function
};
use std::env;
use std::str::FromStr;
use once_cell::sync::Lazy;
use program::instruction::StablecoinInstruction; // Import the StablecoinInstruction type
use solana_sdk::instruction::AccountMeta; // Import the AccountMeta type
use solana_sdk::signature::Signer; // Import the Signer trait
use crate::config; // Import the config module

static PROGRAM_ID: Lazy<Pubkey> = Lazy::new(|| {
    let program_id_str = env::var("PROGRAM_ID").expect("PROGRAM_ID must be set");
    Pubkey::from_str(&program_id_str).expect("Invalid PROGRAM_ID format")
});

pub async fn mint_stablecoins(amount: u64, wallet_address: &str) -> Result<(), String> {
    let config = config::load_config();
    let rpc_client = RpcClient::new(config.rpc_url);
    let keypair_path = env::var("KEYPAIR_PATH").expect("KEYPAIR_PATH must be set");
    let mint_keypair = read_keypair_file(&keypair_path).map_err(|e| e.to_string())?;

    let wallet_pubkey = Pubkey::from_str(wallet_address).map_err(|e| e.to_string())?;
    let mint_instruction = Instruction::new_with_borsh(
        *PROGRAM_ID, // Use the program ID
        &StablecoinInstruction::Mint { amount },
        vec![
            AccountMeta::new(wallet_pubkey, true),
            AccountMeta::new(mint_keypair.pubkey(), false),
        ],
    );

    let transaction = Transaction::new_signed_with_payer(
        &[mint_instruction],
        Some(&wallet_pubkey),
        &[&mint_keypair],
        rpc_client.get_latest_blockhash().await.map_err(|e| e.to_string())?,
    );

    rpc_client.send_and_confirm_transaction(&transaction).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn redeem_stablecoins(amount: u64, wallet_address: &str) -> Result<(), String> {
    let config = config::load_config();
    let rpc_client = RpcClient::new(config.rpc_url);
    let keypair_path = env::var("KEYPAIR_PATH").expect("KEYPAIR_PATH must be set");
    let mint_keypair = read_keypair_file(&keypair_path).map_err(|e| e.to_string())?;

    let wallet_pubkey = Pubkey::from_str(wallet_address).map_err(|e| e.to_string())?;
    let redeem_instruction = Instruction::new_with_borsh(
        *PROGRAM_ID, // Use the program ID
        &StablecoinInstruction::Redeem { amount },
        vec![
            AccountMeta::new(wallet_pubkey, true),
            AccountMeta::new(mint_keypair.pubkey(), false),
        ],
    );

    let transaction = Transaction::new_signed_with_payer(
        &[redeem_instruction],
        Some(&wallet_pubkey),
        &[&mint_keypair],
        rpc_client.get_latest_blockhash().await.map_err(|e| e.to_string())?,
    );

    rpc_client.send_and_confirm_transaction(&transaction).await.map_err(|e| e.to_string())?;
    Ok(())
}