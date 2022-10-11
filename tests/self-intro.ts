import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SelfIntro } from "../target/types/self_intro";

describe("self-intro", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SelfIntro as Program<SelfIntro>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
