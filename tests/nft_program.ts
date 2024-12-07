import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftProgram } from "../target/types/nft_program";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("nft_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.NftProgram as Program<NftProgram>;

  const [tokenMint] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint-token")],
    program.programId
  );
  const [mintAuthority] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint-authority")],
    program.programId
  );
  const [poolAuthority] = PublicKey.findProgramAddressSync(
    [Buffer.from("pool-authority")],
    program.programId
  );
  const [userDataAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from("user-data")],
    program.programId
  );

  const poolTokenAccount = Keypair.generate();
  const userTokenAccount = Keypair.generate();

  it("Sets up the token mint", async () => {
    const tx = await program.methods
      .setupMint(9) // Decimal precision for the token
      .accounts({
        tokenMint: tokenMint,
        mintAuthority: mintAuthority,
        payer: provider.wallet.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Token mint setup transaction: ", tx);
  });

  it("Sets up the staking pool", async () => {
    const tx = await program.methods
      .setupPool()
      .accounts({
        tokenMint: tokenMint,
        poolAuthority: poolAuthority,
        poolTokenAccount: poolTokenAccount.publicKey,
        payer: provider.wallet.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([poolTokenAccount])
      .rpc();

    console.log("Staking pool setup transaction: ", tx);
  });

  it("Distributes an airdrop", async () => {
    const tx = await program.methods
      .distributeAirdrop(new anchor.BN(100)) // Airdrop 100 tokens
      .accounts({
        tokenMint: tokenMint,
        mintAuthority: mintAuthority,
        userAuthority: provider.wallet.publicKey,
        userWalletAccount: userTokenAccount.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([userTokenAccount])
      .rpc();

    console.log("Airdrop transaction: ", tx);
  });

  it("Performs staking", async () => {
    const tx = await program.methods
      .performStake(new anchor.BN(50)) // Stake 50 tokens
      .accounts({
        tokenMint: tokenMint,
        poolAuthority: poolAuthority,
        userAuthority: provider.wallet.publicKey,
        userWalletAccount: userTokenAccount.publicKey,
        userDataAccount: userDataAccount,
        poolWalletAccount: poolTokenAccount.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Staking transaction: ", tx);
  });

  it("Performs unstaking", async () => {
    const tx = await program.methods
      .performUnstake(new anchor.BN(30)) // Unstake 30 tokens
      .accounts({
        tokenMint: tokenMint,
        poolAuthority: poolAuthority,
        userAuthority: provider.wallet.publicKey,
        userWalletAccount: userTokenAccount.publicKey,
        userDataAccount: userDataAccount,
        poolWalletAccount: poolTokenAccount.publicKey,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Unstaking transaction: ", tx);
  });
});
