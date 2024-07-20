import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ArbitraryLimitInputs5Light } from "../target/types/arbitrary_limit_inputs_5_light";

describe("arbitrary-limit-inputs-5-light", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ArbitraryLimitInputs5Light as Program<ArbitraryLimitInputs5Light>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
