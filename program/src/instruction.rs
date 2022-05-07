use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ExchangeBoothInstruction{
InitializeExchangeBooth{

},
Deposit{
    token_name: Vec<u8>,
    amount: u64
},
Withdraw{
    token_name: Vec<u8>,
    amount: u64    
},
Exchange{
    token_name_from: Vec<u8>,
    amount: u64
},
CloseExchangeBooth{
},
}