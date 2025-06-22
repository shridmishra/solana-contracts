use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct StakingPool {
    pub admin: [u8; 32],
    pub reward_rate: u64,
    pub total_staked: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserStakeInfo {
    pub staker: [u8; 32],
    pub amount: u64,
    pub last_stake_time: u64,
}
