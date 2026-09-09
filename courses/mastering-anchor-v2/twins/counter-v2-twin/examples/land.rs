//! Builds and signs one twin transaction offline, prints it base64 for
//! `sendTransaction`. Usage:
//!   land <init|increment> <payer-keypair.json> <counter-keypair.json> <recent-blockhash>
use std::str::FromStr;

use anchor_lang::InstructionData;
use solana_sdk::{
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args[1].as_str();
    let payer = read_keypair_file(&args[2]).expect("payer keypair");
    let counter = read_keypair_file(&args[3]).expect("counter keypair");
    let blockhash = Hash::from_str(&args[4]).expect("blockhash");

    let program_id = Pubkey::from_str(&counter_v2_twin::id().to_string()).unwrap();

    let (ix, signers): (Instruction, Vec<&dyn Signer>) = match mode {
        "init" => (
            Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new(counter.pubkey(), true),
                    AccountMeta::new_readonly(Pubkey::from_str("11111111111111111111111111111111").unwrap(), false),
                ],
                data: counter_v2_twin::instruction::Initialize {}.data(),
            },
            vec![&payer, &counter],
        ),
        "increment" => (
            Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new_readonly(payer.pubkey(), true),
                    AccountMeta::new(counter.pubkey(), false),
                ],
                data: counter_v2_twin::instruction::Increment {}.data(),
            },
            vec![&payer],
        ),
        other => panic!("unknown mode {other}"),
    };

    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let tx = Transaction::new(&signers, msg, blockhash);
    let bytes = bincode::serialize(&tx).expect("serialize");
    println!("{}", base64_encode(&bytes));
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { TABLE[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[n as usize & 63] as char } else { '=' });
    }
    out
}
