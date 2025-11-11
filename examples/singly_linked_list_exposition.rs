//! Chapter 10 – Singly linked list exposition
//!
//! We walk through `LIST-INSERT`, `LIST-SEARCH`, and `LIST-DELETE` from
//! CLRS Section 10.2 using the `SinglyLinkedList` provided in the library.
//! The narration makes the implicit loop invariants explicit:
//!   - During search, every node before the current pointer has already been examined
//!     and does not contain the key.
//!   - During deletion, pointer rewiring preserves the linked structure.

use clrs::chapter_10::SinglyLinkedList;

struct NarratedList {
    list: SinglyLinkedList<i32>,
}

impl NarratedList {
    fn new() -> Self {
        println!("Creating an empty singly linked list.");
        NarratedList {
            list: SinglyLinkedList::new(),
        }
    }

    fn insert_front(&mut self, key: i32) {
        println!("\nLIST-INSERT({key}) at head");
        self.list.insert(key);
        println!("  • New head is {key}. Remaining suffix unchanged.");
        self.dump();
    }

    fn narrate_search(&self, key: i32) {
        println!("\nLIST-SEARCH({key})");
        let mut step = 0;
        let mut current = self.list.head.as_ref();
        while let Some(node) = current {
            println!(
                "  Step {step}: examining node with key = {} (prefix contains no match so far).",
                node.key
            );
            if node.key == key {
                println!("  ✓ Found key {key} after {} comparisons.", step + 1);
                return;
            }
            current = node.next.as_ref();
            step += 1;
        }
        println!("  ✗ Reached the end after {step} steps; key {key} not present.");
    }

    fn narrate_delete(&mut self, key: i32) {
        println!("\nLIST-DELETE({key})");
        let before = self.snapshot();
        println!("  • List before deletion: {:?}", before);
        let removed = self.list.delete(key);
        if removed {
            println!("  ✓ Key {key} removed. Rewired predecessor to skip the deleted node.");
        } else {
            println!("  ✗ Key {key} not found; structure unchanged.");
        }
        self.dump();
    }

    fn dump(&self) {
        println!("    Current list (head → tail): {:?}", self.snapshot());
    }

    fn snapshot(&self) -> Vec<i32> {
        let mut out = Vec::new();
        let mut node = self.list.head.as_ref();
        while let Some(current) = node {
            out.push(current.key);
            node = current.next.as_ref();
        }
        out
    }
}

fn main() {
    println!("=== CLRS Singly Linked List Narration ===");

    let mut list = NarratedList::new();

    list.insert_front(5);
    list.insert_front(9);
    list.insert_front(12);
    list.insert_front(7);

    list.narrate_search(12);
    list.narrate_search(3);

    list.narrate_delete(9);
    list.narrate_delete(42);

    list.insert_front(21);
    list.narrate_delete(7);

    println!(
        "\nNarration complete. Final list (head → tail): {:?}",
        list.snapshot()
    );
}
