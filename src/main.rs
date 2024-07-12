mod linked_list; // Importiere das Modul für die verkettete Liste
mod block; // Importiere das Modul für die Blockstruktur
mod blockchain; // Importiere das Modul für die Blockchain
mod mresult; // Importiere das Modul `mresult`

use blockchain::Blockchain; // Verwende die Blockchain-Struktur
use block::{Block, Transaction}; // Verwende die Block- und Transaktionsstrukturen
use mresult::MResult; // Verwende die `MResult`-Enum

fn main() {
    // Beispielverwendung von `MResult` im Hauptprogramm
    let result: MResult<i32, &str> = MResult::ok(42);

    if result.is_ok() {
        println!("Result is Ok with value: {}", result.unwrap());
    } else {
        println!("Result is an Err");
    }

    let mut blockchain = Blockchain::new(); // Erstelle eine neue Blockchain

    // Erstelle den Genesis-Block
    let genesis_transactions = vec![
        Transaction {
            sender: String::from("0"),
            receiver: String::from("Alice"),
            amount: 50,
        },
    ];
    let genesis_block = Block::new(
        0,
        String::from("0"),
        1625244673,
        genesis_transactions
    );

    if let Err(e) = blockchain.add_block(genesis_block) { // Füge den Genesis-Block hinzu
        println!("Failed to add genesis block: {}", e);
    }

    // Erstelle den zweiten Block
    let second_transactions = vec![
        Transaction {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 25,
        },
    ];
    let second_block = Block::new(
        1,
        blockchain.blocks.head.as_ref().unwrap().val.hash.clone(), // Verwende den Hash des vorherigen Blocks
        1625244674,
        second_transactions
    );

    if let Err(e) = blockchain.add_block(second_block) { // Füge den zweiten Block hinzu
        println!("Failed to add second block: {}", e);
    }

    // Abfrage aller Blöcke
    let all_blocks = blockchain.get_all_blocks();
    println!("All blocks in the blockchain:");
    for block in &all_blocks {
        println!("{}", block);
    }

    // Abfrage der letzten N Blöcke (z.B. die letzten 2 Blöcke)
    let last_blocks = blockchain.get_last_n_blocks(2);
    println!("Last 2 blocks in the blockchain:");
    for block in &last_blocks {
        println!("{}", block);
    }

    // Abfrage der Transaktionen eines bestimmten Blocks (z.B. Genesis-Block)
    if let Some(transactions) = blockchain.get_transactions_of_block(0) {
        println!("Transactions in the Genesis block:");
        for transaction in &transactions {
            println!("{}", transaction);
        }
    } else {
        println!("Genesis block not found");
    }

    // Gib den Genesis-Block aus
    match blockchain.get_block_by_index(0) {
        Some(block) => println!("{}", block),
        None => println!("Block not found"),
    }

    // Gib den zweiten Block aus
    let second_block_hash = blockchain.get_block_by_index(1).unwrap().hash.clone();

    match blockchain.get_block_by_hash(&second_block_hash) {
        Some(block) => println!("{}", block),
        None => println!("Block not found"),
    }

    // Speichere die Blockchain in eine Datei
    if let Err(e) = blockchain.save_to_file("blockchain.json") {
        println!("Failed to save blockchain: {}", e);
    }

    // Lade die Blockchain aus der Datei
    match Blockchain::load_from_file("blockchain.json") {
        Ok(loaded_blockchain) => {
            println!("Loaded blockchain from file:");
            for block in loaded_blockchain.blocks.iter() {
                println!("{}", block);
            }
        },
        Err(e) => println!("Failed to load blockchain: {}", e), // Hier das Semikolon durch ein Komma ersetzen
    }
}
