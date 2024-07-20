import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UncheckedArithmetic0Light } from "../target/types/unchecked_arithmetic_0_light";

describe("unchecked-arithmetic-0-light", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.UncheckedArithmetic0Light as Program<UncheckedArithmetic0Light>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
