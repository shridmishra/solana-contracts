
use crate::{
    error::StakingError,
    instruction::StakingInstruction,
    state::{StakingPool, UserStakeInfo},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use spl_token::instruction::transfer;

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StakingInstruction::try_from_slice(instruction_data)?;

    match instruction {
        StakingInstruction::InitializePool { reward_rate } => {
            process_initialize_pool(accounts, reward_rate, program_id)
        }
        StakingInstruction::Stake { amount } => {
            process_stake(accounts, amount, program_id)
        }
        StakingInstruction::UnStake { amount } => {
            process_unstake(accounts, amount, program_id)
        }
        StakingInstruction::ClaimRewards => {
            process_claim_rewards(accounts, program_id)
        }
    }
}


pub fn process_initialize_pool(
    accounts: &[AccountInfo],
    reward_rate: u64,
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let admin_account = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    let rent = &Rent::from_account_info(rent_sysvar)?;
    if !rent.is_exempt(pool_account.lamports(), pool_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    if pool_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let pool_data = StakingPool::try_from_slice(&pool_account.data.borrow())?;
    if pool_data.total_staked != 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let pool = StakingPool {
        admin: admin_account.key.to_bytes(),
        reward_rate,
        total_staked: 0,
    };

    pool.serialize(&mut &mut pool_account.data.borrow_mut()[..])?;
    msg!("Staking pool initialized");
    Ok(())
}

pub fn process_stake(accounts: &[AccountInfo], amount: u64, program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let vault_account = next_account_info(account_info_iter)?;
    let user_wallet = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let user_stake_info_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    let (expected_pda, bump) = Pubkey::find_program_address(
        &[
            b"user-stake",
            user_wallet.key.as_ref(),
            pool_account.key.as_ref(),
        ],
        program_id,
    );

    if user_stake_info_account.key != &expected_pda {
        return Err(ProgramError::InvalidArgument);
    }

    if user_stake_info_account.data_is_empty() {
        let rent = Rent::from_account_info(rent_sysvar)?;
        let space = std::mem::size_of::<UserStakeInfo>();
        let lamports = rent.minimum_balance(space);

        invoke_signed(
            &system_instruction::create_account(
                user_wallet.key,
                &expected_pda,
                lamports,
                space as u64,
                program_id,
            ),
            &[
                user_wallet.clone(),
                user_stake_info_account.clone(),
                system_program.clone(),
            ],
            &[&[
                b"user-stake",
                user_wallet.key.as_ref(),
                pool_account.key.as_ref(),
                &[bump],
            ]],
        )?;
    }

    let ix = transfer(
        token_program.key,
        user_token_account.key,
        vault_account.key,
        user_wallet.key,
        &[],
        amount,
    )?;

    invoke(
        &ix,
        &[
            user_token_account.clone(),
            user_wallet.clone(),
            vault_account.clone(),
            user_wallet.clone(),
        ],
    )?;
    let clock = Clock::get()?;
    let user_stake_info = UserStakeInfo {
        staker: user_wallet.key.to_bytes(),
        amount,
        last_stake_time: clock.unix_timestamp as u64,
    };

    let mut pool_data = StakingPool::try_from_slice(&pool_account.data.borrow())?;
    pool_data.total_staked += amount;
    pool_data.serialize(&mut &mut pool_account.data.borrow_mut()[..])?;

    user_stake_info.serialize(&mut &mut user_stake_info_account.data.borrow_mut()[..])?;
    msg!("User staked {} tokens at {}", amount, clock.unix_timestamp);
    Ok(())
}

pub fn process_unstake(
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

    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[
            b"user-stake",
            user_wallet.key.as_ref(),
            pool_account.key.as_ref(),
        ],
        program_id,
    );

    if user_stake_info_account.key != &expected_pda {
        return Err(StakingError::InvalidPda.into());
    }

    let mut stake_info = UserStakeInfo::try_from_slice(&user_stake_info_account.data.borrow())?;
    if stake_info.amount < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    let (vault_authority, vault_bump) = Pubkey::find_program_address(&[b"vault-auth"], program_id);

    let ix = transfer(
        token_program.key,
        vault_account.key,
        user_token_account.key,
        &vault_authority,
        &[],
        amount,
    )?;

    invoke_signed(
        &ix,
        &[
            vault_account.clone(),
            user_token_account.clone(),
            token_program.clone(),
        ],
        &[&[b"vault-auth", &[vault_bump]]],
    )?;

    stake_info.amount -= amount;
    stake_info.serialize(&mut &mut user_stake_info_account.data.borrow_mut()[..])?;

    let mut pool = StakingPool::try_from_slice(&pool_account.data.borrow())?;
    pool.total_staked -= amount;
    pool.serialize(&mut &mut pool_account.data.borrow_mut()[..])?;

    if stake_info.amount == 0 {
        **user_wallet.lamports.borrow_mut() += **user_stake_info_account.lamports.borrow();
        **user_stake_info_account.lamports.borrow_mut() = 0;

        let mut data = user_stake_info_account.data.borrow_mut();
        for byte in data.iter_mut() {
            *byte = 0;
        }
    }

    Ok(())
}

pub fn process_claim_rewards(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let pool_account = next_account_info(account_info_iter)?;
    let reward_vault = next_account_info(account_info_iter)?;
    let user_wallet = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let user_stake_info_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[b"user-stake", user_wallet.key.as_ref(), pool_account.key.as_ref()],
        program_id,
    );

    if user_stake_info_account.key != &expected_pda {
        return Err(StakingError::InvalidPda.into());
    }

    let mut stake_info = UserStakeInfo::try_from_slice(&user_stake_info_account.data.borrow())?;
    let pool = StakingPool::try_from_slice(&pool_account.data.borrow())?;
    let clock = Clock::from_account_info(clock_sysvar)?;
    let now = clock.unix_timestamp as u64;

    if now <= stake_info.last_stake_time {
        return Err(ProgramError::InvalidArgument);
    }

    let duration = now - stake_info.last_stake_time;
    let reward = duration
        .checked_mul(pool.reward_rate)
        .and_then(|r| r.checked_mul(stake_info.amount))
        .ok_or(ProgramError::InvalidArgument)?;

    if reward == 0 {
        return Err(StakingError::NothingToClaim.into());
    }

    let (vault_authority, vault_bump) =
        Pubkey::find_program_address(&[b"vault-auth"], program_id);

    let ix = transfer(
        token_program.key,
        reward_vault.key,
        user_token_account.key,
        &vault_authority,
        &[],
        reward,
    )?;

    invoke_signed(
        &ix,
        &[
            reward_vault.clone(),
            user_token_account.clone(),
            token_program.clone(),
        ],
        &[&[b"vault-auth", &[vault_bump]]],
    )?;

    stake_info.last_stake_time = now;
    stake_info.serialize(&mut &mut user_stake_info_account.data.borrow_mut()[..])?;

    Ok(())
}
