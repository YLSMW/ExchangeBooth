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
    pubkey::Pubkey,
    //log::sol_log_compute_units,
};

declare_check_assert_macros!(SourceFileId::Utils);

// go with processes and come back on-demand
pub fn check_token_account(
    program_id: &Pubkey,
    key_token_account: &Pubkey,
    key_authority:  &Pubkey,
    key_mint: &Pubkey,
) -> ExchangeBoothResult {
    let expected_mint_key = key_token_account; //parseddata(key_token_account.data).mint; 
    let expected_owner_key = key_token_account; //parseddata(key_token_account.data).owner; 
    let expected_authority_key = key_token_account; //parseddata(key_token_account.data).owner; 
    check_eq!(
        expected_mint_key,
        key_mint,
        ExchangeBoothErrorCode::TokenMintDismatch
    )?;

    check_eq!(
        expected_owner_key,
        program_id,
        ExchangeBoothErrorCode::TokenOwnerDismatch
    )?;

    check_eq!(
        expected_authority_key,
        key_authority,
        ExchangeBoothErrorCode::TokenOwnerDismatch
    )?;

    Ok(())
}

