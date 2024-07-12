use crate::block::{Block, Transaction}; // Importiere die Block- und Transaktionsstrukturen
use crate::linked_list::LinkedList; // Importiere die verkettete Liste
use serde::{Serialize, Deserialize}; // Importiere die Serialize und Deserialize Traits für die Serialisierung und Deserialisierung
use std::fs::File; // Importiere die File-Struktur für Dateioperationen
use std::io::{self, BufReader, BufWriter}; // Importiere IO-Funktionalitäten

// Definition der Struktur für die Blockchain
#[derive(Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: LinkedList<Block>, // Die Blockchain besteht aus einer verketteten Liste von Blöcken
}

// Implementiere Methoden für die Blockchain
impl Blockchain {
    // Erstelle eine neue, leere Blockchain
    pub fn new() -> Self {
        Blockchain {
            blocks: LinkedList::new(),
        }
    }

    // Füge einen neuen Block zur Blockchain hinzu
    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
        if let Some(last_block) = self.blocks.head.as_ref() {
            if last_block.val.hash != block.previous_hash {
                return Err(String::from("Invalid previous hash")); // Überprüfe den vorherigen Hash
            }
        }
        self.blocks.append(block); // Füge den neuen Block hinzu
        Ok(())
    }

    // Erhalte einen Block nach seinem Index
    pub fn get_block_by_index(&self, index: u32) -> Option<&Block> {
        for block in self.blocks.iter() {
            if block.index == index {
                return Some(block);
            }
        }
        None
    }

    // Erhalte einen Block nach seinem Hash
    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        for block in self.blocks.iter() {
            if block.hash == hash {
                return Some(block);
            }
        }
        None
    }

    // Speichere die Blockchain in eine Datei
    pub fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?; // Erstelle die Datei
        let writer = BufWriter::new(file); // Erstelle einen gepufferten Writer
        serde_json::to_writer(writer, &self)?; // Schreibe die Blockchain als JSON in die Datei
        Ok(())
    }

    // Lade die Blockchain aus einer Datei
    pub fn load_from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?; // Öffne die Datei
        let reader = BufReader::new(file); // Erstelle einen gepufferten Reader
        let blockchain = serde_json::from_reader(reader)?; // Lade die Blockchain aus der JSON-Datei
        Ok(blockchain)
    }

    // Funktion zur Abfrage aller Blöcke
    pub fn get_all_blocks(&self) -> Vec<&Block> {
        self.blocks.iter().collect()
    }

    // Funktion zur Abfrage der letzten N Blöcke
    pub fn get_last_n_blocks(&self, n: usize) -> Vec<&Block> {
        self.blocks.iter().collect::<Vec<_>>().into_iter().rev().take(n).collect()
    }

    // Funktion zur Abfrage der Transaktionen eines bestimmten Blocks
    pub fn get_transactions_of_block(&self, index: u32) -> Option<Vec<&Transaction>> {
        self.get_block_by_index(index).map(|block| block.transactions.iter().collect())
    }
}
