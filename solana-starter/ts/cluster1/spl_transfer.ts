import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("DiYKcnVNL1Hmd7YPhzKFLutRrEGA73dRVYqLJyCo8mVo");

// Recipient address
const to = new PublicKey("zbBjhHwuqyKMmz8ber5oUtJJ3ZV4B6ePmANfGyKzVGV");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it

        const getAta = await getOrCreateAssociatedTokenAccount(
                    connection,
                    keypair,
                    mint,
                    keypair.publicKey,
                );

        console.log(`Your ata is: ${getAta.address.toBase58()}`);

        // Get the token account of the toWallet address, and if it does not exist, create it

        const receiverata = await getOrCreateAssociatedTokenAccount(
                    connection,
                    keypair,
                    mint,
                    to,
        );
        console.log(`Your ata is: ${receiverata.address.toBase58()}`);

        // Transfer the new token to the "toTokenAccount" we just created

        const tx = await transfer(
            connection,
            keypair,
            getAta.address,
            receiverata.address,
            keypair,
            1e6
        );

        console.log(`Transaction successfull: ${tx}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();