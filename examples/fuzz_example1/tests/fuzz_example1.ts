import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FuzzExample1 } from "../target/types/fuzz_example1";
import { assert } from "chai";

const STATE_SEED = "state_seed";
const PROJECT_SEED = "project_seed";


describe("fuzz_example1", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.FuzzExample1 as Program<FuzzExample1>;

  const authority = anchor.web3.Keypair.generate();
  const project_author = anchor.web3.Keypair.generate();
  const investor = anchor.web3.Keypair.generate();


  it("Initializing state!", async () => {
    await airdrop(provider.connection, authority.publicKey);

    const [state, state_bump] = anchor.web3.PublicKey.findProgramAddressSync([
      authority.publicKey.toBuffer(),
      anchor.utils.bytes.utf8.encode(STATE_SEED),
    ], program.programId)


    await program.methods.initialize().accounts({
      author: authority.publicKey,
      state: state,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([authority]).rpc({ commitment: "confirmed" });



    const state_data = await program.account.state.fetch(state);

    assert.strictEqual(state_data.registrationsRound, false);
    assert.strictEqual(state_data.author.toString(), authority.publicKey.toString());

  });

  it.skip("End registrations", async () => {
    const [state, state_bump] = anchor.web3.PublicKey.findProgramAddressSync([
      authority.publicKey.toBuffer(),
      anchor.utils.bytes.utf8.encode(STATE_SEED),
    ], program.programId)


    await program.methods.endRegistrations().accounts({
      author: authority.publicKey,
      state: state
    }).signers([authority]).rpc({ commitment: "confirmed" });


    const state_data = await program.account.state.fetch(state);

    assert.strictEqual(state_data.registrationsRound, false);
    assert.strictEqual(state_data.author.toString(), authority.publicKey.toString());
  });


  it("Register project", async () => {
    await airdrop(provider.connection, project_author.publicKey);

    const [state, state_bump] = anchor.web3.PublicKey.findProgramAddressSync([
      authority.publicKey.toBuffer(),
      anchor.utils.bytes.utf8.encode(STATE_SEED),
    ], program.programId)


    const [project, project_bump] = anchor.web3.PublicKey.findProgramAddressSync([
      project_author.publicKey.toBuffer(),
      state.toBuffer(),
      anchor.utils.bytes.utf8.encode(PROJECT_SEED),
    ], program.programId)



    await program.methods.register().accounts({
      projectAuthor: project_author.publicKey,
      project: project,
      state: state,
      systemProgram: anchor.web3.SystemProgram.programId
    }).signers([project_author]).rpc({
      commitment: "confirmed"
    });


    const project_data = await program.account.project.fetch(project);

    assert.strictEqual(project_data.investedAmount.toString(), new anchor.BN(0).toString());
    assert.strictEqual(project_data.projectAuthor.toString(), project_author.publicKey.toString());

  });

  it("Invest into project", async () => {
    await airdrop(provider.connection, investor.publicKey);

    const [state, state_bump] = anchor.web3.PublicKey.findProgramAddressSync([
      authority.publicKey.toBuffer(),
      anchor.utils.bytes.utf8.encode(STATE_SEED),
    ], program.programId)


    const [project, project_bump] = anchor.web3.PublicKey.findProgramAddressSync([
      project_author.publicKey.toBuffer(),
      state.toBuffer(),
      anchor.utils.bytes.utf8.encode(PROJECT_SEED),
    ], program.programId)


    const stateDataBefore = await program.account.state.fetch(state);
    const projectDataBefore = await program.account.project.fetch(project);
    const projectBalaanceBefore = await provider.connection.getBalance(project);

    const investing_amount = new anchor.BN(50);

    await program.methods.invest(investing_amount).accounts({
      investor: investor.publicKey,
      project: project,
      state: state,
    }).signers([investor]).rpc({ commitment: "confirmed" });


    const stateDataAfter = await program.account.state.fetch(state);
    const projectDataAfter = await program.account.project.fetch(project);
    const projectBalaanceAfter = await provider.connection.getBalance(project);



    assert.strictEqual(projectDataAfter.investedAmount.toString(), projectDataBefore.investedAmount.add(investing_amount).toString());
    assert.strictEqual(stateDataAfter.totalInvested.toString(), stateDataBefore.totalInvested.add(investing_amount).toString());
    assert.strictEqual(projectBalaanceAfter, projectBalaanceBefore + investing_amount.toNumber());


  });
});


async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}
