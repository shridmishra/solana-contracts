import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Staking } from "../target/types/staking";
import {
  
  getMinimumBalanceForRentExemptAccount,
  createInitializeAccountInstruction,
  TOKEN_PROGRAM_ID,
  createMint,
} from "@solana/spl-token";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("staking", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Staking as Program<Staking>;

  let mint: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;
  let staking_pool: anchor.web3.PublicKey;
  const authority = provider.wallet;

  it("initialize the staking pool", async () => {
    mint = await createMint(
      provider.connection,
      authority.payer,
      authority.publicKey,
      null,
      6
    );

    [staking_pool] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("staking_pool"), mint.toBuffer()],
      program.programId
    );

    const vaultKeypair = Keypair.generate();
    const lamports = await getMinimumBalanceForRentExemptAccount(
      provider.connection
    );
    const tx = new anchor.web3.Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: authority.publicKey,
        newAccountPubkey: vaultKeypair.publicKey,
        space: 165,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      }),
      createInitializeAccountInstruction(
        vaultKeypair.publicKey,
        mint,
        staking_pool,
        TOKEN_PROGRAM_ID
      )
    );
    await provider.sendAndConfirm(tx, [vaultKeypair]);
    vault = vaultKeypair.publicKey;

    await program.methods
      .initializePool(new anchor.BN(1_000_000))
      .accounts({
        stakingPool: staking_pool,
        authority: authority.publicKey,
        vault,
        mint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      } as any)
      .rpc();

    const poolState = await program.account.stakingPool.fetch(staking_pool);

    assert.ok(poolState.authority.equals(authority.publicKey));
    assert.equal(poolState.rewardRate.toNumber(), 1_000_000);
    assert.ok(poolState.vault.equals(vault));
  });
});
