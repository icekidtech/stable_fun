use std::env;
use std::str::FromStr;
use once_cell::sync::Lazy;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    transaction::Transaction,
};

static PROGRAM_ID: Lazy<Pubkey> = Lazy::new(|| {
    let program_id_str = env::var("PROGRAM_ID").expect("PROGRAM_ID must be set");
    Pubkey::from_str(&program_id_str).expect("Invalid PROGRAM_ID format")
});

pub async fn mint_stablecoins(amount: u64, wallet_address: &str) -> Result<(), String> {
    let rpc_client = RpcClient::new(utils::get_rpc_url());
    let keypair_path = env::var("KEYPAIR_PATH").expect("KEYPAIR_PATH must be set");
    let mint_keypair = read_keypair_file(&keypair_path).map_err(|e| e.to_string())?;

    let wallet_pubkey = Pubkey::from_str(wallet_address).map_err(|e| e.to_string())?;
    let mint_instruction = Instruction::new_with_borsh(
        program_id, // Replace with your program ID
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