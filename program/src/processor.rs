use crate::{error::StablecoinError, instruction::StablecoinInstruction};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StablecoinInstruction::try_from_slice(instruction_data)
        .map_err(|_| StablecoinError::InvalidInstruction)?;

    match instruction {
        StablecoinInstruction::InitializeToken { name, symbol } => {
            // Logic for initializing a token
        }
        StablecoinInstruction::Mint { amount } => {
            // Logic for minting tokens
        }
        StablecoinInstruction::Redeem { amount } => {
            // Logic for redeeming tokens
        }
    }
    Ok(())
}