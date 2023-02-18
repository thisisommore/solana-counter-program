import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Counter } from "../target/types/counter";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  console.log(anchor.getProvider().connection.rpcEndpoint)
  const counter = anchor.web3.Keypair.generate()
  const program = anchor.workspace.Counter as Program<Counter>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      baseAccount: counter.publicKey
    }).signers([counter]).rpc();

    await program.methods.increment().accounts({
      baseAccount: counter.publicKey
    }).rpc();
    let v = (await program.account.counterData.fetch(counter.publicKey)).value
    expect(v).to.eq(1)

    await program.methods.decrement().accounts({
      baseAccount: counter.publicKey
    }).rpc();
    v = (await program.account.counterData.fetch(counter.publicKey)).value
    expect(v).to.eq(0)

  });
});
