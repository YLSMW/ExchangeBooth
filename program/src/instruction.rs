use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ExchangeBoothInstruction {
    InitializeExchangeBooth {
        buffer_seed: u64,
        fee_rate: [u8; 2]
    },
    Deposit {
        token_name: Vec<u8>,
        amount: u64,
    },
    Withdraw {
        token_name: Vec<u8>,
        amount: u64,
    },
    Exchange {
        token_name_from: Vec<u8>,
        amount: u64,
    },
    CloseExchangeBooth {},
}
