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

async function getState() {
    // #region main
    // Read the generated IDL.
    const idl = JSON.parse(require('fs').readFileSync('./target/idl/turnstile.json', 'utf8'));
    // Address of the deployed program.
    const programId = getPublicKey("program");
    // Generate the program client from IDL.
    const program = new anchor.Program(idl, programId);
    const state = getKeypair("state");
    const preAccount = await program.account.state.fetch(state.publicKey);
    console.log(preAccount.locked);
    console.log(preAccount.res);
}

getState();
