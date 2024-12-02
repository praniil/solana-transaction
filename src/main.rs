use std::str::FromStr;

use solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::{Signer, read_keypair_file}, system_instruction, transaction::Transaction
};

use solana_client::rpc_client::RpcClient;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let client = RpcClient::new_with_commitment("https://api.devnet.solana.com", CommitmentConfig::confirmed());

    let sender = read_keypair_file("/home/pranil/new_keypair.json")
    .expect("Failed to read keypair from file");


    let recepient = Pubkey::from_str("DVZosVVFp8pf7eksRZjQzEYPySrpELZePmb8qFh4n1pk").unwrap();

    let lamports = 1_000_000_000;

    //create a transaction instruction
    let transfer_ix = system_instruction::transfer(&sender.pubkey(), &recepient, lamports);

    //create a transaction
    let mut tx = Transaction::new_with_payer(&[transfer_ix], Some(&sender.pubkey()));

    //sign the transaction
    tx.sign(&[&sender], client.get_latest_blockhash()?);

    //send the signature to the sol network
    let signature = client.send_and_confirm_transaction(&tx)?;

    println!("transaction successful with signature: {}", signature);

    Ok(())
}
