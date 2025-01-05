use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

pub fn create_account(public_key: Pubkey, lamports: u64) -> Account {
    Account::new(lamports, 0, &system_program::id())
}

pub fn create_mint_instruction(
    program_id: Pubkey,
    wallet_pubkey: Pubkey,
    mint_pubkey: Pubkey,
    amount: u64,
) -> Instruction {
    Instruction::new_with_borsh(
        program_id,
 StablecoinInstruction::Mint { amount },
        vec![
            AccountMeta::new(wallet_pubkey, false),
            AccountMeta::new(mint_pubkey, false),
        ],
    )
}