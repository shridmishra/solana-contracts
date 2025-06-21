use borsh::{BorshDeserialize,BorshSerialize}

#[derive(BorshDeserialize,BorshSerialize,debug)]
pub struct StakingPool {
    pub admin : [u8,32] ,
    pub reward_rate:[u64],
    pub total_staked: [u64],
}
 
  
#[derive(BorshDeserialize,BorshSerialize,debug)]
pub struct UserStakeInfo {
    pub staker: [u8;32];
    pub amount: u8;
    pub last_stake_time:u8;
}
    

