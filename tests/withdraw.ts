
import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import { PythSolanaReceiver} from "@pythnetwork/pyth-solana-receiver";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TempProject } from "../target/types/temp_project";
describe("Withdraw", () => {
  // Provider
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  // Get the program and provider wallet
  const program = anchor.workspace.TempProject as Program<TempProject>;
  const wallet = provider.wallet as anchor.Wallet;

    // Derive userAccount PDA
  const [userAccount, userAccountBump] =
  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user_account"), program.provider.publicKey.toBuffer()],
    program.programId
  );


  const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
  const SOL_PRICE_FEED_ID =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
  const solUsdPriceFeedAccount = pythSolanaReceiver
    .getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID)
    .toBase58();

  console.log("Pyth Account Address: ", solUsdPriceFeedAccount);

  it("Withdraw Amount", async () => {
    const amount = new BN(20); // withdraw 20 dollars

    const withdrawTx = await program.methods
      .deposit(amount)
      .accounts({
        userAccount: userAccount.toBase58(),
        holder: program.provider.publicKey,
        priceUpdate: solUsdPriceFeedAccount,
      })
      .rpc();

      console.log("Withdraw transaction: ", withdrawTx)
  });

});