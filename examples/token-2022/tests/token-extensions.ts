import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { TokenExtensions } from "../target/types/token_extensions";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { assert, expect } from "chai";

const TOKEN_2022_PROGRAM_ID = new anchor.web3.PublicKey(
  "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
);

export function associatedAddress({
  mint,
  owner,
}: {
  mint: PublicKey;
  owner: PublicKey;
}): PublicKey {
  return PublicKey.findProgramAddressSync(
    [owner.toBuffer(), TOKEN_2022_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_PROGRAM_ID
  )[0];
}

describe("token extensions", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TokenExtensions as Program<TokenExtensions>;

  const payer = Keypair.generate();

  it("airdrop payer", async () => {
    const tx = await provider.connection.requestAirdrop(payer.publicKey, 10000000000);
    let confirmed = false;
    while (!confirmed) {
      await new Promise(resolve => setTimeout(resolve, 1000));
      confirmed = await provider.connection.getSignatureStatuses([tx]);
      if (!confirmed) continue;
      if (confirmed.value[0].err) {
        throw new Error(confirmed.value[0].err.toString());
      }
      if (confirmed.value[0].confirmationStatus === 'confirmed') {
        confirmed = true;
        break;
      }
    }
  });

  let mint = new Keypair();

  it("Create mint account test passes", async () => {
    const [extraMetasAccount] = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("extra-account-metas"),
        mint.publicKey.toBuffer(),
      ],
      program.programId
    );
    await program.methods
      .createMintAccount({
        name: "quick token",
        symbol: "QT",
        uri: "https://my-token-data.com/metadata.json",
      })
      .accountsStrict({
        payer: payer.publicKey,
        authority: payer.publicKey,
        receiver: payer.publicKey,
        mint: mint.publicKey,
        mintTokenAccount: associatedAddress({
          mint: mint.publicKey,
          owner: payer.publicKey,
        }),
        extraMetasAccount: extraMetasAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([mint, payer])
      .rpc();
  });

  it("mint extension constraints test passes", async () => {
    try {
      const tx = await program.methods
      .checkMintExtensionsConstraints()
      .accountsStrict({
        authority: payer.publicKey,
        mint: mint.publicKey,
      })
      .signers([payer])
      .rpc();
      assert.ok(tx, "transaction should be processed without error");
    } catch (e) {
      assert.fail('should not throw error');
    }
  });
  it("mint extension constraints fails with invalid authority", async () => {
    const wrongAuth = Keypair.generate();
    try {
      const x = await program.methods
      .checkMintExtensionsConstraints()
      .accountsStrict({
        authority: wrongAuth.publicKey,
        mint: mint.publicKey,
      })
      .signers([payer, wrongAuth])
      .rpc();
      assert.fail('should have thrown an error');
    } catch (e) {
      expect(e, 'should throw error');
    }

  })
});
