import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VehicleRental } from "../target/types/vehicle_rental";
describe("End Rent", () => {
  // Provider
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  // Get the program and provider wallet
  const program = anchor.workspace.VehicleRental as Program<VehicleRental>;
  const wallet = provider.wallet as anchor.Wallet;
  
  // Derive Seq, userAccount and AdminAccount PDAs
  const [Seq, seqBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [program.provider.publicKey.toBuffer()],
    program.programId
  );

  const [userAccount, userAccountBump] =
  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user_account"), program.provider.publicKey.toBuffer()],
    program.programId
  );

const [adminAccount, adminBump] =
  anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("admin"), program.provider.publicKey.toBuffer()],
    program.programId
  );

  it("End Rent", async () => {
    // Derive carAccount and rentAccount by fetching the sequence and then the carAccount address
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
      [Buffer.from("rent_account"), carAccountData.car.toBuffer()],
      program.programId
    );

    const rentData = await program.account.rentAccount.fetch(rentAccount);  
    console.log(JSON.stringify(rentData, null, 2));

    // Check if current time is before the end_date
    const currentUnixTimestamp = new BN(Math.floor(Date.now() / 1000)); // Get current time in Unix seconds
    const endDate = rentData.endDate; // Assuming endDate is stored in rentData

    const currentReadableDate = new Date(currentUnixTimestamp.toNumber() * 1000).toLocaleString();
    const endReadableDate = new Date(endDate.toNumber() * 1000).toLocaleString();
  
    console.log("Current Unix Timestamp: ", currentUnixTimestamp, " -> Readable: ", currentReadableDate);
    console.log("Rent End Date: ", endDate, " -> Readable: ", endReadableDate);
  

    if (currentUnixTimestamp < endDate) {
      console.log("Cannot end rent. The rent period is not over yet.");
      return;
    }

    console.log("End Rent: ", rentAccount.toBase58());
    const endRentTx = await program.methods
      .endRent()
      .accounts({
        authority: program.provider.publicKey,
        seq: Seq,
        rentAccount: rentAccount,
        carAccount: carAccount,
        userAccount: userAccount,
        adminAccount: adminAccount,
      })
      .rpc();

    console.log("End Rent Transaction: ", endRentTx);
  });

});