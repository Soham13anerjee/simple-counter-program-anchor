import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { Keypair, PublicKey } from "@solana/web3.js"

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const counterAccount = new Keypair();

  const program = anchor.workspace.counter as Program<Counter>;

  const [counterPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    program.programId,
  )

  it("Is initialized!", async () => {
    // Add your test here.
    try {
      const tx = await program.methods.initialize().rpc();
      const accountData = await program.account.counter.fetch(counterPDA);
      console.log("Your transaction signature", tx);
      console.log("The count value is ",accountData.count); 
    } catch (error) {
      console.log(error);
    }
  });

  it("Is incremented!", async () => {
    const before_accountData = await program.account.counter.fetch(counterPDA);
    const before_count = before_accountData.count;
    console.log("Before increment : ",before_count);
    const tx = await program.methods.increment().rpc();

    console.log("Your transaction signature is, ",tx);
    const after_counterData = await program.account.counter.fetch(counterPDA);

    console.log("After increment : ",after_counterData.count);
  })
});
