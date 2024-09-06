
import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import { PythSolanaReceiver} from "@pythnetwork/pyth-solana-receiver";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VehicleRental } from "../target/types/vehicle_rental";
describe("Rent Car", () => {
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

  const pythSolanaReceiver = new PythSolanaReceiver({ connection, wallet });
  const SOL_PRICE_FEED_ID =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
  const solUsdPriceFeedAccount = pythSolanaReceiver
    .getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID)
    .toBase58();

  console.log("Pyth Account: ", solUsdPriceFeedAccount);


  // inputs
const amount = new BN(20); // 20 dollars to rent a small car
const rentTimeInDays = new BN(1) // rent for 1 day

it("Rent Car", async () => {
  const seqData = await program.account.seq.fetch(Seq);  
  console.log(JSON.stringify(seqData, null, 2));

    
  const carSeq = new BN("00"); // The first carAccount was generated with car seq 0
  
  const [carAccount, CarBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("car_account"), Buffer.from(carSeq.toString())],
      program.programId
    );

  const carAccountData = await program.account.carAccount.fetch(carAccount);  
console.log(JSON.stringify(carAccountData, null, 2));

const [rentAccount, rentAccountBump] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("rent_account"), carAccount.toBuffer()],
  program.programId
);

console.log("Rent Account: ", rentAccount.toBase58());
    const rentCarTx = await program.methods
      .rentCar(rentTimeInDays, amount)
      .accounts({
        authority: program.provider.publicKey,
        seq: Seq,
        rentAccount: rentAccount,
        carAccount: carAccountData.car,
        userAccount: userAccount,
        priceUpdate: solUsdPriceFeedAccount,
      })
      .rpc();

      console.log("Rent Car Transaction: ", rentCarTx)
  });

});