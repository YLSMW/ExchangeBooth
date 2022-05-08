use num_enum::IntoPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError, pubkey::PubkeyError};
use thiserror::Error;
pub type ExchangeBoothResult<T = ()> = Result<T, ExchangeBoothError>;

#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum SourceFileId {
    Processor = 0,
    State = 1,
    Utils = 2,
    Instruction = 3,
}

impl std::fmt::Display for SourceFileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SourceFileId::Processor => write!(f, "src/processor.rs"),
            SourceFileId::State => write!(f, "src/state.rs"),
            SourceFileId::Utils => write!(f, "src/utils.rs"),
            SourceFileId::Instruction => write!(f, "src/instruction.rs"),
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ExchangeBoothError {
    #[error(transparent)]
    ProgramError(#[from] ProgramError),
    #[error("{exchange_booth_error_code}; {source_file_id}:{line}")]
    ExchangeBoothErrorCode {
        exchange_booth_error_code: ExchangeBoothErrorCode,
        line: u32,
        source_file_id: SourceFileId,
    },
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)] // up to 2^32 error types
pub enum ExchangeBoothErrorCode
{
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Not Rent Exempt")]
    NotRentExempt,

    #[error("Authority Check Failed")]
    AuthorityCheckFailed,

    #[error("Efficient Token")]
    EfficientToken,

    #[error("Token Out Of Supply")]
    TokenOutOfSupply,

    #[error("Target Token Check Failed")]
    TargetTokenCheckFailed,

    #[error("Sourse Token Check Failed")]
    SourseTokenCheckFailed,

    #[error("Input Amount Illegal")]
    InputAmountIllegal,

    #[error("Token Mint Dismatch")]
    TokenMintDismatch,

    #[error("Token Owner Dismatch")]
    TokenOwnerDismatch


}

impl From<ExchangeBoothError> for ProgramError {
    fn from(e: ExchangeBoothError) -> Self {
        match e {
            ExchangeBoothError::ProgramError(pe) => pe,
            ExchangeBoothError::ExchangeBoothErrorCode {
                exchange_booth_error_code,
                line: _,
                source_file_id: _,
            } => ProgramError::Custom(exchange_booth_error_code.into()),
        }
    }
}

impl From<solana_program::pubkey::PubkeyError> for ExchangeBoothError {
    fn from(de: solana_program::pubkey::PubkeyError) -> Self {
        let pe: PubkeyError = de.into();
        pe.into()
    }
}

impl<T> DecodeError<T> for ExchangeBoothError {
    fn type_of() -> &'static str {
        "ExchangeBoothError"
    }
}


#[inline]
pub fn check_assert(
    cond: bool,
    exchange_booth_error_code: ExchangeBoothErrorCode,
    line: u32,
    source_file_id: SourceFileId,
) -> ExchangeBoothResult<()> {
    if cond {
        Ok(())
    } else {
        Err(ExchangeBoothError::ExchangeBoothErrorCode {
            exchange_booth_error_code,
            line,
            source_file_id,
        })
    }
}

#[macro_export]
macro_rules! declare_check_assert_macros {
    ($source_file_id:expr) => {
        #[allow(unused_macros)]
        macro_rules! check {
            ($cond:expr, $err:expr) => {
                check_assert($cond, $err, line!(), $source_file_id)
            };
        }

        #[allow(unused_macros)]
        macro_rules! check_eq {
            ($x:expr, $y:expr, $err:expr) => {
                check_assert($x == $y, $err, line!(), $source_file_id)
            };
        }

        #[allow(unused_macros)]
        macro_rules! throw_err {
            ($err:expr) => {
                ExchangeBoothError::ExchangeBoothErrorCode {
                    exchange_booth_error_code: $err,
                    line: line!(),
                    source_file_id: $source_file_id,
                }
            };
        }
    };
}
pub(crate) use declare_check_assert_macros;

