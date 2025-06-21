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

pub fn process(
    program_id:&pubkey;
    accounts:&[AccountInfo];
    instruction_data:&[u8];
)->ProgramResult{
    let instruction = StakingInstruction::try_from_slice(instruction_data)
     .map_err(|_|StakingError::InvalidInstruction)?;

    match instruction {
        StakingInstruction::InitializePool {reward_rate} => {
            process_initialize_pool(accounts,reward_rate,program_id)
        }
    }
}

fn process_initialize_pool(
    program_id:&pubkey;
    reward_rate:&u64;
    accounts:&[AccountInfo];

)->ProgramResult{
    let account_info_iter = &mut account_info.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let admin_account = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    let rent= &Rent::from_account_info(rent_sysvar)?;
    if !rent.is_exempt(pool_account.lamports(),pool_account.data_len()) {
        return Err(StakingError::NotRentExempt.into());
    }

    if pool_account.owner != program_id{
        return Err(ProgramError::IncorrectProgramId);

    }

    let mut pool_data = StakingPool::try_from_slice(&pool_account.data.borrow())?;
        if pool_data.total_staked != 0 {
        return Err(StakingError::AlreadyInitialized.into());
    }

    let pool = StakingPool{
        admin: admin_account.key.to_bytes();
        reward_rate,
        total_staked=0,
    }

    pool.serialize(&mut &mut pool_account.data.borrow()[..])?;
    msg!("Staking Pool initialized");
    Ok(())








}

pub fn process_stake(
    accounts: &[AccountInfo],
    amount: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let pool_account = next_account_info(account_info_iter)?;
    let vault_account = next_account_info(account_info_iter)?;
    let user_wallet = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let user_stake_info_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    let seeds = &[b"user-stake",user_wallet.key.as_ref(),pool_account.key.as]

    let (expected_pda,bump ) = Pubkey::find_program_address(seeds,program_id);

    user_stake_info_account.key != &expected_pda{
        return Err(ProgramError::InvalidArgument);
    }

















}