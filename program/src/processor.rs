use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    system_instruction,
    sysvar::rent::Rent,
};
use spl_token::{
    instruction::{transfer, close_account},
    state::{Account, Mint},
};

use crate::{
    error::{
        check_assert, declare_check_assert_macros, ExchangeBoothError, ExchangeBoothErrorCode,
        ExchangeBoothResult, SourceFileId,
    },
    instruction::ExchangeBoothInstruction,
    state::{ExchangeBooth, ExchangeBoothOracle},
};
declare_check_assert_macros!(SourceFileId::Processor);

pub struct Processor {}

impl Processor {
    pub fn process_ix(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ExchangeBoothResult {
        let instruction = ExchangeBoothInstruction::try_from_slice(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        match instruction {
            ExchangeBoothInstruction::InitializeExchangeBooth {
                // buffer_seed,
                fee_rate,
            } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_initialize_exchangebooth(
                    program_id, accounts, /* buffer_seed, */ fee_rate,
                )
            }
            ExchangeBoothInstruction::Deposit { amount } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_deposit(program_id, accounts, amount)
            }
            ExchangeBoothInstruction::Withdraw { amount } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_withdraw(program_id, accounts, amount)
            }
            ExchangeBoothInstruction::Exchange { amount } => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_exchange(program_id, accounts, amount)
            }
            ExchangeBoothInstruction::CloseExchangeBooth {} => {
                msg!("Instruction: Initialize ExchangeBooth");
                Self::process_close_exchangebooth(program_id, accounts)
            }
        }
    }
    fn process_initialize_exchangebooth(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        // buffer_seed: u64,
        fee_rate: [u8; 2],
    ) -> ExchangeBoothResult {
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
                b"TokenVault",
                x_mint_account.key.as_ref(),
                admin.key.as_ref(),
            ],
            &exchange_booth_key,
        );
        let (token_y_vault_key, bump_seed_token_y_vault) = Pubkey::find_program_address(
            &[
                b"TokenVault",
                y_mint_account.key.as_ref(),
                admin.key.as_ref(),
            ],
            &exchange_booth_key,
        );

        check_eq!(
            exchange_booth.key,
            &exchange_booth_key,
            ExchangeBoothErrorCode::PDAAccountDismatch
        )?;
        check_eq!(
            x_vault_account.key,
            &token_x_vault_key,
            ExchangeBoothErrorCode::PDAAccountDismatch
        )?;
        check_eq!(
            y_vault_account.key,
            &token_y_vault_key,
            ExchangeBoothErrorCode::PDAAccountDismatch
        )?;

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
                b"TokenVault",
                x_mint_account.key.as_ref(),
                admin.key.as_ref(),
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
                b"TokenVault",
                y_mint_account.key.as_ref(),
                admin.key.as_ref(),
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
        let _oracle_data = ExchangeBoothOracle::try_from_slice(&oracle.data.borrow_mut())?;

        let data_dst = &mut *exchange_booth.data.borrow_mut();

        let data: Vec<u8> = admin
            .key
            .to_bytes()
            .iter()
            .copied()
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
    ) -> ExchangeBoothResult {
        let account_info_iter = &mut accounts.iter();
        let admin = next_account_info(account_info_iter)?;
        let token_account = next_account_info(account_info_iter)?;
        let exchange_booth = next_account_info(account_info_iter)?;
        let token_vault = next_account_info(account_info_iter)?;
        let system_token_program_account = next_account_info(account_info_iter)?;
        let token_account_data = Account::unpack_from_slice(&token_account.data.borrow_mut())?;
        let token_vault_data = Account::unpack_from_slice(&token_vault.data.borrow_mut())?;

        check_eq!(
            token_vault_data.is_initialized(),
            true,
            ExchangeBoothErrorCode::PDAAccountNotInitialized
        )?;
        check_eq!(
            admin.is_signer,
            true,
            ExchangeBoothErrorCode::AuthorityCheckFailed
        )?;
        check_eq!(
            token_vault_data.owner,
            *exchange_booth.key,
            ExchangeBoothErrorCode::TokenOwnerDismatch
        )?;
        check_eq!(
            token_account_data.owner,
            *admin.key,
            ExchangeBoothErrorCode::TokenOwnerDismatch
        )?;
        check_eq!(
            token_vault_data.mint,
            token_account_data.mint,
            ExchangeBoothErrorCode::TokenMintDismatch
        )?;
        let create_token_deposit_ix = transfer(
            system_token_program_account.key,
            token_account.key,
            token_vault.key,
            admin.key,
            &[&admin.key],
            amount,
        )?;
        // to debug: what if the amount is larger than token_account.amount
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
    ) -> ExchangeBoothResult {
        let account_info_iter = &mut accounts.iter();
        let admin = next_account_info(account_info_iter)?;
        let token_account = next_account_info(account_info_iter)?;
        let exchange_booth = next_account_info(account_info_iter)?;
        let token_vault = next_account_info(account_info_iter)?;
        let system_token_program_account = next_account_info(account_info_iter)?;
        let token_account_data = Account::unpack_from_slice(&token_account.data.borrow_mut())?;
        let token_vault_data = Account::unpack_from_slice(&token_vault.data.borrow_mut())?;

        check_eq!(
            token_vault_data.is_initialized(),
            true,
            ExchangeBoothErrorCode::PDAAccountNotInitialized
        )?;
        check_eq!(
            admin.is_signer,
            true,
            ExchangeBoothErrorCode::AuthorityCheckFailed
        )?;
        check_eq!(
            token_vault_data.owner,
            *exchange_booth.key,
            ExchangeBoothErrorCode::TokenOwnerDismatch
        )?;
        check_eq!(
            token_account_data.owner,
            *admin.key,
            ExchangeBoothErrorCode::TokenOwnerDismatch
        )?;
        check_eq!(
            token_vault_data.mint,
            token_account_data.mint,
            ExchangeBoothErrorCode::TokenMintDismatch
        )?;
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
        amount: u64,
    ) -> ExchangeBoothResult {
        let account_info_iter = &mut accounts.iter();
        let exchange_booth_account = next_account_info(account_info_iter)?;
        let user_account = next_account_info(account_info_iter)?;
        let oracle_account = next_account_info(account_info_iter)?;
        let source_token_account = next_account_info(account_info_iter)?;
        let dest_token_account = next_account_info(account_info_iter)?;
        let x_vault_account = next_account_info(account_info_iter)?;
        let y_vault_account = next_account_info(account_info_iter)?;
        let x_mint_account = next_account_info(account_info_iter)?;
        let y_mint_account = next_account_info(account_info_iter)?;
        // let system_program_account = next_account_info(account_info_iter)?;
        let system_token_program_account = next_account_info(account_info_iter)?;
        check_eq!(
            user_account.is_signer,
            true,
            ExchangeBoothErrorCode::AuthorityCheckFailed
        )?;
        let exchange_booth_account_data =
            ExchangeBooth::try_from_slice(&exchange_booth_account.data.borrow_mut())?;
        check_eq!(
            Pubkey::new_from_array(exchange_booth_account_data.TokenXVault),
            *x_vault_account.key,
            ExchangeBoothErrorCode::TokenMintDismatch
        )?;
        check_eq!(
            Pubkey::new_from_array(exchange_booth_account_data.TokenYVault),
            *y_vault_account.key,
            ExchangeBoothErrorCode::TokenMintDismatch
        )?;
        check_eq!(
            Pubkey::new_from_array(exchange_booth_account_data.OraclePubkey),
            *oracle_account.key,
            ExchangeBoothErrorCode::OracleDismatch
        )?;
        let fee_rate = exchange_booth_account_data.FeeRate; //way to cut fee is return dest_token with deducted tokens. Keep the fee in vault in stead of returning to admin's account.

        let source_token_account_data =
            Account::unpack_from_slice(&source_token_account.data.borrow_mut())?;
        let dest_token_account_data =
            Account::unpack_from_slice(&dest_token_account.data.borrow_mut())?;
        let x_vault_account_data = Account::unpack_from_slice(&x_vault_account.data.borrow_mut())?;
        let y_vault_account_data = Account::unpack_from_slice(&y_vault_account.data.borrow_mut())?;

        let oracle_account_data =
            ExchangeBoothOracle::try_from_slice(&oracle_account.data.borrow_mut())?;

        check_eq!(
            Pubkey::new_from_array(oracle_account_data.TokenXMInt),
            *x_mint_account.key,
            ExchangeBoothErrorCode::TokenMintDismatch
        )?;
        check_eq!(
            Pubkey::new_from_array(oracle_account_data.TokenYMint),
            *y_mint_account.key,
            ExchangeBoothErrorCode::TokenMintDismatch
        )?;
        let exchange_rate = oracle_account_data.RatioXTo1Y;

        check_eq!(
            x_vault_account_data.mint,
            *x_mint_account.key,
            ExchangeBoothErrorCode::TokenMintDismatch
        )?;
        check_eq!(
            y_vault_account_data.mint,
            *y_mint_account.key,
            ExchangeBoothErrorCode::TokenMintDismatch
        )?;

        if source_token_account_data.mint == *x_mint_account.key
            && dest_token_account_data.mint == *y_mint_account.key
        {
            // calculate amount, note that mint decimal is handled in FE.
            let exchange_amount_y = (amount as u128)
                .checked_mul(10u128.checked_pow(exchange_rate[1] as u32).unwrap())
                .unwrap()
                .checked_div(exchange_rate[0] as u128)
                .unwrap()
                .checked_mul(
                    10u128
                        .checked_pow(fee_rate[1] as u32)
                        .unwrap()
                        .checked_sub(fee_rate[0] as u128)
                        .unwrap(),
                )
                .unwrap()
                .checked_div(10u128.checked_pow(fee_rate[1] as u32).unwrap())
                .unwrap();

            if exchange_amount_y > u64::MAX as u128 {
                return Err(throw_err!(ExchangeBoothErrorCode::InputAmountIllegal));
            }
            let exchange_amount_y = exchange_amount_y as u64;

            let create_token_deposit_ix = transfer(
                system_token_program_account.key,
                source_token_account.key,
                x_vault_account.key,
                user_account.key,
                &[&user_account.key],
                amount,
            )?;
            msg!("Conduct Deposit...");
            invoke(
                &create_token_deposit_ix,
                &[
                    system_token_program_account.clone(),
                    source_token_account.clone(),
                    x_vault_account.clone(),
                    user_account.clone(),
                ],
            )?;

            let create_token_deposit_ix = transfer(
                system_token_program_account.key,
                y_vault_account.key,
                dest_token_account.key,
                user_account.key,
                &[&user_account.key],
                exchange_amount_y,
            )?;
            msg!("Conduct Withdraw...");
            invoke(
                &create_token_deposit_ix,
                &[
                    system_token_program_account.clone(),
                    y_vault_account.clone(),
                    dest_token_account.clone(),
                    user_account.clone(),
                ],
            )?;
        } else if source_token_account_data.mint == *y_mint_account.key
            && dest_token_account_data.mint == *x_mint_account.key
        {
            let exchange_amount_x = (amount as u128)
                .checked_mul(exchange_rate[0] as u128)
                .unwrap()
                .checked_div(10u128.checked_pow(exchange_rate[1] as u32).unwrap())
                .unwrap()
                .checked_mul(
                    10u128
                        .checked_pow(fee_rate[1] as u32)
                        .unwrap()
                        .checked_sub(fee_rate[0] as u128)
                        .unwrap(),
                )
                .unwrap()
                .checked_div(10u128.checked_pow(fee_rate[1] as u32).unwrap())
                .unwrap();

            if exchange_amount_x > u64::MAX as u128 {
                return Err(throw_err!(ExchangeBoothErrorCode::InputAmountIllegal));
            }
            let exchange_amount_x = exchange_amount_x as u64;

            let create_token_deposit_ix = transfer(
                system_token_program_account.key,
                source_token_account.key,
                y_vault_account.key,
                user_account.key,
                &[&user_account.key],
                amount,
            )?;
            msg!("Conduct Deposit...");
            invoke(
                &create_token_deposit_ix,
                &[
                    system_token_program_account.clone(),
                    source_token_account.clone(),
                    y_vault_account.clone(),
                    user_account.clone(),
                ],
            )?;

            let create_token_deposit_ix = transfer(
                system_token_program_account.key,
                x_vault_account.key,
                dest_token_account.key,
                user_account.key,
                &[&user_account.key],
                exchange_amount_x,
            )?;

            msg!("Conduct Withdraw...");
            invoke(
                &create_token_deposit_ix,
                &[
                    system_token_program_account.clone(),
                    x_vault_account.clone(),
                    dest_token_account.clone(),
                    user_account.clone(),
                ],
            )?;
        } else {
            return Err(throw_err!(ExchangeBoothErrorCode::TokenMintDismatch));
        }

        Ok(())
    }

    fn process_close_exchangebooth(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
    ) -> ExchangeBoothResult {
        // close vaultX : transfer token back to Admin token account, withdraw all sols close account(how)
        // close vaultY : transfer token back to Admin token account, withdraw all sols close account(how)
        // close ExchangeBooth : withdraw all sols close account(how)
        Ok(())
    }
}
