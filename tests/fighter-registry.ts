import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FighterRegistry } from "../target/types/fighter_registry";
import { expect } from "chai";

describe("fighter_registry", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.FighterRegistry as Program<FighterRegistry>;

  let fighterPda: anchor.web3.PublicKey;
  let fighterBump: number;
  const user = provider.wallet;

  const getFighterPda = async (authority: anchor.web3.PublicKey) => {
    return anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("fighter"), authority.toBuffer()],
      program.programId
    );
  };

  it("Initializes a new Fighter!", async () => {
    [fighterPda, fighterBump] = await getFighterPda(user.publicKey);

    const tx = await program.methods
      .initializefighter("Luffy")
      .accounts({
        fighter: fighterPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Fighter created with tx:", tx);

    // Fetch the created fighter
    const fighterAccount = await program.account.fighter.fetch(fighterPda);

    expect(fighterAccount.name).to.equal("Luffy");
    expect(fighterAccount.authority.toBase58()).to.equal(user.publicKey.toBase58());
    expect(fighterAccount.level).to.equal(1);
  });

  it("Updates the Fighter", async () => {
    await program.methods
      .updatefighter(98, 45, 123, 3, new anchor.BN(0))
      .accounts({
        fighter: fighterPda,
        authority: user.publicKey,
      })
      .rpc();

    const updated = await program.account.fighter.fetch(fighterPda);

    expect(updated.health).to.equal(98);
    expect(updated.stamina).to.equal(45);
    expect(updated.experience).to.equal(123);
    expect(updated.level).to.equal(3);
  });

  it("Deletes the Fighter", async () => {
    await program.methods
      .deletefighter()
      .accounts({
        fighter: fighterPda,
        authority: user.publicKey,
        user: user.publicKey,
      })
      .rpc();

    try {
      await program.account.fighter.fetch(fighterPda);
      // If fetch does not throw, it failed
      throw new Error("Fighter was not deleted");
    } catch (err) {
      expect(err.message).to.include("Account does not exist");
    }
  });
});
