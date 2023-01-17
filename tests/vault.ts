import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {Vault} from "../target/types/vault";

describe("vault", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Vault as Program<Vault>;
    let vault = anchor.web3.Keypair.generate();
    let depositEntry = anchor.web3.Keypair.generate();

    it("should be initialized!", async () => {
        const tx = await program.methods.initialize(new anchor.BN(100))
            .accounts({
                vault: vault.publicKey
            })
            .signers([vault])
            .rpc();
        console.log(`Tx ${tx}`);
    });

    it("should perform a deposit", async () => {
        const tx = await program.methods.deposit(new anchor.BN(5000))
            .accounts({
                vault: vault.publicKey,
                depositEntry: depositEntry.publicKey,
            })
            .signers([depositEntry])
            .rpc();
        console.log(`Tx ${tx}`);
    });

    it("should perform a withdrawal", async () => {
        const tx = await program.methods.withdraw(new anchor.BN(10))
            .accounts({
                vault: vault.publicKey,
                depositEntry: depositEntry.publicKey,
            })
            .rpc();
        console.log(`Tx ${tx}`);
    });
});
