use serde::{Serialize, Deserialize}; // Importiere die Serialize und Deserialize Traits für die Serialisierung und Deserialisierung

// Definition der Struktur für einen Knoten in der verketteten Liste
#[derive(Serialize, Deserialize)]
pub struct Node<T> {
    pub val: T, // Der Wert des Knotens
    pub next: Option<Box<Node<T>>>, // Ein optionaler Zeiger auf den nächsten Knoten
}

// Definition der Struktur für die verkettete Liste
#[derive(Serialize, Deserialize)]
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>, // Ein optionaler Zeiger auf den Kopf der Liste
}

// Implementiere Methoden für die verkettete Liste
impl<T> LinkedList<T> {
    // Erstelle eine neue, leere verkettete Liste
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Füge einen neuen Knoten am Anfang der Liste hinzu
    pub fn append(&mut self, val: T) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(), // Setze den neuen Knoten als Kopf der Liste und verlinke den alten Kopf
        });
        self.head = Some(new_node);
    }

    // Erhalte einen Iterator für die Liste
    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            next: self.head.as_deref(), // Setze den Iterator auf den Kopf der Liste
        }
    }
}

// Definition des Iterators für die verkettete Liste
pub struct LinkedListIterator<'a, T> {
    next: Option<&'a Node<T>>, // Ein optionaler Zeiger auf den nächsten Knoten
}

// Implementiere den Iterator für die verkettete Liste
impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T; // Der Typ des Werts, der iteriert wird

    // Erhalte das nächste Element im Iterator
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref(); // Setze den Iterator auf den nächsten Knoten
            &node.val // Gib den Wert des aktuellen Knotens zurück
        })
    }
}

// Implementiere das DoubleEndedIterator-Trait für LinkedListIterator
impl<'a, T> DoubleEndedIterator for LinkedListIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut prev = None;
        let mut current = self.next;

        while let Some(node) = current {
            if node.next.is_none() {
                self.next = prev;
                return Some(&node.val);
            }
            prev = current;
            current = node.next.as_deref();
        }
        None
    }
}
