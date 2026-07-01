import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolCollector } from "../target/types/sol_collector";
import { expect } from "chai";

describe("sol-collector", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolCollector as Program<SolCollector>;

  let vaultPda: anchor.web3.PublicKey;
  let vaultBump: number;

  before(async () => {
    // Derive vault PDA
    [vaultPda, vaultBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault")],
      program.programId
    );
  });

  it("Initializes the vault", async () => {
    const tx = await program.methods
      .initializeVault()
      .accounts({
        vault: vaultPda,
        admin: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("✓ Vault initialized:", tx);
  });

  it("Deposits SOL into vault", async () => {
    const depositAmount = new anchor.BN(1_000_000); // 0.001 SOL

    const [userDepositPda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("deposit"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods
      .deposit(depositAmount)
      .accounts({
        vault: vaultPda,
        userDeposit: userDepositPda,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("✓ SOL deposited:", tx);
  });

  it("Retrieves vault state", async () => {
    const vault = await program.account.vault.fetch(vaultPda);
    console.log("Vault state:", {
      admin: vault.admin.toString(),
      totalDeposited: vault.totalDeposited.toString(),
      isPaused: vault.isPaused,
    });
    expect(vault.totalDeposited.toString()).to.equal("1000000");
  });

  it("Toggles vault pause", async () => {
    const tx = await program.methods
      .togglePause()
      .accounts({
        vault: vaultPda,
        admin: provider.wallet.publicKey,
      })
      .rpc();

    console.log("✓ Pause toggled:", tx);

    const vault = await program.account.vault.fetch(vaultPda);
    expect(vault.isPaused).to.be.true;
  });
});
