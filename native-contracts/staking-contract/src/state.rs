use borsh::{BorshDeserialize,BorshSerialize}

#[derive(BorshDeserialize,BorshSerialize,debug)]
pub struct StakingPool {
    pub admin : [u8,32] ,
    pub reward_rate:[u64],
    pub total_staked: [u64],


}
 
  


    

