
import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import { PythSolanaReceiver} from "@pythnetwork/pyth-solana-receiver";
import { getAssociatedTokenAddressSync } from '@solana/spl-token';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TempProject } from "../target/types/temp_project";

describe("Initialize", () => {
    // Provider
    const provider = anchor.AnchorProvider.env();
    const connection = provider.connection;
    anchor.setProvider(provider);
  
    // Get the program and provider wallet
    const program = anchor.workspace.TempProject as Program<TempProject>;
    const wallet = provider.wallet as anchor.Wallet; // providers wallet
  
      // Derive Seq and AdminAccount PDAs
    const [Seq, seqBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [program.provider.publicKey.toBuffer()],
      program.programId
    );
  
  
  const [adminAccount, adminBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("admin"), program.provider.publicKey.toBuffer()],
      program.programId
    );

    it("Initialize", async () => {
        try {
          // Try fetching the `Seq` account to check if it already exists
          const seqData = await program.account.seq.fetch(Seq);
          
          console.log("Seq account is already initialized:", seqData);
    
        } catch (error) {
          if (error.message.includes("Account does not exist")) {
            // If the account doesn't exist, proceed with initialization
            console.log("Seq account not found, initializing...");
    
            const initTx = await program.methods
              .initialize(program.provider.publicKey)
              .accounts({
                authority: program.provider.publicKey,
                seq: Seq.toBase58(),
                adminAccount: adminAccount.toBase58(),
              })
              .rpc();
      
            console.log("Initialize transaction: ", initTx);
          } else {
            // Handle any other errors
            console.error("Error checking Seq account:", error);
            throw error;
          }
        }
      });
});