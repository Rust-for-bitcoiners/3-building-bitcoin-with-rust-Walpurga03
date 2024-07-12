use serde::{Serialize, Deserialize}; // Importiere die Serialize und Deserialize Traits für die Serialisierung und Deserialisierung
use sha2::{Sha256, Digest}; // Importiere den Sha256-Hasher und das Digest-Trait für die Hash-Berechnung
use std::fmt; // Importiere das fmt-Modul für die Implementierung des Display-Traits

// Definition der Struktur für eine Transaktion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String, // Der Absender der Transaktion
    pub receiver: String, // Der Empfänger der Transaktion
    pub amount: u64, // Der Betrag der Transaktion
}

// Implementiere den Display-Trait für die Transaktionsstruktur
impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sender: {}, Receiver: {}, Amount: {}", self.sender, self.receiver, self.amount)
    }
}

// Definition der Struktur für einen Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u32, // Der Index des Blocks in der Blockchain
    pub previous_hash: String, // Der Hash des vorherigen Blocks
    pub timestamp: u64, // Der Zeitstempel des Blocks
    pub transactions: Vec<Transaction>, // Eine Liste von Transaktionen im Block
    pub hash: String, // Der Hash des Blocks
}

// Implementiere Methoden für die Blockstruktur
impl Block {
    // Erstelle einen neuen Block
    pub fn new(index: u32, previous_hash: String, timestamp: u64, transactions: Vec<Transaction>) -> Self {
        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            hash: String::new(), // Initialisiere den Hash mit einem leeren String
        };
        block.hash = block.calculate_hash(); // Berechne den Hash des Blocks
        block
    }

    // Berechne den Hash des Blocks
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new(); // Erstelle einen neuen Sha256-Hasher
        hasher.update(self.index.to_string()); // Füge den Index zum Hasher hinzu
        hasher.update(&self.previous_hash); // Füge den Hash des vorherigen Blocks hinzu
        hasher.update(self.timestamp.to_string()); // Füge den Zeitstempel hinzu
        for transaction in &self.transactions { // Füge jede Transaktion hinzu
            hasher.update(&transaction.sender);
            hasher.update(&transaction.receiver);
            hasher.update(transaction.amount.to_string());
        }
        format!("{:x}", hasher.finalize()) // Finalisiere den Hash und formatiere ihn als Hex-String
    }
}

// Implementiere den Display-Trait für die Blockstruktur
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block #{}\nPrevious Hash: {}\nTimestamp: {}\nHash: {}\nTransactions:\n",
            self.index, self.previous_hash, self.timestamp, self.hash)?; // Schreibe die Blockdaten
        for transaction in &self.transactions { // Schreibe jede Transaktion
            write!(f, "{}\n", transaction)?;
        }
        Ok(())
    }
}
