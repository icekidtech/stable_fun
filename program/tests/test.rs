use borsh;

#[cfg(test)]
mod tests {
    use program::processor;
    use solana_program::pubkey::Pubkey;
    use program::instruction::StablecoinInstruction;

    #[test]
    fn test_initialize_token() {
        let program_id = Pubkey::new_unique();
        // Mock accounts and data
        let accounts = vec![];
        let instruction_data = StablecoinInstruction::InitializeToken {
            name: "Test Token".to_string(),
            symbol: "TEST".to_string(),
        };
        let mut buffer = Vec::new();
        <StablecoinInstruction as borsh::BorshSerialize>::serialize(&instruction_data, &mut buffer).unwrap();
        processor::process(accounts, &buffer).unwrap();
        // Test the initialize_token logic
    }
}