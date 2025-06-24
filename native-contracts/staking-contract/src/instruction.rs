use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum StakingInstruction {
    InitializePool { reward_rate: u64 },
    Stake { amount: u64 },
    UnStake { amount: u64 },
    ClaimRewards,
}
