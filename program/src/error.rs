use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum ExchangeBoothError {

    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Not Rent Exempt")]
    NotRentExempt,

    #[error("Authority Check Failed")]
    AuthorityCheckFailed,   

}


impl From<ExchangeBoothError> for ProgramError {
    fn from(e : ExchangeBoothError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
