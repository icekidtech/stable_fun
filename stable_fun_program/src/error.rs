use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum StablecoinError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Insufficient funds")]
    InsufficientFunds,
}

impl From<StablecoinError> for ProgramError {
    fn from(e: StablecoinError) -> Self {
        ProgramError::Custom(e as u32)
    }
}