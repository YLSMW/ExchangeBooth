use crate::error::{
    check_assert, declare_check_assert_macros, ExchangeBoothError, ExchangeBoothErrorCode,
    ExchangeBoothResult, SourceFileId,
};
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    //log::sol_log_compute_units,
};
use spl_token::{
    state::Account,
};
declare_check_assert_macros!(SourceFileId::Utils);

// go with processes and come back on-demand
pub fn check_token_account(
    program_id: &Pubkey,
    key_token_account: &Account,
    key_mint: &Pubkey,
) -> ExchangeBoothResult {
    let expected_mint_key = key_token_account.mint;
    let expected_owner_key = key_token_account.owner;
    
    check_eq!(
        &expected_mint_key,
        key_mint,
        ExchangeBoothErrorCode::TokenMintDismatch
    )?;

    check_eq!(
        &expected_owner_key,
        program_id,
        ExchangeBoothErrorCode::TokenOwnerDismatch
    )?;

    Ok(())
}

