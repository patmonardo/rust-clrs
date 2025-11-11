//! Chapter 10 – Stack exposition
//!
//! This executable narrates the textbook array-based stack from CLRS Section 10.1.
//! We trace the PUSH and POP procedures, check the overflow/underflow guards,
//! and keep a parallel "model" vector so we can assert the conceptual loop
//! invariant that:
//!   - `top` always indexes the next free slot (so it equals the stack height); and
//!   - all slots below `top` hold the elements in LIFO order.

use clrs::chapter_10::Stack;

struct NarratedStack {
    stack: Stack<i32>,
    model: Vec<i32>,
    capacity: usize,
}

impl NarratedStack {
    fn new(capacity: usize) -> Self {
        println!("Creating STACK with capacity {capacity}.");
        NarratedStack {
            stack: Stack::new(capacity),
            model: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn push(&mut self, value: i32) {
        println!("\nPUSH({value})");
        let full = self.model.len() == self.capacity;
        println!(
            "  • Expected top = {}, capacity = {}, full? {}",
            self.model.len(),
            self.capacity,
            full
        );

        match self.stack.push(value) {
            Ok(()) => {
                self.model.push(value);
                println!(
                    "  ✓ Element accepted. top now conceptually {}",
                    self.model.len()
                );
                self.report_state();
            }
            Err(msg) => {
                println!("  ✗ {msg}");
                assert!(
                    full,
                    "Stack reported overflow even though our model was not full."
                );
            }
        }

        self.verify_basic_invariants();
    }

    fn pop(&mut self) {
        println!("\nPOP()");
        let empty = self.model.is_empty();
        println!("  • Expected top = {}, empty? {}", self.model.len(), empty);

        match self.stack.pop() {
            Some(actual) => {
                let expected = self
                    .model
                    .pop()
                    .expect("Model predicted empty but real stack produced a value");
                println!("  ✓ Returned {}", actual);
                assert_eq!(
                    actual, expected,
                    "Model and CLRS stack disagree on the popped value."
                );
                println!("  • top should now be {}", self.model.len());
                self.report_state();
            }
            None => {
                println!("  ✗ Stack underflow");
                assert!(
                    empty,
                    "Stack reported underflow but the model still had elements."
                );
            }
        }

        self.verify_basic_invariants();
    }

    fn report_state(&self) {
        println!("    Current stack content (bottom → top): {:?}", self.model);
        if let Some(last) = self.model.last() {
            println!("    Top element (just below conceptual top pointer): {last}");
        } else {
            println!("    Stack is empty; top pointer sits at index 0.");
        }
    }

    fn verify_basic_invariants(&self) {
        assert!(
            self.model.len() <= self.capacity,
            "Model exceeded capacity — PUSH/POP logic broken."
        );
        assert_eq!(
            self.stack.is_empty(),
            self.model.is_empty(),
            "CLRS stack emptiness diverged from model."
        );
    }
}

fn main() {
    println!("=== CLRS Stack Narration ===");
    println!("We maintain the loop invariant \"top indexes the next free slot\".\n");

    let mut stack = NarratedStack::new(4);

    stack.push(4);
    stack.push(1);
    stack.push(3);
    stack.pop();
    stack.push(8);
    stack.pop();
    stack.pop();
    stack.pop(); // underflow attempt
    stack.pop(); // repeated underflow to illustrate guard

    stack.push(10);
    stack.push(12);
    stack.push(14);
    stack.push(16);
    stack.push(18); // overflow attempt

    println!(
        "\nNarration complete. Stack model ended with: {:?}",
        stack.model
    );
}
