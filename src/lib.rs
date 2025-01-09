#[cfg(test)]
mod convert;
mod programs;
mod tests {
    use crate::programs::Turbin3_prereq::{CompleteArgs, Turbin3PrereqProgram, UpdateArgs};
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer};
    use solana_sdk::{
        message::Message,
        signature::{read_keypair_file, Keypair, Signer},
        system_program,
        sysvar::recent_blockhashes,
        transaction::{self, Transaction},
    };
    use std::str::FromStr;

    #[test]
    fn keygen() {
        let kp = Keypair::new();

        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );

        println!(
            "To save your wallet, copy and paste the following into a JSON file: {:?}",
            kp.to_bytes()
        );
    }

    #[test]
    fn airdrop() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL.to_string());

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!(
                    "Success!, Check out your TX here: https://explorer.solana.com/tx/{:?}?cluster=devnet",
                    s
                );
            }
            Err(e) => println!("Oops, something went wrong: {:?}", e),
        }
    }

    #[test]
    fn enroll() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        let rpc_client = RpcClient::new(RPC_URL);

        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        let prereq = Turbin3PrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);

        //    define our instruction data
        let args = CompleteArgs {
            github: b"AirZED".to_vec(),
        };

        // get recent blockhash
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get blockhash");

        // invoke the complete function
        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        // publish the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        // printout the transaction
        println!(
            "SUccess! Check out your TX here: https://explorer.solana.com/tx/{:?}?cluster=devnet",
            signature
        );
    }

    #[test]
    fn transfer_sol() {
        const RPC_URL: &str = "https://api.devnet.solana.com";

        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        let to_pubkey = Pubkey::from_str("2WKb1EQDfEKbivtmYHjx2cErQjASaNizVUC1AW2nbHKR").unwrap();

        let rpc_client = RpcClient::new(RPC_URL.to_string());

        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // submit transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success!, Check out your TX here: https://explorer.solana.com/tx/{:?}?cluster=devnet",
            signature
        );
    }
}
