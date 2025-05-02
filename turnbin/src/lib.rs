use solana_client::nonblocking::rpc_client;
mod programs;
use crate::programs::Turbin3_prereq::{TurbinePrereqProgram, CompleteArgs, UpdateArgs};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::{self, system_program};

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn turbin() {

        use solana_client::rpc_client::RpcClient;
        use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}};


        let rpc_client = RpcClient::new(RPC_URL);

        let signer = read_keypair_file("binwallet.json").expect("failed to read wallet");

        let prereq = TurbinePrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

        let args = CompleteArgs { github: b"cdpandora".to_vec()};
        let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");

        let transaction = TurbinePrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash
        );

        let signature = rpc_client .send_and_confirm_transaction(&transaction).expect("Failed 
        to send transaction");

        println!("https://explorer.solana.com/tx/{}/?cluster=devnet", signature)
    }

    // #[test]
   fn keygen() {
    use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet {}", kp.pubkey().to_string());
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file, {:?}", kp.to_bytes());
   }

//    #[test]
   fn airdrop() {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}};

    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(s) => {
            println!("Success, check your TX here:");
            println!("Your TX hash is: https://explorer.solana.com/tx/{:?}?cluster=devnet", s.to_string());
        },
        Err(e) => println!("Airdrop failed: {}", e.to_string()),
    }

   }

//    #[test]
   fn transfer_sol() {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};
    use std::str::FromStr;
    use crate::tests::solana_sdk::blake3::hash;

    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't read wallet from file");
    let to = read_keypair_file("binwallet.json").expect("Failed to read wallet");

    let pubkey = keypair.pubkey();

    let message_bytes = b"I verify my solana Keypair!";
    let sig = keypair.sign_message(message_bytes);
    let sig_hashed = hash(sig.as_ref());

    match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
        true => println!("Signature verified"),
        false => println!("Verification Failed"),
    }


   


    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(
            &keypair.pubkey(), &to.pubkey(), 1_000_000
        )],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction)
    .expect("Transfer failed");

    println!( 
       
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", 
        signature 
               
        );

   }

//    #[test]
fn allsol() {
    use solana_sdk::{message::Message,signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};

    let rpc_client = RpcClient::new(RPC_URL);
    let keypair = read_keypair_file("dev-wallet.json").expect("Failed to read file");
    let to_pubkey = read_keypair_file("binwallet.json").expect("Failed to read file");
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

    let message = Message::new_with_blockhash( 
       
        &[transfer( &keypair.pubkey(), &to_pubkey.pubkey(), balance, 
               
        )], Some(&keypair.pubkey()), &recent_blockhash 
               
        );
    
    let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

    let transaction = 
    Transaction::new_signed_with_payer( 
       
    &[transfer( &keypair.pubkey(), &to_pubkey.pubkey(), balance - fee, 
       
    )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(s) => {
            println!("Transaction confirmed");
            println!("Check your transaction here: https://explorer.solana.com/tx/{}/?cluster=devnet", s);
        },
        Err(e) => println!("Failed to send transaction: {}", e)
    }
    



}

}


