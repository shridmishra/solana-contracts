use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub use borsh::BorshSerialize;


pub mod instruction;
pub mod processor;
pub mod state;
pub mod error;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process(program_id, accounts, instruction_data)
}
