import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { HelloAnchor } from '../target/types/hello_anchor';

describe('hello-anchor', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.HelloAnchor as Program<HelloAnchor>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
