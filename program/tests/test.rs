use program::processor;
use solana_program::pubkey::Pubkey;
use solana_program::account_info::AccountInfo;
use program::instruction::StablecoinInstruction;
use borsh::BorshSerialize;

#[test]
fn test_initialize_token() {
    let program_id = Pubkey::new_unique();

    // Mock accounts
    let mut lamports = 0;
    let owner = Pubkey::new_unique();
    let token_account = AccountInfo::new(
        &program_id,
        false,
        true,
        &mut lamports,
        &mut [0; 0],
        &owner,
        false,
        0,
    ); // Writable token account
    let rent_sysvar_id = solana_program::sysvar::rent::id();
    let mut rent_lamports = 0;
    let rent_sysvar_owner = Pubkey::new_unique();
    let rent_sysvar_account = AccountInfo::new(
        &rent_sysvar_id,
        false,
        false,
        &mut rent_lamports,
        &mut [0; 0],
        &rent_sysvar_owner,
        false,
        0,
    ); // Rent sysvar account
    let accounts = vec![token_account, rent_sysvar_account];

    // Mock instruction data
    let instruction_data = StablecoinInstruction::InitializeToken {
        name: "Test Token".to_string(),
        symbol: "TEST".to_string(),
    }
    .try_to_vec()
    .unwrap();

    // Call the process function
    processor::process(&accounts, &instruction_data).unwrap();

    // Add assertions to verify the result
}