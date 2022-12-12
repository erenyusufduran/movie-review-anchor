import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BuildPda } from "../target/types/build_pda";

describe("build-pda", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BuildPda as Program<BuildPda>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
