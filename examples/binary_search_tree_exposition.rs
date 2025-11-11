//! Chapter 12 – Binary search tree exposition
//!
//! We instrument the fundamental CLRS BST operations (INSERT, SEARCH,
//! SUCCESSOR, DELETE) to watch the shape of the tree evolve.  The narrator
//! keeps a sorted Vec model so we can assert the BST-order invariant after
//! every mutation.

use clrs::chapter_12::BinarySearchTree;

#[derive(Debug)]
struct NarratedBst {
    tree: BinarySearchTree<i32, String>,
    model: Vec<(i32, String)>,
}

impl NarratedBst {
    fn new() -> Self {
        println!("Creating empty BST.");
        NarratedBst {
            tree: BinarySearchTree::new(),
            model: Vec::new(),
        }
    }

    fn insert(&mut self, key: i32, value: &str) {
        println!("\nTREE-INSERT({key}, {value:?})");
        self.tree.insert(key, value.to_owned());
        self.model.push((key, value.to_owned()));
        self.model.sort_by_key(|entry| entry.0);
        self.verify_and_dump();
    }

    fn search(&self, key: i32) {
        println!("\nTREE-SEARCH({key})");
        match self.tree.search(key) {
            Some(val) => println!("  ✓ Found key {key} → {val:?}"),
            None => println!("  ✗ Key {key} not present."),
        }
    }

    fn successor(&self, key: i32) {
        println!("\nTREE-SUCCESSOR({key})");
        match self.tree.successor(&key) {
            Some((succ_key, succ_val)) => {
                println!("  ✓ Successor of {key} is {succ_key} → {succ_val:?}");
            }
            None => println!("  ✗ Key {key} has no successor (it is maximum or absent)."),
        }
    }

    fn delete(&mut self, key: i32) {
        println!("\nTREE-DELETE({key})");
        match self.tree.delete(&key) {
            Some(val) => {
                println!("  ✓ Removed {key} → {val:?}");
                if let Some(pos) = self.model.iter().position(|entry| entry.0 == key) {
                    self.model.remove(pos);
                }
            }
            None => println!("  ✗ Key {key} not found; tree unchanged."),
        }
        self.verify_and_dump();
    }

    fn verify_and_dump(&self) {
        let mut in_order = Vec::new();
        self.tree
            .inorder_walk(|k, v| in_order.push((*k, v.clone())));
        assert_eq!(
            in_order, self.model,
            "BST inorder walk diverged from sorted model."
        );

        println!("  • In-order walk (key → value): {in_order:?}");
        if let Some((min_k, min_v)) = self.tree.minimum() {
            println!("    Minimum key = {min_k}, value = {min_v:?}");
        }
        if let Some((max_k, max_v)) = self.tree.maximum() {
            println!("    Maximum key = {max_k}, value = {max_v:?}");
        }
    }
}

fn main() {
    println!("=== CLRS Binary Search Tree Narration ===");

    let mut bst = NarratedBst::new();

    // Build a non-trivial tree so each structural case appears.
    bst.insert(15, "root");
    bst.insert(6, "left subtree");
    bst.insert(18, "right subtree");
    bst.insert(3, "left-left");
    bst.insert(7, "left-right");
    bst.insert(17, "right-left");
    bst.insert(20, "right-right");
    bst.insert(2, "leaf");
    bst.insert(4, "leaf");
    bst.insert(13, "successor candidate");
    bst.insert(9, "inner node");

    bst.search(13);
    bst.search(8);

    bst.successor(13);
    bst.successor(20);

    bst.delete(2); // Case 1: leaf
    bst.delete(6); // Case 2: one child
    bst.delete(15); // Case 3: two children (root replacement)

    println!(
        "\nNarration complete. Final in-order sequence: {:?}",
        bst.model
    );
}
