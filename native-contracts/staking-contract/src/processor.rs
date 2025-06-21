use crate::{
    instruction::StakingInstruction,
    state::StakingPool,
    error::StakingError,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    rent::Rent,
    sysvar::Sysvar,
};