//! Chapter 13 – Red-black tree exposition
//!
//! This narrator mirrors the CLRS RB-INSERT story for the key sequence
//! 41, 38, 31, 12, 19, 8.  Our library currently performs the BST insert
//! and enforces a black root, so we watch the color assignments and the
//! structural recursion after each step.

use clrs::chapter_13::{RBNode, RedBlackTree};

#[derive(Debug)]
struct NarratedRbTree {
    tree: RedBlackTree<i32, String>,
    model: Vec<(i32, String)>,
}

impl NarratedRbTree {
    fn new() -> Self {
        println!("Creating empty red-black tree.");
        NarratedRbTree {
            tree: RedBlackTree::new(),
            model: Vec::new(),
        }
    }

    fn insert(&mut self, key: i32, value: &str) {
        println!("\nRB-INSERT({key}, {value:?})");
        self.tree.insert(key, value.to_owned());
        match self.model.iter_mut().find(|entry| entry.0 == key) {
            Some(existing) => existing.1 = value.to_owned(),
            None => {
                self.model.push((key, value.to_owned()));
                self.model.sort_by_key(|entry| entry.0);
            }
        }
        self.report();
    }

    fn search(&self, key: i32) {
        println!("\nRB-SEARCH({key})");
        match self.tree.search(key) {
            Some(val) => println!("  ✓ Found key {key} → {val:?}"),
            None => println!("  ✗ Key {key} not present."),
        }
    }

    fn report(&self) {
        let mut inorder = Vec::new();
        self.tree.inorder_walk(|k, v| inorder.push((*k, v.clone())));
        assert_eq!(
            inorder, self.model,
            "In-order traversal diverged from sorted model."
        );

        println!("  • In-order keys: {inorder:?}");
        if let Some((min_k, min_v)) = self.tree.minimum() {
            println!("    Minimum key = {min_k}, value = {min_v:?}");
        }
        if let Some((max_k, max_v)) = self.tree.maximum() {
            println!("    Maximum key = {max_k}, value = {max_v:?}");
        }

        println!("  • Structure (indent = depth):");
        self.print_structure(&self.tree.root, 0);
    }

    fn print_structure(&self, node: &Option<Box<RBNode<i32, String>>>, depth: usize) {
        let indent = "  ".repeat(depth);
        match node {
            Some(n) => {
                println!("{indent}- key {} ({:?})", n.key, n.color);
                self.print_structure(&n.left, depth + 1);
                self.print_structure(&n.right, depth + 1);
            }
            None => {
                println!("{indent}- nil");
            }
        }
    }
}

fn main() {
    println!("=== CLRS Red-Black Tree Narration ===");

    let mut rbt = NarratedRbTree::new();
    for (k, label) in [
        (41, "root"),
        (38, "left child"),
        (31, "grandchild"),
        (12, "insert sequence"),
        (19, "balancing key"),
        (8, "leaf insert"),
    ] {
        rbt.insert(k, label);
    }

    rbt.search(19);
    rbt.search(99);

    println!(
        "\nNarration complete. Final in-order sequence: {:?}",
        rbt.model
    );
}
