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
    error::{
        check_assert, declare_check_assert_macros, ExchangeBoothError, ExchangeBoothErrorCode,
        ExchangeBoothResult, SourceFileId,
    },
    instruction::ExchangeBoothInstruction, state::ExchangeBooth,
};

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        let instruction = ExchangeBoothInstruction::try_from_slice(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            ExchangeBoothInstruction::InitializeExchangeBooth { buffer_seed,fee_rate } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_initialize_exchangebooth(program_id, accounts, buffer_seed, fee_rate)
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
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        buffer_seed: u64,
        fee_rate: [u8; 2]
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let admin = next_account_info(account_info_iter)?;
        let x_vault_account = next_account_info(account_info_iter)?;
        let y_vault_account = next_account_info(account_info_iter)?;
        let x_mint_account = next_account_info(account_info_iter)?;
        let y_mint_account = next_account_info(account_info_iter)?;
        let exchange_booth = next_account_info(account_info_iter)?;
        let oracle = next_account_info(account_info_iter)?;
        let system_program_account = next_account_info(account_info_iter)?;
        let system_token_program_account = next_account_info(account_info_iter)?;        
        let (exchange_booth_key, bump_seed) = Pubkey::find_program_address(
            &[
                b"ExchangeBoothForTokenXAndTokenY",
                admin.key.as_ref(),
                &fee_rate[..],
                &buffer_seed.to_le_bytes(),
            ],
            program_id,
        ); 
        
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
