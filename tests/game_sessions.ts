import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { GameSession } from "../target/types/game_session";
import { expect } from "chai";

describe("game_session", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GameSession as Program<GameSession>;

  const pvp = { pvp: {} };
  const playerAWon = { playerAWon: {} };

  it("Creates and ends a game session", async () => {
    const sessionId = new anchor.BN(Date.now());

    const [gameSessionPda, bump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("game_session"), sessionId.toArrayLike(Buffer, "le", 8)],
      program.programId,
    );

    const creator = provider.wallet;
    const playerB = web3.Keypair.generate().publicKey;

    await program.methods
      .startSession(sessionId, playerB, pvp)
      .accounts({
        gameSession: gameSessionPda,
        creator: creator.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    const sessionAccount = await program.account.gameSession.fetch(gameSessionPda);

    console.log("Session started:", sessionAccount);

    expect(sessionAccount.creator.toBase58()).to.equal(creator.publicKey.toBase58());
    expect(sessionAccount.playerB.toBase58()).to.equal(playerB.toBase58());
    expect(sessionAccount.isActive).to.be.true;
    expect(sessionAccount.result.ongoing).to.exist;

    await program.methods
      .endSession(playerAWon)
      .accounts({
        gameSession: gameSessionPda,
        creator: creator.publicKey,
      })
      .rpc();

    const endedSession = await program.account.gameSession.fetch(gameSessionPda);
    expect(endedSession.isActive).to.be.false;
    expect(endedSession.result.playerAWon).to.exist;
    // Fixed assertion:
    expect(endedSession.endTime.toNumber()).to.be.greaterThanOrEqual(sessionAccount.startTime.toNumber());

    console.log("Session ended successfully:", endedSession);
  });
});
