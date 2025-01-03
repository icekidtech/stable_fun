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
        }
        .try_to_vec()
        .unwrap();
        processor::process(accounts, &instruction_data).unwrap();
        // Test the initialize_token logic
    }
}