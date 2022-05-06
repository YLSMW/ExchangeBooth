use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke_signed, invoke},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
};
use spl_token::{
    instruction::{burn/* , approve, close_account, initialize_mint, mint_to, transfer */},
    state::Account,
    /* state::Mint, */
};

use crate::{
    error::ExchangeBoothError,
    instruction::ExchangeBoothInstruction,
    state::{ExchangeBooth},
};

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]){}
}