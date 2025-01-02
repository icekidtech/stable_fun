pub mod instruction;
pub mod processor;
pub mod state;
pub mod error;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

// Define the entrypoint to the Solana program
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process(program_id, accounts, instruction_data)
}