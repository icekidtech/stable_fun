use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenState {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
}