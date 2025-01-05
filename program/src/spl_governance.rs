// This is a file for Solana governance.

use spl_governance::{
    account::{Account, AccountInfo},
    instruction::{AccountKeys, Instruction},
    pubkey::Pubkey,
};

pub fn create_governance_account(public_key: Pubkey, lamports: u64) -> Account {
    Account::new(public_key, lamports, &[])
}

pub fn create_governance_instruction(program_id: Pubkey, accounts: AccountKeys, data: &[u8]) -> Instruction {
    Instruction::new(program_id, data, accounts)
}