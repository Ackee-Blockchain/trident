import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FuzzExample2 } from "../target/types/fuzz_example2";
import { assert } from "chai";


const ESCROW_SEED = "escrow_seed";


describe("fuzz_example2", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.FuzzExample2 as Program<FuzzExample2>;


  const author = anchor.web3.Keypair.generate();
  const receiver = anchor.web3.Keypair.generate();
  const hacker = anchor.web3.Keypair.generate();

  const amount = new anchor.BN(100);

  it("Initialize", async () => {
    await airdrop(provider.connection, author.publicKey);

    const [escrow, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync([
      author.publicKey.toBuffer(),
      receiver.publicKey.toBuffer(),
      anchor.utils.bytes.utf8.encode(ESCROW_SEED),
    ], program.programId)




    await program.methods.initialize(receiver.publicKey, amount).accounts({
      author: author.publicKey,
      escrow: escrow,
    }).signers([author]).rpc({ commitment: "confirmed" });


    const escrowData = await program.account.escrow.fetch(escrow);
    const escorwBalance = await provider.connection.getBalance(escrow);

    const escrowRent = await provider.connection.getMinimumBalanceForRentExemption(program.account.escrow.size);

    assert.strictEqual(escrowData.author.toString(), author.publicKey.toString());
    assert.strictEqual(escrowData.amount.toString(), amount.toString());
    assert.strictEqual(escrowData.receiver.toString(), receiver.publicKey.toString());
    assert.strictEqual(escorwBalance.toString(), amount.addn(escrowRent).toString());

  });

  it.skip("Withdraw", async () => {
    await airdrop(provider.connection, receiver.publicKey);

    const [escrow, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync([
      author.publicKey.toBuffer(),
      receiver.publicKey.toBuffer(),
      anchor.utils.bytes.utf8.encode(ESCROW_SEED),
    ], program.programId)

    const receiverBalanceBefore = await provider.connection.getBalance(receiver.publicKey);


    await program.methods.withdraw().accounts({
      receiver: receiver.publicKey,
      escrow: escrow
    }).signers([receiver]).rpc({ commitment: "confirmed" });


    const escrowRent = await provider.connection.getMinimumBalanceForRentExemption(program.account.escrow.size);
    const receiverBalanceAfter = await provider.connection.getBalance(receiver.publicKey);

    assert.strictEqual((receiverBalanceAfter - receiverBalanceBefore).toString(), amount.addn(escrowRent).toString());

  });

  it("Withdraw Hacker", async () => {
    await airdrop(provider.connection, hacker.publicKey);

    const [escrow, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync([
      author.publicKey.toBuffer(),
      receiver.publicKey.toBuffer(),
      anchor.utils.bytes.utf8.encode(ESCROW_SEED),
    ], program.programId)

    const hackerBalanceBefore = await provider.connection.getBalance(hacker.publicKey);


    await program.methods.withdraw().accounts({
      receiver: hacker.publicKey,
      escrow: escrow
    }).signers([hacker]).rpc({ commitment: "confirmed" });


    const escrowRent = await provider.connection.getMinimumBalanceForRentExemption(program.account.escrow.size);
    const hackerBalanceAfter = await provider.connection.getBalance(hacker.publicKey);

    assert.strictEqual((hackerBalanceAfter - hackerBalanceBefore).toString(), amount.addn(escrowRent).toString());

  });
});



async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}
