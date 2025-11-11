//! Chapter 10 – Tree representation exposition
//!
//! We contrast two CLRS Section 10.4 structures:
//!   * `BinaryTree`: every node has explicit `left` / `right` pointers.
//!   * `LCRSTree`: an arbitrary rooted tree captured via `left_child` /
//!     `right_sibling`.
//!
//! The narrators walk the textbook printing procedures so we can see how
//! each representation explores the tree.

use clrs::chapter_10::{BinaryTree, BinaryTreeNode, LCRSTree, LCRSTreeNode};
use std::collections::VecDeque;

fn main() {
    println!("=== CLRS Tree Representation Narration ===");

    let binary_tree = build_sample_binary_tree();
    println!("\nRecursive PRINT-BINARY-TREE (in-order) trace:");
    narrate_binary_inorder(binary_tree.root.as_ref(), 0);

    println!("\nIterative PRINT-BINARY-TREE trace (stack-based in-order):");
    narrate_binary_iterative(&binary_tree);

    let lcrs_tree = build_sample_lcrs_tree();
    println!("\nLeft-child/right-sibling PRINT trace (pre-order over general tree):");
    narrate_lcrs_preorder(lcrs_tree.root.as_ref(), 0, "root");
}

fn build_sample_binary_tree() -> BinaryTree<i32> {
    BinaryTree {
        root: Some(Box::new(BinaryTreeNode {
            key: 10,
            left: Some(Box::new(BinaryTreeNode {
                key: 5,
                left: Some(Box::new(BinaryTreeNode {
                    key: 2,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTreeNode {
                    key: 7,
                    left: None,
                    right: Some(Box::new(BinaryTreeNode {
                        key: 9,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            right: Some(Box::new(BinaryTreeNode {
                key: 15,
                left: Some(Box::new(BinaryTreeNode {
                    key: 13,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(BinaryTreeNode {
                    key: 20,
                    left: None,
                    right: None,
                })),
            })),
        })),
    }
}

fn build_sample_lcrs_tree() -> LCRSTree<&'static str> {
    LCRSTree {
        root: Some(Box::new(LCRSTreeNode {
            key: "A",
            left_child: Some(Box::new(LCRSTreeNode {
                key: "B",
                left_child: Some(Box::new(LCRSTreeNode {
                    key: "E",
                    left_child: None,
                    right_sibling: Some(Box::new(LCRSTreeNode {
                        key: "F",
                        left_child: Some(Box::new(LCRSTreeNode {
                            key: "I",
                            left_child: None,
                            right_sibling: None,
                        })),
                        right_sibling: None,
                    })),
                })),
                right_sibling: Some(Box::new(LCRSTreeNode {
                    key: "C",
                    left_child: Some(Box::new(LCRSTreeNode {
                        key: "G",
                        left_child: None,
                        right_sibling: Some(Box::new(LCRSTreeNode {
                            key: "H",
                            left_child: None,
                            right_sibling: None,
                        })),
                    })),
                    right_sibling: Some(Box::new(LCRSTreeNode {
                        key: "D",
                        left_child: None,
                        right_sibling: None,
                    })),
                })),
            })),
            right_sibling: None,
        })),
    }
}

fn narrate_binary_inorder(node: Option<&Box<BinaryTreeNode<i32>>>, depth: usize) {
    let indent = "  ".repeat(depth);
    match node {
        Some(n) => {
            println!("{indent}Node {}: descend left", n.key);
            narrate_binary_inorder(n.left.as_ref(), depth + 1);
            println!("{indent}Visit {}", n.key);
            println!("{indent}Node {}: descend right", n.key);
            narrate_binary_inorder(n.right.as_ref(), depth + 1);
        }
        None => println!("{indent}Reached nil (empty child)"),
    }
}

fn narrate_binary_iterative(tree: &BinaryTree<i32>) {
    let mut stack: VecDeque<&BinaryTreeNode<i32>> = VecDeque::new();
    let mut current = tree.root.as_ref().map(|n| n.as_ref());
    let mut step = 0;

    loop {
        while let Some(node) = current {
            println!(
                "  Step {step}: push {} and move to its left child",
                node.key
            );
            stack.push_back(node);
            current = node.left.as_ref().map(|n| n.as_ref());
            step += 1;
        }

        if stack.is_empty() {
            println!("  Stack empty—iteration complete.");
            break;
        }

        let node = stack.pop_back().expect("stack not empty");
        println!("  Step {step}: visit {} (pop from stack)", node.key);
        step += 1;
        current = node.right.as_ref().map(|n| n.as_ref());
        if current.is_some() {
            println!(
                "    Move to right child of {} and continue descending",
                node.key
            );
        }
    }
}

fn narrate_lcrs_preorder(
    node: Option<&Box<LCRSTreeNode<&'static str>>>,
    depth: usize,
    relation: &str,
) {
    let indent = "  ".repeat(depth);
    match node {
        Some(n) => {
            println!("{indent}Visit {} ({relation})", n.key);
            if let Some(first_child) = n.left_child.as_ref() {
                println!(
                    "{indent}  → Descend to first child {} of {}",
                    first_child.key, n.key
                );
                narrate_lcrs_preorder(Some(first_child), depth + 1, "first child");

                let mut sibling = first_child.right_sibling.as_ref();
                while let Some(s) = sibling {
                    println!(
                        "{indent}  → Step across right sibling {} at depth {}",
                        s.key,
                        depth + 1
                    );
                    narrate_lcrs_preorder(Some(s), depth + 1, "right sibling");
                    sibling = s.right_sibling.as_ref();
                }
            } else {
                println!("{indent}  (no children for {})", n.key);
            }
        }
        None => println!("{indent}Reached nil (no node)"),
    }
}
