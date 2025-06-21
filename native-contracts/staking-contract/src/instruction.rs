use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, debug)]

pub enum StakingInstruction {
    InitializePool { reward_rate: u64 },
    Stake { amount: u64 },
}
