use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
};
use spl_token::{
    instruction::{approve, burn, close_account, initialize_mint, mint_to, transfer},
    state::Account,
    state::Mint,
};

use crate::{
    error::ExchangeBoothError, instruction::ExchangeBoothInstruction, state::ExchangeBooth,
};

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        let instruction = ExchangeBoothInstruction::try_from_slice(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            ExchangeBoothInstruction::InitializeExchangeBooth {} => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_initialize_exchangebooth(program_id, accounts)
            }
            ExchangeBoothInstruction::Deposit { token_name, amount } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_deposit(program_id, accounts, token_name, amount)
            }
            ExchangeBoothInstruction::Withdraw { token_name, amount } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_withdraw(program_id, accounts, token_name, amount)
            }
            ExchangeBoothInstruction::Exchange { token_name_from, amount } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_exchange(program_id, accounts, token_name_from, amount)
            }
            ExchangeBoothInstruction::CloseExchangeBooth {} => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_close_exchangebooth(program_id, accounts)
            }

            _ => Ok(()),
        }
    }
    fn process_initialize_exchangebooth(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }

    fn process_deposit(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        token_name: Vec<u8>,
        amount: u64,
    ) -> ProgramResult {
        Ok(())
    }

    fn process_withdraw(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        token_name: Vec<u8>,
        amount: u64,
    ) -> ProgramResult {
        Ok(())
    }
    
    fn process_exchange(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        token_name_from: Vec<u8>,
        amount: u64,
    ) -> ProgramResult {
        Ok(())
    }

    fn process_close_exchangebooth(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        Ok(())
    }
}
