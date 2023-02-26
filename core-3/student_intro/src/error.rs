use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IntroError {
    // Error 0
    #[error("Account not initialized yet")]
    UninitializedAccount,

    // Error 1
    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    // Error 2
    #[error("Input data exceeds max length")]
    InvalidDataLength,

    // Error 3
    #[error("Name exceeds maximum character length")]
    InvalidNameLength,

    // Error 4
    #[error("Intro is too large")]
    InvalidIntroLength,
}

impl From<IntroError> for ProgramError {
    fn from(value: IntroError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
