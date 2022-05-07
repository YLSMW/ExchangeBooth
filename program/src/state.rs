use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ExchangeBooth {
    // pub MintTokenX: [u8; 32],
    pub TokenXVault: [u8; 32],
    // pub TokenXAmount : u64,
    // pub MintTokenY: [u8; 32],
    pub TokenYVault: [u8; 32],
    // pub TokenYAmount: u64,
    pub AdminPubkey: [u8; 32],
    pub OraclePubkey: [u8; 32],
    pub FeeRate: [u8; 2], //rateNum + decimal For Example [1, 3] means 0.1%
}

