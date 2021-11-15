const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;
const fs = require('fs')

// Configure the local cluster.
anchor.setProvider(anchor.Provider.local());

const getPublicKey = (name) =>
  new anchor.web3.PublicKey(
    JSON.parse(fs.readFileSync(`./keys/${name}_pub.json`))
  );

const getPrivateKey = (name) =>
  Uint8Array.from(
    JSON.parse(fs.readFileSync(`./keys/${name}.json`))
  );

const getKeypair = (name) =>
  new anchor.web3.Keypair({
    publicKey: getPublicKey(name).toBytes(),
    secretKey: getPrivateKey(name),
  });

async function init() {
    // #region main
    // Read the generated IDL.
    const idl = JSON.parse(fs.readFileSync('./target/idl/turnstile.json', 'utf8'));
    // Address of the deployed program.
    const programId = getPublicKey("program");
    // Generate the program client from IDL.
    const program = new anchor.Program(idl, programId);

    const provider = anchor.Provider.local();
    const state = getKeypair("state");

    const tx = await program.rpc.initialize({
        accounts: {
            state: state.publicKey,
            user: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
        },
        signers: [state],
    });
    // console.log("Your transaction signature", tx);

    const account = await program.account.state.fetch(state.publicKey);
    console.log("Locked: " + account.locked);
    console.log("Result: " + account.res);
}

init();
