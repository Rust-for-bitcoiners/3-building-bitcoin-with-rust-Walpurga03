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

    // Überprüfe das Ergebnis und drucke entsprechend
    match result {
        MResult::Ok(value) => println!("Result is Ok with value: {}", value), // Ausgabe: "Result is Ok with value: 42"
        MResult::Err(_) => println!("Result is an Err"), // Ausgabe, wenn result ein Fehler wäre
    }

    let mut blockchain = Blockchain::new(); // Erstelle eine neue Blockchain

    // Erstelle den Genesis-Block
    let genesis_transactions = vec![
        Transaction {
            sender: String::from("0"), // Der Absender der Transaktion
            receiver: String::from("Alice"), // Der Empfänger der Transaktion
            amount: 50, // Der Betrag der Transaktion
        },
    ];
    let genesis_block = Block::new(
        0, // Der Index des Blocks
        String::from("0"), // Der Hash des vorherigen Blocks (hier "0" für den Genesis-Block)
        1625244673, // Der Zeitstempel des Blocks
        genesis_transactions, // Die Transaktionen des Blocks
    );

    // Versuche, den Genesis-Block zur Blockchain hinzuzufügen und drucke das Ergebnis
    match blockchain.add_block(genesis_block) {
        Ok(_) => println!("Genesis block added successfully"), // Ausgabe: "Genesis block added successfully"
        Err(e) => println!("Failed to add genesis block: {}", e), // Ausgabe im Fehlerfall
    }

    // Erstelle den zweiten Block
    let second_transactions = vec![
        Transaction {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 25,
        },
    ];
    let previous_block_hash = blockchain.get_block_by_index(0).unwrap().hash.clone(); // Hole den Hash des Genesis-Blocks
    let second_block = Block::new(
        1, // Der Index des zweiten Blocks
        previous_block_hash, // Der Hash des Genesis-Blocks als vorheriger Hash
        1625244674, // Der Zeitstempel des zweiten Blocks
        second_transactions, // Die Transaktionen des zweiten Blocks
    );

    // Versuche, den zweiten Block zur Blockchain hinzuzufügen und drucke das Ergebnis
    match blockchain.add_block(second_block) {
        Ok(_) => println!("Second block added successfully"), // Ausgabe: "Second block added successfully"
        Err(e) => println!("Failed to add second block: {}", e), // Ausgabe im Fehlerfall
    }

    // Abfrage aller Blöcke und deren Ausgabe
    let all_blocks = blockchain.get_all_blocks();
    println!("All blocks in the blockchain:"); // Überschrift für die Liste aller Blöcke
    for block in all_blocks.iter() {
        println!("{}", block); // Ausgabe jedes Blocks in der Blockchain
    }

    // Abfrage der Transaktionen eines bestimmten Blocks (z.B. Genesis-Block)
    match blockchain.get_transactions_of_block(0) {
        Some(transactions) => {
            println!("Transactions in the Genesis block:"); // Überschrift für die Transaktionen im Genesis-Block
            for transaction in &transactions {
                println!("{}", transaction); // Ausgabe jeder Transaktion im Genesis-Block
            }
        }
        None => println!("Genesis block not found"), // Ausgabe, wenn der Genesis-Block nicht gefunden wird
    }

    // Gib den Genesis-Block aus
    match blockchain.get_block_by_index(0) {
        Some(block) => println!("{}", block), // Ausgabe des Genesis-Blocks
        None => println!("Block not found"), // Ausgabe, wenn der Genesis-Block nicht gefunden wird
    }

    // Gib den zweiten Block aus
    let second_block_hash = blockchain.get_block_by_index(1).unwrap().hash.clone();
    match blockchain.get_block_by_hash(&second_block_hash) {
        Some(block) => println!("{}", block), // Ausgabe des zweiten Blocks
        None => println!("Block not found"), // Ausgabe, wenn der zweite Block nicht gefunden wird
    }

    // Speichere die Blockchain in eine Datei und drucke das Ergebnis
    match blockchain.save_to_file("blockchain.json") {
        Ok(_) => println!("Blockchain saved to file successfully"), // Ausgabe: "Blockchain saved to file successfully"
        Err(e) => println!("Failed to save blockchain: {}", e), // Ausgabe im Fehlerfall
    }

    // Lade die Blockchain aus der Datei und drucke das Ergebnis
    match Blockchain::load_from_file("blockchain.json") {
        Ok(loaded_blockchain) => {
            println!("Loaded blockchain from file:"); // Überschrift für die geladene Blockchain
            for block in loaded_blockchain.get_all_blocks().iter() {
                println!("{}", block); // Ausgabe jedes Blocks in der geladenen Blockchain
            }
        }
        Err(e) => println!("Failed to load blockchain: {}", e), // Ausgabe im Fehlerfall
    }
}
