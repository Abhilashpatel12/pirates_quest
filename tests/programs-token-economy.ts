import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenEconomy } from "../target/types/token_economy";
import { expect } from "chai";
import { PublicKey, Keypair } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

describe("🏴‍☠️ Pirates Quest - Token Economy Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenEconomy as Program<TokenEconomy>;
  const authority = provider.wallet as anchor.Wallet;

  let mintKeypair: Keypair;
  let piratePda: PublicKey;
  let playerVault: Keypair;
  let player2Vault: Keypair;
  let player2: Keypair;

  before(async () => {
    console.log("🔧 Setting up test environment...");

    player2 = Keypair.generate();
    const airdropSig = await provider.connection.requestAirdrop(
      player2.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);

    mintKeypair = Keypair.generate();
    await createMint(
      provider.connection,
      authority.payer,
      authority.publicKey,
      null,
      9,
      mintKeypair
    );

    console.log("✅ Mint created:", mintKeypair.publicKey.toBase58());

    [piratePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("pirate"), mintKeypair.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializePirate()
      .accounts({
        pirate: piratePda,
        mint: mintKeypair.publicKey,
        authority: authority.publicKey,
      })
      .rpc();
    console.log("✅ Pirate PDA initialized");

    playerVault = Keypair.generate();
    player2Vault = Keypair.generate();

    // Always sign for new keypairs
    await program.methods
      .initializeVault()
      .accounts({
        vault: playerVault.publicKey,
        owner: authority.publicKey,
      })
      .signers([playerVault])
      .rpc();

    const player2Rpc = new anchor.AnchorProvider(
      provider.connection,
      new anchor.Wallet(player2),
      anchor.AnchorProvider.defaultOptions()
    );

    // Set provider temporarily to get program2 instance
    const originalProvider = anchor.getProvider();
    anchor.setProvider(player2Rpc);
    const program2 = anchor.workspace.TokenEconomy as Program<TokenEconomy>;
    
    await program2.methods
      .initializeVault()
      .accounts({
        vault: player2Vault.publicKey,
        owner: player2.publicKey,
      })
      .signers([player2Vault, player2])
      .rpc();
    
    anchor.setProvider(originalProvider);

    console.log("✅ Test setup complete!\n");
  });

  it("Should reward player for completing level 1", async () => {
    console.log("🎮 Testing level completion reward...");
    const level = 1;
    await program.methods
      .rewardLevelCompletion(level)
      .accounts({
        pirate: piratePda,
        fromVault: playerVault.publicKey,
        authority: authority.publicKey,
      })
      .rpc();
    const vaultData = await program.account.vault.fetch(playerVault.publicKey);
    console.log(`💰 Player earned ${vaultData.balance} PIRATE tokens`);
    expect(vaultData.balance.toNumber()).to.be.greaterThan(0);
  });

  it("Should reward player for completing harder level (Level 10)", async () => {
    console.log("🎮 Testing harder level reward...");
    const level = 10;
    await program.methods
      .rewardLevelCompletion(level)
      .accounts({
        pirate: piratePda,
        fromVault: playerVault.publicKey,
        authority: authority.publicKey,
      })
      .rpc();
    const vaultData = await program.account.vault.fetch(playerVault.publicKey);
    console.log(`💰 Total balance: ${vaultData.balance} PIRATE tokens`);
    expect(vaultData.balance.toNumber()).to.be.greaterThan(10);
  });

  it("Should reward player for finding common treasure", async () => {
    console.log("💎 Testing treasure discovery reward...");
    const treasureType = 1;
    await program.methods
      .rewardTreasureFound(treasureType)
      .accounts({
        pirate: piratePda,
        fromVault: playerVault.publicKey,
        authority: authority.publicKey,
      })
      .rpc();
    const vaultData = await program.account.vault.fetch(playerVault.publicKey);
    console.log(`🏴‍☠️ Treasure found! Balance: ${vaultData.balance} PIRATE`);
    expect(vaultData.balance.toNumber()).to.be.greaterThan(0);
  });

  it("Should give daily login bonus", async () => {
    console.log("📅 Testing daily login reward...");
    const balanceBefore = (await program.account.vault.fetch(playerVault.publicKey)).balance;
    await program.methods
      .rewardDailyLogin()
      .accounts({
        pirate: piratePda,
        fromVault: playerVault.publicKey,
        authority: authority.publicKey,
      })
      .rpc();
    const balanceAfter = (await program.account.vault.fetch(playerVault.publicKey)).balance;
    const dailyBonus = balanceAfter.sub(balanceBefore);
    console.log(`🎁 Daily bonus received: ${dailyBonus} PIRATE tokens`);
    expect(dailyBonus.toNumber()).to.equal(55);
  });

  it("Should transfer tokens between players", async () => {
    console.log("💸 Testing token transfer between players...");
    const transferAmount = new anchor.BN(10);
    await program.methods
      .transferPirateTokens(transferAmount)
      .accounts({
        fromVault: playerVault.publicKey,
        toVault: player2Vault.publicKey,
        authority: authority.publicKey,
      })
      .rpc();
    const player1BalanceAfter = (await program.account.vault.fetch(playerVault.publicKey)).balance;
    const player2Balance = (await program.account.vault.fetch(player2Vault.publicKey)).balance;
    console.log(`👤 Player 1 balance: ${player1BalanceAfter} PIRATE`);
    console.log(`👤 Player 2 balance: ${player2Balance} PIRATE`);
    expect(player2Balance.eq(transferAmount)).to.be.true;
  });

  it("Should burn tokens from player's account", async () => {
    console.log("🔥 Testing token burning...");
    const burnAmount = new anchor.BN(5);
    const balanceBefore = (await program.account.vault.fetch(playerVault.publicKey)).balance;
    await program.methods
      .burnPirateTokens(burnAmount)
      .accounts({
        pirate: piratePda,
        fromVault: playerVault.publicKey,
        authority: authority.publicKey,
      })
      .rpc();
    const balanceAfter = (await program.account.vault.fetch(playerVault.publicKey)).balance;
    const burned = balanceBefore.sub(balanceAfter);
    console.log(`🔥 Burned ${burned} PIRATE tokens`);
    expect(burned.eq(burnAmount)).to.be.true;
  });

  it("Should fail when trying to burn more than balance", async () => {
    console.log("❌ Testing insufficient balance error...");
    const balance = (await program.account.vault.fetch(playerVault.publicKey)).balance;
    const hugeAmount = balance.add(new anchor.BN(100));
    try {
      await program.methods
        .burnPirateTokens(hugeAmount)
        .accounts({
          pirate: piratePda,
          fromVault: playerVault.publicKey,
          authority: authority.publicKey,
        })
        .rpc();
      expect.fail("Should have thrown error");
    } catch (err) {
      console.log("✅ Correctly rejected insufficient balance");
      expect(err.toString()).to.include("InsufficientBalance");
    }
  });

  after(() => {
    console.log("\n🏴‍☠️ All token economy tests completed!");
  });
});
