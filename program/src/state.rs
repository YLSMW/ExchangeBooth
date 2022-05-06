use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct ExchangeBooth {
    pub MintTokenX: [u8; 32],
    pub TokenXAmount : u64,
    pub MintTokenY: [u8; 32],
    pub TokenYAmount: u64,
    pub AdminPubkey: [u8; 32],
    pub OraclePubkey: [u8; 32],
        
}

