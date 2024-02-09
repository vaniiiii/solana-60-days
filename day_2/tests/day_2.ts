import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day_2";

describe("day_2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(777), new anchor.BN(888), "hello").rpc();
    console.log("Your transaction signature", tx);
  });

  // added this test
  it("Array test", async () => {
    const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888)]).rpc();
    console.log("Your transaction signature", tx);
  });

  it ("Sqrt test", async () => {
    const tx = await program.methods.sqrt(new anchor.BN(888)).rpc();
    console.log("Your transaction signature", tx);
  });
});
