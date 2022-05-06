use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ExchangeBoothInstruction{
InitializeExchangeBooth{
},
Deposit{
},
Withdraw{
},
Exchange{
},
CloseExchangeBooth{
},
}