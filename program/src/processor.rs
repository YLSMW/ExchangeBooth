use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
};
use spl_token::{
    instruction::{approve, burn, close_account, initialize_mint, mint_to, transfer},
    state::{Account,Mint},
};

use crate::{
    error::{
        check_assert, declare_check_assert_macros, ExchangeBoothError, ExchangeBoothErrorCode,
        ExchangeBoothResult, SourceFileId,
    },
    instruction::ExchangeBoothInstruction,
    state::ExchangeBooth,
    utils::{check_token_account},
};
declare_check_assert_macros!(SourceFileId::Processor);

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
        let instruction = ExchangeBoothInstruction::try_from_slice(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            ExchangeBoothInstruction::InitializeExchangeBooth {
                // buffer_seed,
                fee_rate,
            } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_initialize_exchangebooth(program_id, accounts, /* buffer_seed, */ fee_rate)
            }
            ExchangeBoothInstruction::Deposit { amount } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_deposit(program_id, accounts, amount)
            }
            ExchangeBoothInstruction::Withdraw { amount } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_withdraw(program_id, accounts, amount)
            }
            ExchangeBoothInstruction::Exchange {
                token_name_from,
                amount,
            } => {
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
        // buffer_seed: u64,
        fee_rate: [u8; 2],
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

        // order: 
        // 1 get exchange_booth info, 
        // 2 get vaults info from exchange_booth,
        // 3 create exchange_booth
        // 4 create vaults with owner as token program
        // 5 initiate exchange_booth
        let (exchange_booth_key, bump_seed_exchange_booth) = Pubkey::find_program_address(
            &[
                b"ExchangeBoothForTokenXAndTokenY",
                admin.key.as_ref(),
                &fee_rate[..],
                // &buffer_seed.to_le_bytes(),
            ],
            program_id,
        );

        let (token_x_vault_key, bump_seed_token_x_vault) = Pubkey::find_program_address(
            &[
                b"TokenXVault",
                admin.key.as_ref(),
            ],
            &exchange_booth_key,
        );
        let (token_y_vault_key, bump_seed_token_y_vault) = Pubkey::find_program_address(
            &[
                b"TokenYVault",
                admin.key.as_ref(),
            ],
            &exchange_booth_key,
        );

        check_eq! (exchange_booth.key, &exchange_booth_key, ExchangeBoothErrorCode::PDAAccountDismatch);
        check_eq! (x_vault_account.key, &token_x_vault_key, ExchangeBoothErrorCode::PDAAccountDismatch);
        check_eq! (y_vault_account.key, &token_y_vault_key, ExchangeBoothErrorCode::PDAAccountDismatch);

        let rent = Rent::default();
        let create_exchange_booth_account_ix = system_instruction::create_account(
            admin.key,
            &exchange_booth_key,
            rent.minimum_balance(130 as usize), // 4 pubkey and 1 [u8; 2]
            130 as u64,
            program_id,
        );

        msg!("Creating Exchange Booth Account...");
        invoke_signed(
            &create_exchange_booth_account_ix,
            &[
                system_program_account.clone(),
                admin.clone(),
                exchange_booth.clone(),
            ],
            &[&[
                b"ExchangeBoothForTokenXAndTokenY",
                admin.key.as_ref(),
                &fee_rate,
                &[bump_seed_exchange_booth],
            ]],
        )?;
        msg!("Exchange Booth Account Created.");

        let create_token_x_vault_account_ix = system_instruction::create_account(
            admin.key,
            &token_x_vault_key,
            rent.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            system_token_program_account.key,
        );

        msg!("Creating Token X Vault...");
        invoke_signed(
            &create_token_x_vault_account_ix,
            &[
                system_program_account.clone(),
                admin.clone(),
                exchange_booth.clone(),
            ],
            &[&[
                b"TokenXVault",
                admin.key.as_ref(),
                &fee_rate,
                &[bump_seed_token_x_vault],
            ]],
        )?;

        msg!("Initiating Token X Vault...");
        let initiate_token_x_vault_account_ix = spl_token::instruction::initialize_account(
            &system_token_program_account.key,
            &token_x_vault_key,
            &x_mint_account.key,
            &exchange_booth_key,
        )?;

        invoke(
            &initiate_token_x_vault_account_ix,
            &[
                system_token_program_account.clone(),
                x_vault_account.clone(),
                x_mint_account.clone(),
                exchange_booth.clone(),
            ],
        )?;

        let create_token_y_vault_account_ix = system_instruction::create_account(
            admin.key,
            &token_y_vault_key,
            rent.minimum_balance(Mint::LEN),
            Mint::LEN as u64,
            system_token_program_account.key,
        );
        msg!("Creating Token Y Vault Account...");
        invoke_signed(
            &create_token_y_vault_account_ix,
            &[
                system_program_account.clone(),
                admin.clone(),
                exchange_booth.clone(),
            ],
            &[&[
                b"TokenYVault",
                admin.key.as_ref(),
                &fee_rate,
                &[bump_seed_token_y_vault],
            ]],
        )?;

        msg!("Initiating Token Y Vault...");
        let initiate_token_y_vault_account_ix = spl_token::instruction::initialize_account(
            &system_token_program_account.key,
            &token_y_vault_key,
            &y_mint_account.key,
            &exchange_booth_key,
        )?;

        invoke(
            &initiate_token_y_vault_account_ix,
            &[
                system_token_program_account.clone(),
                y_vault_account.clone(),
                y_mint_account.clone(),
                exchange_booth.clone(),

            ],
        )?;

        msg!("Check Oracle...");
        let _oracle_data = ExchangeBooth::try_from_slice(&oracle.data.borrow_mut())?;

        let data_dst = &mut *exchange_booth.data.borrow_mut();

        let data: Vec<u8> = admin.key.to_bytes().iter().copied()
            .chain(oracle.key.to_bytes().iter().copied()) 
            .chain(x_vault_account.key.to_bytes().iter().copied())   
            .chain(y_vault_account.key.to_bytes().iter().copied())              
            .chain(fee_rate.iter().copied())
            .collect();

        let data = ExchangeBooth::try_from_slice(&data)?;
        data.serialize(data_dst)?;
        msg!("Initialization Done");

        Ok(())
    }

    fn process_deposit(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let admin = next_account_info(account_info_iter)?;
        let token_account = next_account_info(account_info_iter)?;       
        let exchange_booth = next_account_info(account_info_iter)?;
        let token_vault = next_account_info(account_info_iter)?;
        let system_token_program_account = next_account_info(account_info_iter)?;
        
        let token_account_data = Account::unpack_from_slice(&token_account.data.borrow_mut())?;
        let token_vault_data = Account::unpack_from_slice(&token_vault.data.borrow_mut())?;

        check_eq! (token_vault_data.is_initialized(), true, ExchangeBoothErrorCode::PDAAccountNotInitialized);
        check_eq! (admin.is_signer, true, ExchangeBoothErrorCode::AuthorityCheckFailed);
        check_eq! (token_vault_data.owner, *exchange_booth.key, ExchangeBoothErrorCode::TokenOwnerDismatch);
        check_eq! (token_account_data.owner, *admin.key, ExchangeBoothErrorCode::TokenOwnerDismatch);
        check_eq! (token_vault_data.mint, token_account_data.mint, ExchangeBoothErrorCode::TokenMintDismatch);
        let create_token_deposit_ix = transfer(
            system_token_program_account.key,
            token_account.key,
            token_vault.key,
            admin.key,
            &[&admin.key],
            amount,
        )?;

        msg!("Conduct Deposit...");
        invoke(
            &create_token_deposit_ix,
            &[
                system_token_program_account.clone(),
                token_account.clone(),
                token_vault.clone(),
                admin.clone(),
            ],
        )?; 
        Ok(())
    }

    fn process_withdraw(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let admin = next_account_info(account_info_iter)?;
        let token_account = next_account_info(account_info_iter)?;       
        let exchange_booth = next_account_info(account_info_iter)?;
        let token_vault = next_account_info(account_info_iter)?;
        let system_token_program_account = next_account_info(account_info_iter)?;
        
        let token_account_data = Account::unpack_from_slice(&token_account.data.borrow_mut())?;
        let token_vault_data = Account::unpack_from_slice(&token_vault.data.borrow_mut())?;

        check_eq! (token_vault_data.is_initialized(), true, ExchangeBoothErrorCode::PDAAccountNotInitialized);
        check_eq! (admin.is_signer, true, ExchangeBoothErrorCode::AuthorityCheckFailed);
        check_eq! (token_vault_data.owner, *exchange_booth.key, ExchangeBoothErrorCode::TokenOwnerDismatch);
        check_eq! (token_account_data.owner, *admin.key, ExchangeBoothErrorCode::TokenOwnerDismatch);
        check_eq! (token_vault_data.mint, token_account_data.mint, ExchangeBoothErrorCode::TokenMintDismatch);
        
        let create_token_deposit_ix = transfer(
            system_token_program_account.key,
            token_vault.key,
            token_account.key,
            admin.key,
            &[&admin.key],
            amount,
        )?;

        msg!("Conduct Withdraw...");
        invoke(
            &create_token_deposit_ix,
            &[
                system_token_program_account.clone(),
                token_vault.clone(),
                token_account.clone(),
                admin.clone(),
            ],
        )?; 
        
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
