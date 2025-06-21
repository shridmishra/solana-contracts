use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Account not rent exempt")]
    NotRentExempt,

    #[error("Pool already initialized")]
    AlreadyInitialized,
}

impl From<StakingError> for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
