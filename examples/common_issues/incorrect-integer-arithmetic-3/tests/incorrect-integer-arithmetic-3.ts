import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IncorrectIntegerArithmetic3 } from "../target/types/incorrect_integer_arithmetic_3";

describe("incorrect-integer-arithmetic-3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.IncorrectIntegerArithmetic3 as Program<IncorrectIntegerArithmetic3>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
