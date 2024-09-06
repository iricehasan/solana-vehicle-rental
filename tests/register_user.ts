import * as web3 from "@solana/web3.js";
import { PythSolanaReceiver} from "@pythnetwork/pyth-solana-receiver";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VehicleRental } from "../target/types/vehicle_rental";
describe("Register User", () => {
  // Provider
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  // Get the program and provider wallet
  const program = anchor.workspace.VehicleRental as Program<VehicleRental>;
  const wallet = provider.wallet as anchor.Wallet;

  // Derive Seq and userAccount PDAs
  const [Seq, seqBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [program.provider.publicKey.toBuffer()],
    program.programId
  );

  const [userAccount, userAccountBump] =
  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user_account"), program.provider.publicKey.toBuffer()],
    program.programId
  );

  // Pyth account to fetch SOL/USD price
  const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
  const SOL_PRICE_FEED_ID =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"; // Price feed id
  const solUsdPriceFeedAccount = pythSolanaReceiver
    .getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID)
    .toBase58();

  console.log("Pyth Account: ", solUsdPriceFeedAccount);

  // inputs
  const userName = "userName";
  const userLastname = "userLastName";

  it("Register User Account", async () => {
    try {
      // Try fetching the user account to check if it already exists
      const userAccountData = await program.account.userAccount.fetch(userAccount);
      
      console.log("User is already registered:", userAccountData);
    } catch (error) {
      if (error.message.includes("Account does not exist")) {
        // If the account doesn't exist, proceed with registration
        console.log("User account not found, registering...");

        const registerUserAccountTx = await program.methods
          .registerUserAccount(userName, userLastname)
          .accounts({
            authority: program.provider.publicKey,
            userAccount: userAccount,
          })
          .rpc();

        console.log("Register User Account Transaction: ", registerUserAccountTx);
      } else {
        // Handle other potential errors
        console.error("Error checking user account:", error);
        throw error;
      }
    }
  });
});