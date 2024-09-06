import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TempProject } from "../target/types/temp_project";
describe("End Rent", () => {
  // Provider
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  // Get the program and provider wallet
  const program = anchor.workspace.TempProject as Program<TempProject>;
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
  
  const [carAccount, CarBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("car_account"), Buffer.from(seqData.carSeq.toString())],
      program.programId
    );

  const carAccountData = await program.account.carAccount.fetch(carAccount);  
console.log(JSON.stringify(carAccountData, null, 2));

const [rentAccount, rentAccountBump] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("rent_account"), carAccountData.car.toBuffer()],
  program.programId
);

console.log("End Rent: ", rentAccount.toBase58());
    const endRentTx = await program.methods
      .endRent()
      .accounts({
        authority: program.provider.publicKey,
        seq: Seq,
        rentAccount: rentAccount,
        carAccount: carAccountData.car,
        userAccount: userAccount,
        adminAccount: adminAccount,
      })
      .rpc();

      console.log("End Rent Transaction: ", endRentTx)
  });

});