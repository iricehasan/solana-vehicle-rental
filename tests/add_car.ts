import * as web3 from "@solana/web3.js";
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VehicleRental } from "../target/types/vehicle_rental";
describe("Add Car", () => {
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

  // Inputs
  const model = "sedan"
  const car_type = "Small";
  const nft_name = "name";
  const nft_symbol = "symbol";
  const nft_uri = "uri";

  // Generate a keypair to use as the address of our mint account
  const mintKeypair = new web3.Keypair();
  console.log(`   Mint Address: ${mintKeypair.publicKey}`);

  // Derive the associated token address account for the mint and payer.
  const associatedTokenAccountAddress = getAssociatedTokenAddressSync(mintKeypair.publicKey, wallet.publicKey);

it("Add Car", async () => {
  const seqData = await program.account.seq.fetch(Seq);  

const [carAccount, CarBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("car_account"), Buffer.from(seqData.carSeq.toString())],
    program.programId
  );

    const addCarTx = await program.methods
    .addCarAccount(model, car_type, nft_name, nft_symbol, nft_uri)
    .accounts({
      authority: program.provider.publicKey,
      seq: Seq,
      carAccount: carAccount,
      userAccount: userAccount,
      adminAccount: adminAccount,
      mintAccount: mintKeypair.publicKey,
      associatedTokenAccount: associatedTokenAccountAddress,
    })
    .signers([wallet.payer, mintKeypair])
    .rpc()

    console.log("Add Car Transaction: ", addCarTx)
  
  const carAccountData = await program.account.carAccount.fetch(carAccount);  
  console.log("Car Account Data: ",JSON.stringify(carAccountData, null, 2));
});


});