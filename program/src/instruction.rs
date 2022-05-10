use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ExchangeBoothInstruction {
    InitializeExchangeBooth {
        // buffer_seed: u64,
        fee_rate: [u8; 2]
    },
    Deposit {
        amount: u64,
    },
    Withdraw {
        amount: u64,
    },
    Exchange {
        amount: u64,
    },
    CloseExchangeBooth {},
}
