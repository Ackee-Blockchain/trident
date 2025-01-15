import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ArbitraryCustomTypes4 } from "../target/types/arbitrary_custom_types_4";

describe("arbitrary-custom-types-4", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ArbitraryCustomTypes4 as Program<ArbitraryCustomTypes4>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
