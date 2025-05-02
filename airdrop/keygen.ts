import { Keypair } from "@solana/web3.js";

let kp = Keypair.generate();

console.log(`You have generated a New Solana Keypair: ${kp.publicKey.toBase58()}`);

console.log(`[${kp.secretKey}]`);