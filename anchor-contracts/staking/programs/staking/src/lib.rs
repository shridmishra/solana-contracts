use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};


pub mod constants;
pub mod context;
pub mod state;
pub mod utils;

use context::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg5uVhRxg9bC"); 

#[program]

pub mod staking {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, reward_rate: u64) -> Result<()> {
        let bump = ctx.accounts.staking_pool.bump;
        let staking_pool = &mut ctx.accounts.staking_pool;

        staking_pool.authority = ctx.accounts.authority.key();
        staking_pool.reward_rate = reward_rate;
        staking_pool.vault = ctx.accounts.vault.key();
        staking_pool.total_stake = 0;
        staking_pool.bump = bump;

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let user = &ctx.accounts.user;
        let user_token_account = &ctx.accounts.user_token_account;
        let vault = &ctx.accounts.vault;
        let staking_pool = &mut ctx.accounts.staking_pool;
        let user_stake = &mut ctx.accounts.user_stake_account;

        if user_token_account.owner != user.key() {
            return Err(ProgramError::IllegalOwner.into());
        }
        if user_token_account.mint != ctx.accounts.mint.key() {
            return Err(ProgramError::InvalidAccountData.into());
        }
        if vault.owner != staking_pool.key() {
            return Err(ProgramError::InvalidAccountData.into());
        }

        if user_stake.amount > 0 {
            let now = Clock::get()?.unix_timestamp;
            let elapsed_time = now - user_stake.last_stake_time;
            let rewards = elapsed_time as u64 * staking_pool.reward_rate * user_stake.amount;
            user_stake.pending_rewards += rewards;
        }

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: user_token_account.to_account_info(),
                to: vault.to_account_info(),
                authority: user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount)?;

        user_stake.amount += amount;
        user_stake.last_stake_time = Clock::get()?.unix_timestamp;
        user_stake.owner = user.key();
        staking_pool.total_stake += amount;

        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        let user_token_account = &ctx.accounts.user_token_account;
        let vault = &ctx.accounts.vault;
        let staking_pool = &mut ctx.accounts.staking_pool;
        let user_stake_account = &mut ctx.accounts.user_stake_account;

        if amount > user_stake_account.amount {
            return Err(ProgramError::InvalidArgument.into());
        }

        let now = Clock::get()?.unix_timestamp;
        let elapsed_time = now - user_stake_account.last_stake_time;
        let rewards = elapsed_time as u64 * staking_pool.reward_rate * user_stake_account.amount;
        user_stake_account.pending_rewards += rewards;

        user_stake_account.amount -= amount;
        staking_pool.total_stake -= amount;
        user_stake_account.last_stake_time = now;

        let staking_pool_key = staking_pool.key();
        let bump = &[staking_pool.bump];
        let signer_seeds = &[&[b"vault", staking_pool_key.as_ref(), bump][..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: vault.to_account_info(),
                to: user_token_account.to_account_info(),
                authority: staking_pool.to_account_info(),
            },
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let vault = &ctx.accounts.vault;
        let user_token_account = &ctx.accounts.user_token_account;
        let user_stake_account = &mut ctx.accounts.user_stake_account;
        let staking_pool = &mut ctx.accounts.staking_pool;

        if user_stake_account.pending_rewards == 0 {
            return Err(ProgramError::InvalidArgument.into());
        }

        let vault_key = vault.key();
        let bump = &[staking_pool.bump];
        let signer_seeds = &[&[b"staking_pool", vault_key.as_ref(), bump][..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: mint.to_account_info(),
                to: user_token_account.to_account_info(),
                authority: staking_pool.to_account_info(),
            },
            signer_seeds,
        );

        let amount = user_stake_account.pending_rewards;
        token::mint_to(cpi_ctx, amount)?;
        user_stake_account.pending_rewards = 0;

        Ok(())
    }
}
