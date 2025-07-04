use anchor_lang::prelude::*;

#[account]
pub struct StakingPool {
    pub authority: Pubkey,
    pub reward_rate: u64,
    pub vault: Pubkey,
    pub total_stake: u64,
    pub bump: u8,
}

impl StakingPool {
    pub const INIT_SPACE: usize = 8 + 32 + 8 + 32 + 8 + 1;
}

#[account]
pub struct UserStakeAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub pending_rewards: u64,
    pub last_stake_time: i64,
}

impl UserStakeAccount {
    pub const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 8;
}
