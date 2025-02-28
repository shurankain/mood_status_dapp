import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MoodStatusDapp } from "../target/types/mood_status_dapp";
import { Keypair, SystemProgram } from "@solana/web3.js";

describe("mood_status_dapp", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MoodStatusDapp as Program<MoodStatusDapp>;

  const baseAccount = Keypair.generate();

  it("Initializes the mood", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      } as any)
      .signers([baseAccount])
      .rpc();

    console.log("Transaction Signature (Initialize):", tx);

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("Mood initialized with value:", account.mood.toString());
  });

  it("Updates the mood", async () => {
    const tx = await program.methods
      .update("Joyful")
      .accounts({
        baseAccount: baseAccount.publicKey,
      })
      .rpc();

    console.log("Transaction Signature (Update):", tx);

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey);
    console.log("Mood updated to: ", account.mood.toString());
  });
});