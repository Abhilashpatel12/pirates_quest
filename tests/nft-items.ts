import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftItems } from "../target/types/nft_items";
import { expect } from "chai";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("Pirates Quest NFT Items", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftItems as Program<NftItems>;
  const wallet = provider.wallet as anchor.Wallet;

  // Test accounts
  let collectionPda: PublicKey;
  let collectionBump: number;
  let collectionMint: Keypair;
  let assetKeypair: Keypair;
  let gameItemPda: PublicKey;

  // MPL Core Program ID
  const MPL_CORE_PROGRAM_ID = new PublicKey(
    "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
  );

  before(async () => {
    // Find collection PDA
    [collectionPda, collectionBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("collection")],
      program.programId
    );

    collectionMint = Keypair.generate();
  });

  describe("Collection Initialization", () => {
    it("should initialize the Pirates Quest collection", async () => {
      const collectionName = "Pirates Quest Legendary Treasure";
      const collectionUri = "https://arweave.net/collection-metadata.json";

      const tx = await program.methods
        .initializeCollection(collectionName, collectionUri)
        .accounts({
          collection: collectionPda,
          authority: wallet.publicKey,
          collectionMint: collectionMint.publicKey,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([collectionMint])
        .rpc();

      console.log("Collection initialized:", tx);

      // Fetch and verify collection account
      const collectionAccount = await program.account.collection.fetch(
        collectionPda
      );

      expect(collectionAccount.authority.toString()).to.equal(
        wallet.publicKey.toString()
      );
      expect(collectionAccount.totalMinted.toNumber()).to.equal(0);
      expect(collectionAccount.name).to.equal(collectionName);
      expect(collectionAccount.uri).to.equal(collectionUri);
    });

    it("should fail if collection already initialized", async () => {
      try {
        await program.methods
          .initializeCollection("Duplicate", "https://example.com")
          .accounts({
            collection: collectionPda,
            authority: wallet.publicKey,
            collectionMint: collectionMint.publicKey,
            mplCoreProgram: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error).to.exist;
      }
    });
  });

  describe("Minting Game Items", () => {
    beforeEach(() => {
      // Generate fresh keypair for each test
      assetKeypair = Keypair.generate();
      [gameItemPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("game_item"), assetKeypair.publicKey.toBuffer()],
        program.programId
      );
    });

    it("should mint a legendary weapon (Sword of Blackwater)", async () => {
      const itemName = "Legendary Sword of Blackwater";
      const itemUri = "https://arweave.net/sword-metadata.json";
      const itemType = { weapon: {} }; // ItemType enum
      const rarity = 5; // Legendary
      const stats = {
        attackPower: 500,
        defense: 100,
        speedBoost: 50,
        specialAbility: 1, // Fire damage
      };

      const tx = await program.methods
        .mintGameItem(itemName, itemUri, itemType, rarity, stats)
        .accounts({
          collection: collectionPda,
          collectionMint: collectionMint.publicKey,
          gameItem: gameItemPda,
          asset: assetKeypair.publicKey,
          payer: wallet.publicKey,
          owner: wallet.publicKey,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([assetKeypair])
        .rpc();

      console.log("Weapon minted:", tx);

      // Fetch game item and verify
      const gameItem = await program.account.gameItem.fetch(gameItemPda);

      expect(gameItem.asset.toString()).to.equal(
        assetKeypair.publicKey.toString()
      );
      expect(gameItem.owner.toString()).to.equal(wallet.publicKey.toString());
      expect(gameItem.rarity).to.equal(rarity);
      expect(gameItem.level).to.equal(1);
      expect(gameItem.experience.toNumber()).to.equal(0);
      expect(gameItem.stats.attackPower).to.equal(500);
      expect(gameItem.isEquipped).to.be.false;
      expect(gameItem.isListed).to.be.false;

      // Verify collection counter increased
      const collection = await program.account.collection.fetch(collectionPda);
      expect(collection.totalMinted.toNumber()).to.equal(1);
    });

    it("should mint a pirate ship with speed boost", async () => {
      const itemName = "Stormbreaker Galleon";
      const itemUri = "https://arweave.net/ship-metadata.json";
      const itemType = { ship: {} };
      const rarity = 4; // Epic
      const stats = {
        attackPower: 200,
        defense: 300,
        speedBoost: 500, // Fastest ship
        specialAbility: 2, // Storm immunity
      };

      const tx = await program.methods
        .mintGameItem(itemName, itemUri, itemType, rarity, stats)
        .accounts({
          collection: collectionPda,
          collectionMint: collectionMint.publicKey,
          gameItem: gameItemPda,
          asset: assetKeypair.publicKey,
          payer: wallet.publicKey,
          owner: wallet.publicKey,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([assetKeypair])
        .rpc();

      console.log("Ship minted:", tx);

      const gameItem = await program.account.gameItem.fetch(gameItemPda);
      expect(gameItem.stats.speedBoost).to.equal(500);
      expect(gameItem.stats.defense).to.equal(300);
    });

    it("should mint a cosmetic outfit with no combat stats", async () => {
      const itemName = "Pirate King's Hat";
      const itemUri = "https://arweave.net/hat-metadata.json";
      const itemType = { outfit: {} };
      const rarity = 3; // Rare
      const stats = {
        attackPower: 0,
        defense: 0,
        speedBoost: 0,
        specialAbility: 0, // Pure cosmetic
      };

      const tx = await program.methods
        .mintGameItem(itemName, itemUri, itemType, rarity, stats)
        .accounts({
          collection: collectionPda,
          collectionMint: collectionMint.publicKey,
          gameItem: gameItemPda,
          asset: assetKeypair.publicKey,
          payer: wallet.publicKey,
          owner: wallet.publicKey,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([assetKeypair])
        .rpc();

      console.log("Outfit minted:", tx);

      const gameItem = await program.account.gameItem.fetch(gameItemPda);
      expect(gameItem.stats.attackPower).to.equal(0);
      expect(gameItem.stats.defense).to.equal(0);
    });

    it("should fail to mint with invalid rarity (0)", async () => {
      try {
        await program.methods
          .mintGameItem(
            "Invalid Item",
            "https://example.com",
            { weapon: {} },
            0, // Invalid
            { attackPower: 10, defense: 10, speedBoost: 10, specialAbility: 0 }
          )
          .accounts({
            collection: collectionPda,
            collectionMint: collectionMint.publicKey,
            gameItem: gameItemPda,
            asset: assetKeypair.publicKey,
            payer: wallet.publicKey,
            owner: wallet.publicKey,
            mplCoreProgram: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([assetKeypair])
          .rpc();

        expect.fail("Should have failed with invalid rarity");
      } catch (error: any) {
        expect(error.error.errorMessage).to.include("Rarity must be between");
      }
    });

    it("should fail to mint with invalid rarity (6)", async () => {
      try {
        await program.methods
          .mintGameItem(
            "Invalid Item",
            "https://example.com",
            { weapon: {} },
            6, // Invalid
            { attackPower: 10, defense: 10, speedBoost: 10, specialAbility: 0 }
          )
          .accounts({
            collection: collectionPda,
            collectionMint: collectionMint.publicKey,
            gameItem: gameItemPda,
            asset: assetKeypair.publicKey,
            payer: wallet.publicKey,
            owner: wallet.publicKey,
            mplCoreProgram: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([assetKeypair])
          .rpc();

        expect.fail("Should have failed with invalid rarity");
      } catch (error: any) {
        expect(error.error.errorMessage).to.include("Rarity must be between");
      }
    });
  });

  describe("Boss Drop Minting", () => {
    it("should mint a boss drop with proof", async () => {
      assetKeypair = Keypair.generate();
      [gameItemPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("game_item"), assetKeypair.publicKey.toBuffer()],
        program.programId
      );

      const itemName = "Phoenix Feather Cannon";
      const itemUri = "https://arweave.net/boss-drop-metadata.json";
      const itemType = { artifact: {} };
      const rarity = 5; // Boss drops are always legendary
      const stats = {
        attackPower: 800,
        defense: 200,
        speedBoost: 100,
        specialAbility: 3, // Fire magic
      };
      const bossProof = {
        bossId: 3, // Volcanic boss
        defeatTimestamp: new anchor.BN(Date.now() / 1000),
        player: wallet.publicKey,
      };

      const tx = await program.methods
        .mintBossDrop(itemName, itemUri, itemType, rarity, stats, bossProof)
        .accounts({
          collection: collectionPda,
          collectionMint: collectionMint.publicKey,
          gameItem: gameItemPda,
          asset: assetKeypair.publicKey,
          payer: wallet.publicKey,
          owner: wallet.publicKey,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([assetKeypair])
        .rpc();

      console.log("Boss drop minted:", tx);

      const gameItem = await program.account.gameItem.fetch(gameItemPda);
      expect(gameItem.bossProof).to.not.be.null;
      expect(gameItem.bossProof.bossId).to.equal(3);
      expect(gameItem.rarity).to.equal(5);
    });

    it("should fail to mint boss drop with non-legendary rarity", async () => {
      assetKeypair = Keypair.generate();
      [gameItemPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("game_item"), assetKeypair.publicKey.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .mintBossDrop(
            "Fake Boss Drop",
            "https://example.com",
            { artifact: {} },
            3, // Not legendary
            {
              attackPower: 100,
              defense: 100,
              speedBoost: 100,
              specialAbility: 0,
            },
            {
              bossId: 1,
              defeatTimestamp: new anchor.BN(Date.now() / 1000),
              player: wallet.publicKey,
            }
          )
          .accounts({
            collection: collectionPda,
            collectionMint: collectionMint.publicKey,
            gameItem: gameItemPda,
            asset: assetKeypair.publicKey,
            payer: wallet.publicKey,
            owner: wallet.publicKey,
            mplCoreProgram: MPL_CORE_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          })
          .signers([assetKeypair])
          .rpc();

        expect.fail("Should have failed");
      } catch (error: any) {
        expect(error.error.errorMessage).to.include("legendary");
      }
    });
  });

  describe("Treasury Drop Minting", () => {
    it("should mint a treasury drop for game completion", async () => {
      assetKeypair = Keypair.generate();
      [gameItemPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("game_item"), assetKeypair.publicKey.toBuffer()],
        program.programId
      );

      const itemName = "Crown of the Pirate King";
      const itemUri = "https://arweave.net/treasury-metadata.json";
      const itemType = { artifact: {} };
      const rarity = 5;
      const stats = {
        attackPower: 1000,
        defense: 500,
        speedBoost: 200,
        specialAbility: 99, // Ultimate power
      };
      const treasuryProof = {
        claimTimestamp: new anchor.BN(Date.now() / 1000),
        player: wallet.publicKey,
        allIslandsConquered: true,
        finalBattleScore: 9999,
      };

      const tx = await program.methods
        .mintTreasuryDrop(
          itemName,
          itemUri,
          itemType,
          rarity,
          stats,
          treasuryProof
        )
        .accounts({
          collection: collectionPda,
          collectionMint: collectionMint.publicKey,
          gameItem: gameItemPda,
          asset: assetKeypair.publicKey,
          payer: wallet.publicKey,
          owner: wallet.publicKey,
          mplCoreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([assetKeypair])
        .rpc();

      console.log("Treasury drop minted:", tx);

      const gameItem = await program.account.gameItem.fetch(gameItemPda);
      expect(gameItem.treasuryProof).to.not.be.null;
      expect(gameItem.treasuryProof.finalBattleScore).to.equal(9999);
      expect(gameItem.stats.attackPower).to.equal(1000);
    });
  });
});
