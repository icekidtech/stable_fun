use borsh::BorshSerialize;
use crate::{error::StablecoinError, instruction::StablecoinInstruction};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    log::sol_log,
    program_error::ProgramError,
    program_pack::Pack,
    sysvar::{rent::Rent, Sysvar},
};
use solana_program::serde_varint::VarInt;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize the instruction data
    let instruction = StablecoinInstruction::try_from_slice(instruction_data)
        .map_err(|_| StablecoinError::InvalidInstruction)?;

    // Match the instruction type and handle it
    match instruction {
        StablecoinInstruction::InitializeToken { name, symbol } => {
            sol_log(&format!("Initializing token: {} ({})", name, symbol));
            initialize_token(program_id, accounts, name, symbol)?;
        }
        StablecoinInstruction::Mint { amount } => {
            sol_log(&format!("Minting {} tokens", amount));
            mint_tokens(program_id, accounts, amount)?;
        }
        StablecoinInstruction::Redeem { amount } => {
            sol_log(&format!("Redeeming {} tokens", amount));
            redeem_tokens(program_id, accounts, amount)?;
        }
    }

    Ok(())
}

/// Initialize the token state
fn initialize_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    symbol: String,
) -> ProgramResult {
    // Validate accounts
    let accounts_iter = &mut accounts.iter();
    let token_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;
    let rent_sysvar_account = next_account_info(accounts_iter)?;

    // Ensure the token account is writable
    if !token_account.is_writable {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Ensure the payer account is writable
    if !payer_account.is_writable {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Ensure the rent sysvar account is provided
    if rent_sysvar_account.key != &Rent::free() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Create a new token state
    let token_state = TokenState {
        name,
        symbol,
        total_supply: 0,
    };

    // Pack the token state into the token account
    #[derive(BorshDeserialize, BorshSerialize, Debug)]
    pub struct TokenState {
        pub name: String,
        pub symbol: String,
        pub total_supply: u64,
    }

    // Set the token account's owner to the program ID
    token_account.owner = program_id;

    Ok(())
}

/// Mint tokens to the specified account
fn mint_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // Validate accounts
    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?;
    let recipient_account = next_account_info(accounts_iter)?;
    let authority_account = next_account_info(accounts_iter)?;

    // Ensure the mint account is writable
    if !mint_account.is_writable {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Ensure the recipient account is writable
    if !recipient_account.is_writable {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Ensure the authority account is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check authority
    if authority_account.key != &Pubkey::new_unique() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Mint tokens
    let mut token_state = TokenState::deserialize(&mut &mut mint_account.data.borrow_mut()[..])?;
    token_state.total_supply += amount;
    token_state.serialize(&mut &mut mint_account.data.borrow_mut()[..])?;

    // Update the recipient account's balance
    let mut recipient_balance = 0;
    if let Some(data) = &mut recipient_account.data.borrow_mut() {
        recipient_balance = u64::deserialize(data)?;
    }
    recipient_balance += amount;
    recipient_balance.serialize_varint(&mut &mut recipient_account.data.borrow_mut()[..])?;

    Ok(())
}

/// Redeem tokens from the specified account
fn redeem_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    // Validate accounts
    let accounts_iter = &mut accounts.iter();
    let mint_account = next_account_info(accounts_iter)?;
    let sender_account = next_account_info(accounts_iter)?;
    let authority_account = next_account_info(accounts_iter)?;

    // Ensure the mint account is writable
    
    if !mint_account.is_writable {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Ensure the sender account is writable
    if !sender_account.is_writable {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Ensure the authority account is a signer
    if !authority_account.is_signer {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check authority
    if authority_account.key != &Pubkey::new_unique() {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Redeem tokens
    let mut token_state = TokenState::deserialize(&mut &mut mint_account.data.borrow_mut()[..])?;
    if token_state.total_supply < amount {
        return Err(ProgramError::InsufficientFunds);
    }
    token_state.total_supply -= amount;
    token_state.serialize(&mut &mut mint_account.data.borrow_mut()[..])?;

    // Update the sender account's balance
    let mut sender_balance = 0;
    if let Some(data) = &mut sender_account.data.borrow_mut() {
        sender_balance = u64::deserialize(data)?;
    }
    if sender_balance < amount {
        return Err(ProgramError::InsufficientFunds);
    }
    sender_balance -= amount;
    sender_balance.serialize_varint(&mut &mut sender_account.data.borrow_mut()[..])?;

    Ok(())
}