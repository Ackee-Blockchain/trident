import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiMetaplex7 } from "../target/types/cpi_metaplex_7";
import { PublicKey } from '@solana/web3.js';
import * as spl_token from '@solana/spl-token';


export const MetaplexTokenMetadataProgram = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")

describe("cpi-metaplex-7", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.CpiMetaplex7 as Program<CpiMetaplex7>;

  const signer = anchor.web3.Keypair.generate();

  it("Initialize with Metaplex!", async () => {
    await airdrop(provider.connection, signer.publicKey)

    const name = "Name1";
    const symbol = "smb1";
    const uri = "uri1";

    const [mint, metadata] = await get_adresses();

    await program.methods.initialize(8, name, symbol, uri).accounts({
      signer: signer.publicKey,
      mint: mint.publicKey,
      metadataAccount: metadata,
      mplTokenMetadata: MetaplexTokenMetadataProgram,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: spl_token.TOKEN_PROGRAM_ID,
    }).signers([signer, mint]).rpc({ commitment: "confirmed" })
  });
});


export async function airdrop(
  connection: any,
  address: any,
  amount = 500_000_000_000
) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount),
    'confirmed'
  );
}

export async function get_adresses(): Promise<[anchor.web3.Keypair, anchor.web3.PublicKey]> {
  // WE GENERATE RANDOM KEYPAIR FOR THE MINT
  const mint = anchor.web3.Keypair.generate();
  const mintAddress = mint.publicKey;

  const [metadata, metadata_bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from('metadata'),
      MetaplexTokenMetadataProgram.toBuffer(),
      mintAddress.toBuffer(),
    ],
    MetaplexTokenMetadataProgram
  );

  return [mint, metadata];
}
