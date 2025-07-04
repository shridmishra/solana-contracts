use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
    system_instruction,
    sysvar,
    instruction::AccountMeta,
};
use spl_token::{
    instruction::initialize_mint,
    state::Mint,
    id as token_program_id,
};
use borsh::{BorshDeserialize, };
use borsh::BorshSerialize;
use borsh::to_vec;



use staking_contract::{
    process_instruction,
    state::{StakingPool, UserStakeInfo},
    instruction::StakingInstruction,
};

async fn setup_test_env() -> (ProgramTestContext, Pubkey) {
    let program_id = Pubkey::new_unique();

    let program_test = ProgramTest::new(
        "staking_contract", 
        program_id,
        processor!(process_instruction),
    );

    let context = program_test.start_with_context().await;
    (context, program_id)
}

#[tokio::test]
async fn test_initialize_pool() {
    let ( context, program_id) = setup_test_env().await;

    // 1. Create a new account for the staking pool
    let staking_pool = Keypair::new();

    let pool_space = std::mem::size_of::<StakingPool>();
    let rent = context.banks_client.get_rent().await.unwrap();
    let rent_lamports = rent.minimum_balance(pool_space);

    let create_pool_account_ix = system_instruction::create_account(
        &context.payer.pubkey(),
        &staking_pool.pubkey(),
        rent_lamports,
        pool_space as u64,
        &program_id,
    );

    let create_tx = Transaction::new_signed_with_payer(
        &[create_pool_account_ix],
        Some(&context.payer.pubkey()),
        &[&context.payer, &staking_pool],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(create_tx).await.unwrap();

    // 2. Create the `InitializePool` instruction
let instruction_data = borsh::to_vec(&StakingInstruction::InitializePool { reward_rate: 5 }).unwrap();



    let ix = solana_sdk::instruction::Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(staking_pool.pubkey(), false),
            AccountMeta::new(context.payer.pubkey(), true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: instruction_data,
    };

    // 3. Send the instruction
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context.banks_client.process_transaction(tx).await.unwrap();

    // 4. Fetch and deserialize the staking pool account
    let account = context
        .banks_client
        .get_account(staking_pool.pubkey())
        .await
        .unwrap()
        .unwrap();

    let pool = StakingPool::try_from_slice(&account.data).unwrap();

    // 5. Assert the data was correctly initialized
    assert_eq!(pool.reward_rate, 5);
    assert_eq!(pool.total_staked, 0);
    assert_eq!(pool.admin, context.payer.pubkey().to_bytes());
}











