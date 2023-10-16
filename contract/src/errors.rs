use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PresaleError {
    /// Account does not have correct owner
    #[error("Account does not have correct owner")]
    IncorrectOwner,

    #[error("Derived key is invalid")]
    DerivedKeyInvalid,

    #[error("Invalid authority")]
    InvalidAuthority,

    #[error("Presale has already started")]
    AlreadyStarted,

    #[error("Data type mismatch")]
    DataTypeMismatch,

    #[error("Already stopped")]
    AlreadyStopped,

    #[error("Invalid client owner")]
    InvalidClientOwner,

    #[error("Invalid presale account")]
    InvalidPresaleAccount,

    #[error("Invalid token program")]
    InvalidTokenProgram,

    #[error("Not match presale address")]
    NotMatchPresale,

    #[error("Preslae is not active yet")]
    NotActiveYet,

    #[error("Amount is invalid")]
    InvalidAmount,

    #[error("Not match token address")]
    NotMatchTokenAddress,

    #[error("Balance too low")]
    BalanceTooLow,

    #[error("Hardcap has been reached")]
    HardcapReached,

    #[error("You will be going over the hardcap")]
    WillOverHardcap,

    #[error("You cant buy more than the max allocation")]
    MoreThanMaxAllocation,

    #[error("You are not whitelisted")]
    NotWhitelisted,

    #[error("Token transfer failed")]
    TokenTransferFailed,

    #[error("Already distributed 100% of tokens")]
    AlreadyDistributedOverflow
}

impl PrintProgramError for PresaleError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<PresaleError> for ProgramError {
    fn from(e: PresaleError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PresaleError {
    fn type_of() -> &'static str {
        "Vault Error"
    }
}
