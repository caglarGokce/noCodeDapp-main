import {
    Connection,
    Keypair,
    PublicKey,
    TransactionMessage,
    VersionedTransaction,
    SystemProgram,
    TransactionInstruction,
    LAMPORTS_PER_SOL,

  } from "@solana/web3.js";
  

  
    import { deserialize, serialize } from "borsh";

  
  


  
  
  
  const privkey1 = 
  [153,187,227,210,27,108,215,173,44,244,156,74,194,28,155,122,71,217,19,208,234,242,206,140,90,56,195,207,
    73,113,207,157,220,189,39,249,130,185,164,194,196,55,144,15,84,36,233,49,66,177,100,45,220,200,
    12,207,135,110,74,254,221,39,178,75]
  
  const payer = Keypair.fromSecretKey(Uint8Array.from(privkey1));

export const programId = new PublicKey("J7ARisVUv6qQYPoYtYDbZ8nFo5n3TQkR7SzrmHBdXCHG")
export const counter = new PublicKey("6JrP5CkQTzoyRcPE4KLrvU1VnngCdjEqvwFrrTZR4nB9")
export const role = new PublicKey("7giWMY4c2zhqpPfbNLNa4F68NEkKLLLW1ahtJNR6trpE")

