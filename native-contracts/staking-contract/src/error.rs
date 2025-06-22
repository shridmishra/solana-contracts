use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError {
    #[error("Invalid PDA derived")]
    InvalidPda,

    #[error("Pool already initialized")]
    PoolAlreadyInitialized,

    #[error("User stake account already exists")]
    UserAlreadyStaked,

    #[error("Nothing to claim")]
    NothingToClaim,
}

impl From<StakingError> for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
