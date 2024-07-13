# Assignment 3

1. Implement your own version of MResult and associated functions to get robust understanding of rust error handling

1. Review linked_list.rs and use it to repsent bitcoin data in block.rs file, add types and fields as you see fit.

1. Add functions simar to the capabilities of bitcoin core rpc commands. It doesn't have to be comprehensive.

For the task 3, implement functionalities to add a new block, query a block based on height,
blochash etc.,
You can store all your blockchain data in RAM itself, writing in files not required for this task
but you can try if you are really interested.

The underlying theme is *How would you represent bitcoin data using Rust?*

Have fun!


Hier ist eine grobe Zusammenfassung und Erklärung der fünf Klassen aus deinem Projekt basierend auf den bereitgestellten Dateien:

### 1. `block.rs`
Diese Datei enthält die Definitionen für die Strukturen `Transaction` und `Block`.

- **Transaction**:
  - Repräsentiert eine Transaktion mit den Feldern `sender`, `receiver` und `amount`.
  - Implementiert das `Display`-Trait, um eine menschenlesbare Darstellung zu ermöglichen.

- **Block**:
  - Repräsentiert einen Block in der Blockchain mit den Feldern `index`, `previous_hash`, `timestamp`, `transactions` und `hash`.
  - Methoden:
    - `new`: Erstellt einen neuen Block.
    - `calculate_hash`: Berechnet den Hash des Blocks unter Verwendung von SHA-256.

### 2. `blockchain.rs`
Diese Datei definiert die Struktur `Blockchain`.

- **Blockchain**:
  - Enthält eine Liste von Blöcken (`blocks`).
  - Methoden:
    - `new`: Erstellt eine neue Blockchain mit einem Genesis-Block.
    - `add_block`: Fügt einen neuen Block zur Blockchain hinzu.
    - `last_block`: Gibt den letzten Block in der Blockchain zurück.
    - `is_valid`: Überprüft, ob die Blockchain valide ist, indem sie die Hashes und die Verkettung der Blöcke überprüft.

### 3. `linked_list.rs`
Diese Datei enthält die Definitionen für eine einfache verknüpfte Liste (`LinkedList` und `Node`).

- **Node**:
  - Repräsentiert einen Knoten in der verknüpften Liste mit den Feldern `value` und `next`.

- **LinkedList**:
  - Repräsentiert die verknüpfte Liste mit dem Feld `head`, das auf den ersten Knoten zeigt.
  - Methoden:
    - `new`: Erstellt eine neue leere verknüpfte Liste.
    - `append`: Fügt ein neues Element ans Ende der Liste an.
    - `display`: Gibt die Elemente der Liste aus.

### 4. `main.rs`
Diese Datei enthält den Hauptprogrammlauf.

- Verwendet die definierten Strukturen, um eine Blockchain zu erstellen und Transaktionen hinzuzufügen.
- Beispiele für die Erstellung von Blöcken und das Hinzufügen zur Blockchain.
- Überprüft die Validität der Blockchain und zeigt die Blöcke an.

### 5. `mresult.rs`
Diese Datei definiert die Enumeration `MResult`, eine einfache Version des Result-Typs in Rust.

- **MResult**:
  - Eine generische Enum, die entweder einen erfolgreichen Wert (`Ok`) oder einen Fehler (`Err`) enthält.
  - Methoden:
    - `ok`: Erstellt eine `Ok`-Variante.
    - `err`: Erstellt eine `Err`-Variante.
    - `is_ok`, `is_err`: Überprüft, ob es sich um eine `Ok`- oder `Err`-Variante handelt.
    - `unwrap`, `unwrap_err`: Entpackt den Wert oder gibt einen Fehler zurück und panikt, falls der falsche Typ vorliegt.

Diese Klassen und Strukturen bilden zusammen ein einfaches Blockchain-System, das Transaktionen in Blöcken speichert und verkettete Listen verwendet, um die Struktur der Blockchain zu unterstützen. Die Hauptdatei (`main.rs`) zeigt, wie man diese Strukturen verwendet, um eine funktionierende Blockchain zu erstellen und zu validieren.