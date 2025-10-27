import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenEconomy } from "../target/types/token_economy";
import { expect } from "chai";
import { PublicKey, Keypair } from "@solana/web3.js";
import { 
  TOKEN_PROGRAM_ID, 
  createMint, 
  getOrCreateAssociatedTokenAccount,
  mintTo 
} from "@solana/spl-token";

describe("🏴‍☠️ Pirates Quest - Token Economy Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenEconomy as Program<TokenEconomy>;
  const authority = provider.wallet as anchor.Wallet;
  
  let mintKeypair: Keypair;
  let piratePda: PublicKey;
  let playerVault: any;
  let player2Vault: any;
  let player2: Keypair;

  before(async () => {
    console.log("🔧 Setting up test environment...");
    
    // Create a test player wallet
    player2 = Keypair.generate();
    
    // Airdrop SOL to player2 for testing
    const airdropSig = await provider.connection.requestAirdrop(
      player2.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
    
    // Create token mint
    mintKeypair = Keypair.generate();
    await createMint(
      provider.connection,
      authority.payer,
      authority.publicKey,
      null,
      9, // 9 decimals like most tokens
      mintKeypair
    );
    
    console.log("✅ Mint created:", mintKeypair.publicKey.toBase58());
    
    // Derive Pirate PDA
    [piratePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("pirate"), mintKeypair.publicKey.toBuffer()],
      program.programId
    );
    
    // Create token accounts for players
    playerVault = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      authority.payer,
      mintKeypair.publicKey,
      authority.publicKey
    );
    
    player2Vault = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      authority.payer,
      mintKeypair.publicKey,
      player2.publicKey
    );
    
    console.log("✅ Test setup complete!\n");
  });

  it("Should reward player for completing level 1", async () => {
    console.log("🎮 Testing level completion reward...");
    
    const level = 1;
    const expectedReward = 100; // Level 1 gives 100 tokens
    
    await program.methods
      .rewardLevelCompletion(level)
      .accounts({
        pirate: piratePda,
        playerVault: playerVault.address,
        authority: authority.publicKey,
        mint: mintKeypair.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    
    const vaultBalance = await provider.connection.getTokenAccountBalance(
      playerVault.address
    );
    
    console.log(`💰 Player earned ${vaultBalance.value.uiAmount} PIRATE tokens`);
    expect(Number(vaultBalance.value.amount)).to.be.greaterThan(0);
  });

  it("Should reward player for completing harder level (Level 10)", async () => {
    console.log("🎮 Testing harder level reward...");
    
    const level = 10;
    
    await program.methods
      .rewardLevelCompletion(level)
      .accounts({
        pirate: piratePda,
        playerVault: playerVault.address,
        authority: authority.publicKey,
        mint: mintKeypair.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    
    const vaultBalance = await provider.connection.getTokenAccountBalance(
      playerVault.address
    );
    
    console.log(`💰 Total balance: ${vaultBalance.value.uiAmount} PIRATE tokens`);
    expect(Number(vaultBalance.value.amount)).to.be.greaterThan(100);
  });

  it("Should reward player for finding common treasure", async () => {
    console.log("💎 Testing treasure discovery reward...");
    
    const treasureType = 1; // Common treasure
    
    await program.methods
      .rewardTreasureFound(treasureType)
      .accounts({
        pirate: piratePda,
        playerVault: playerVault.address,
        authority: authority.publicKey,
        mint: mintKeypair.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    
    const vaultBalance = await provider.connection.getTokenAccountBalance(
      playerVault.address
    );
    
    console.log(`🏴‍☠️ Treasure found! Balance: ${vaultBalance.value.uiAmount} PIRATE`);
    expect(vaultBalance.value.uiAmount).to.be.greaterThan(0);
  });

  it("Should give daily login bonus", async () => {
    console.log("📅 Testing daily login reward...");
    
    const balanceBefore = await provider.connection.getTokenAccountBalance(
      playerVault.address
    );
    
    await program.methods
      .rewardDailyLogin()
      .accounts({
        pirate: piratePda,
        playerVault: playerVault.address,
        authority: authority.publicKey,
        mint: mintKeypair.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    
    const balanceAfter = await provider.connection.getTokenAccountBalance(
      playerVault.address
    );
    
    const dailyBonus = Number(balanceAfter.value.amount) - Number(balanceBefore.value.amount);
    console.log(`🎁 Daily bonus received: ${dailyBonus / 1e9} PIRATE tokens`);
    expect(dailyBonus).to.equal(50 * 1e9); // 50 tokens with 9 decimals
  });

  it("Should transfer tokens between players", async () => {
    console.log("💸 Testing token transfer between players...");
    
    const transferAmount = 100;
    
    const player1BalanceBefore = await provider.connection.getTokenAccountBalance(
      playerVault.address
    );
    
    await program.methods
      .transferPirateTokens(new anchor.BN(transferAmount * 1e9))
      .accounts({
        pirate: piratePda,
        fromVault: playerVault.address,
        toVault: player2Vault.address,
        authority: authority.publicKey,
        mint: mintKeypair.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    
    const player1BalanceAfter = await provider.connection.getTokenAccountBalance(
      playerVault.address
    );
    const player2Balance = await provider.connection.getTokenAccountBalance(
      player2Vault.address
    );
    
    console.log(`👤 Player 1 balance: ${player1BalanceAfter.value.uiAmount} PIRATE`);
    console.log(`👤 Player 2 balance: ${player2Balance.value.uiAmount} PIRATE`);
    
    expect(player2Balance.value.uiAmount).to.equal(transferAmount);
  });

  it("Should burn tokens from player's account", async () => {
    console.log("🔥 Testing token burning...");
    
    const burnAmount = 50;
    const balanceBefore = await provider.connection.getTokenAccountBalance(
      playerVault.address
    );
    
    await program.methods
      .burnPirateTokens(new anchor.BN(burnAmount * 1e9))
      .accounts({
        pirate: piratePda,
        fromVault: playerVault.address,
        authority: authority.publicKey,
        mint: mintKeypair.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    
    const balanceAfter = await provider.connection.getTokenAccountBalance(
      playerVault.address
    );
    
    const burned = Number(balanceBefore.value.amount) - Number(balanceAfter.value.amount);
    console.log(`🔥 Burned ${burned / 1e9} PIRATE tokens`);
    
    expect(burned).to.equal(burnAmount * 1e9);
  });

  it("Should fail when trying to burn more than balance", async () => {
    console.log("❌ Testing insufficient balance error...");
    
    const hugeAmount = 999999999;
    
    try {
      await program.methods
        .burnPirateTokens(new anchor.BN(hugeAmount * 1e9))
        .accounts({
          pirate: piratePda,
          fromVault: playerVault.address,
          authority: authority.publicKey,
          mint: mintKeypair.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      
      // Should not reach here
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
