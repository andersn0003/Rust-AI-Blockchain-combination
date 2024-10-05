use anchor_client;
#[allow(unused_imports)]
pub use borsh::{BorshDeserialize, BorshSerialize};
pub use solana_client::rpc_client::RpcClient;
#[allow(unused_imports)]
pub use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::Signature,
    signature::{Keypair, Signer},
    signer::EncodableKey,
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;

/// Sign and submit a legacy transaction.
///
/// This method fully signs a transaction with all required signers, which
/// must be present in the `keypairs` slice.
///
/// # Panics
///
/// Panics when signing or signature verification fails.
///
/// # Examples
///
/// This example uses the [`solana_program_client`] crate.
///
/// ```

 #[derive(BorshSerialize, BorshDeserialize)]
 #[borsh(crate = "borsh")]
 pub struct NewAccount {
     slope: f64,
     intercept: f64
 }

pub fn save_data_to_solana(slope: f64, intercept: f64) {
     // create a Rpc client connection
     let url = "https://api.devnet.solana.com".to_string();
     let timeout = std::time::Duration::from_secs(50);
     let connection = RpcClient::new_with_timeout(url, timeout);
     let program_id = Pubkey::from_str("GsX4b44N2vkDjnZPLucGV7ou5qxADN2N6BZ7zU8vnJ1X").unwrap();
     // let account_new = Keypair::new().pubkey();
     let payer = Keypair::read_from_file("src/wallet-keypair.json").unwrap();
    //  let mut seed_text = if transaction_num == 0 {
    //     b"new_init_seed"[..12].to_vec()
    // } else {
    //     b"save_added_seed"[..15].to_vec()
    // };
    let seed_text = b"final_seed";
    // Convert string to &[u8]
    let seed_text_slice: &[u8] = seed_text;
    let (account_new, _) = Pubkey::find_program_address(&[&seed_text_slice,&payer.pubkey().to_bytes()], &program_id);
    //  let instruction_name = "initialize";
    // Check if the PDA account exists
    let mut instruction_name = "initialize";
    match connection.get_account(&account_new) {
        Ok(account) => {
            println!("PDA account exists!");

            // Deserialize the data if needed
            // Assuming you have a struct for the account data
            if account.data.len() > 0 {
                println!("PDA account is initialized!");
                // You can deserialize the account data here to check its state
                instruction_name = "save_data"
            } else {
                println!("PDA account exists but is not initialized.");
            }
        }
        Err(_) => {
            println!("PDA account does not exist or is not initialized.");
        }
    }
    println!("instruction_name:{}", instruction_name);
     //  construct instruction data
     let instruction_data = NewAccount {
        slope,
        intercept ,
     };

     // setup signers
     let signers = &[&payer];
     // set up accounts
     let accounts = vec![
         AccountMeta::new(account_new, false),
         AccountMeta::new_readonly(payer.pubkey(), true),
         AccountMeta::new_readonly(system_program::ID, false),
         ];
         
         println!("{:?}", accounts);
     // call signed call
     let _tx_signature = signed_call(
         &connection,
         &program_id,
         &payer,
         signers,
         instruction_name,
         instruction_data,
         accounts,
     )
     .unwrap();
 }

pub fn signed_call(
    connection: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    signers: &[&Keypair],
    instruction_name: &str,
    instruction_data: NewAccount,
    accounts: Vec<AccountMeta>,
) -> Result<Signature, Box<dyn std::error::Error>>

{
    // get discriminant
    let instruction_discriminant = get_discriminant("global", instruction_name);

    // construct instruction
    let ix = Instruction::new_with_borsh(
        program_id.clone(),
        &(instruction_discriminant, instruction_data),
        accounts.clone(),
    );

    // get latest block hash
    let blockhash = connection.get_latest_blockhash().unwrap();

    // construct message
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);

    //construct transaction
    let mut tx = Transaction::new_unsigned(msg);

    // sign transaction
    tx.sign(signers, tx.message.recent_blockhash);

    // send and confirm transaction
    let tx_signature = connection
    .send_and_confirm_transaction_with_spinner(&tx)
    .map_err(|err| {
        println!("{:?}", err);
        }).unwrap();
    println!("Program uploaded successfully. Transaction ID: {}", tx_signature);

    Ok(tx_signature)
}

/// returns function signature
///
/// accepts name space and name function
pub fn get_discriminant(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_client::anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()
            [..8],
    );
    
    // println!("signature-hash:{:?}", sighash);
    sighash
}