use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum StablecoinInstruction {
    InitializeToken {
        name: String,
        symbol: String,
    },
    Mint {
        amount: u64,
    },
    Redeem {
        amount: u64,
    },
}