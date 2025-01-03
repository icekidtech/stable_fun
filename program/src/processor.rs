use crate::state::TokenState;
use crate::instruction::StablecoinInstruction; // Import the StablecoinInstruction enum
use crate::error::StablecoinError; // Import the StablecoinError enum
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    log::sol_log,
    program_error::ProgramError,
};

// The main processor function for handling all instructions
pub fn process(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize the instruction data to StablecoinInstruction enum
    let instruction = StablecoinInstruction::try_from_slice(instruction_data)
        .map_err(|_| StablecoinError::InvalidInstruction)?;

    match instruction {
        StablecoinInstruction::InitializeToken { name, symbol } => {
            // Log the initialization action
            sol_log(&format!("Initializing token: {} ({})", name, symbol));
            initialize_token(accounts, name, symbol)?;
        }
        StablecoinInstruction::Mint { amount } => {
            // Log the minting action
            sol_log(&format!("Minting {} tokens", amount));
            mint_tokens(accounts, amount)?;
        }
        StablecoinInstruction::Redeem { amount } => {
            // Log the redemption action
            sol_log(&format!("Redeeming {} tokens", amount));
            redeem_tokens(accounts, amount)?;
        }
    }

    Ok(())
}

// Function to handle the InitializeToken instruction
fn initialize_token(
    accounts: &[AccountInfo],
    name: String,
    symbol: String,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let token_account = next_account_info(accounts_iter)?; // Get the token account
    let rent_sysvar_account = next_account_info(accounts_iter)?; // Get the rent sysvar account

    // Validate that the token account is writable
    if !token_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Validate the rent sysvar account
    if rent_sysvar_account.key != &solana_program::sysvar::rent::id() {
        return Err(ProgramError::InvalidArgument);
    }

    // Create the initial token state
    let token_state = TokenState {
        name,
        symbol,
        total_supply: 0,
    };

    // Serialize the token state into the account data
    token_state.serialize(&mut &mut token_account.data.borrow_mut()[..])?;
    Ok(())
}

// Function to handle the Mint instruction
fn mint_tokens(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?; // Get the mint account

    // Validate that the mint account is writable
    if !mint_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the token state
    let mut token_state = TokenState::deserialize(&mut &mint_account.data.borrow_mut()[..])?;
    // Update the total supply
    token_state.total_supply += amount;
    // Serialize the updated token state back to the account
    token_state.serialize(&mut &mut mint_account.data.borrow_mut()[..])?;

    Ok(())
}

// Function to handle the Redeem instruction
fn redeem_tokens(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?; // Get the mint account

    // Validate that the mint account is writable
    if !mint_account.is_writable {
        return Err(ProgramError::InvalidAccountData);
    }

    // Deserialize the token state
    let mut token_state = TokenState::deserialize(&mut &mint_account.data.borrow_mut()[..])?;
    // Check if sufficient supply exists
    if token_state.total_supply < amount {
        return Err(ProgramError::InsufficientFunds);
    }
    // Update the total supply
    token_state.total_supply -= amount;
    // Serialize the updated token state back to the account
    token_state.serialize(&mut &mut mint_account.data.borrow_mut()[..])?;

    Ok(())
}