import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ArbitraryLimitInputs5 } from "../target/types/arbitrary_limit_inputs_5";

describe("arbitrary-limit-inputs-5", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ArbitraryLimitInputs5 as Program<ArbitraryLimitInputs5>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
