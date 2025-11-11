//! Chapter 10 – Queue exposition
//!
//! We step through the circular-array queue from CLRS Section 10.1.
//! The narration keeps a mirrored `VecDeque` model to confirm FIFO order,
//! and it manually tracks the conceptual `head` and `tail` indices so we can
//! talk about the wrap-around invariant `(tail + 1) mod n == head` for the
//! "full" condition.

use clrs::chapter_10::Queue;
use std::collections::VecDeque;

struct NarratedQueue {
    queue: Queue<i32>,
    model: VecDeque<i32>,
    head: usize,
    tail: usize,
    capacity: usize,
}

impl NarratedQueue {
    fn new(capacity: usize) -> Self {
        println!("Creating QUEUE with circular array capacity {capacity}.");
        println!(
            "  → Remember: with the CLRS sentinel slot, we can store at most {} elements.\n",
            capacity.saturating_sub(1)
        );
        NarratedQueue {
            queue: Queue::new(capacity),
            model: VecDeque::new(),
            head: 0,
            tail: 0,
            capacity,
        }
    }

    fn enqueue(&mut self, value: i32) {
        println!("\nENQUEUE({value})");
        let expected_full = self.model.len() == self.capacity - 1;
        println!(
            "  • head = {}, tail = {}, len = {}, full? {}",
            self.head,
            self.tail,
            self.model.len(),
            expected_full
        );
        assert_eq!(
            self.queue.is_full(),
            expected_full,
            "Queue::is_full disagrees with model before enqueue."
        );

        match self.queue.enqueue(value) {
            Ok(()) => {
                assert!(
                    !expected_full,
                    "Enqueue succeeded even though model predicted overflow."
                );
                self.model.push_back(value);
                self.tail = (self.tail + 1) % self.capacity;
                println!("  ✓ Element inserted; tail advanced to {}", self.tail);
                self.report_state();
            }
            Err(msg) => {
                println!("  ✗ {msg}");
                assert!(
                    expected_full,
                    "Queue reported overflow when model still had room."
                );
            }
        }

        self.verify_basic_invariants();
    }

    fn dequeue(&mut self) {
        println!("\nDEQUEUE()");
        let expected_empty = self.model.is_empty();
        println!(
            "  • head = {}, tail = {}, len = {}, empty? {}",
            self.head,
            self.tail,
            self.model.len(),
            expected_empty
        );
        assert_eq!(
            self.queue.is_empty(),
            expected_empty,
            "Queue::is_empty disagrees with model before dequeue."
        );

        match self.queue.dequeue() {
            Some(actual) => {
                let expected = self
                    .model
                    .pop_front()
                    .expect("Model predicted empty but dequeue produced a value");
                println!("  ✓ Returned {actual}");
                assert_eq!(actual, expected, "Model and queue disagree on FIFO order.");
                self.head = (self.head + 1) % self.capacity;
                println!("  • head advanced to {}", self.head);
                self.report_state();
            }
            None => {
                println!("  ✗ Queue underflow");
                assert!(
                    expected_empty,
                    "Queue reported underflow but model still had elements."
                );
            }
        }

        self.verify_basic_invariants();
    }

    fn report_state(&self) {
        println!("    Queue contents (front → back): {:?}", self.model);
        println!(
            "    Conceptual indices: head = {}, tail = {}, full slot invariant: (tail + 1) % n = {}",
            self.head,
            self.tail,
            (self.tail + 1) % self.capacity
        );
    }

    fn verify_basic_invariants(&self) {
        let max_payload = self.capacity - 1;
        assert!(
            self.model.len() <= max_payload,
            "Model exceeded the usable payload slots."
        );
        assert_eq!(
            self.queue.is_empty(),
            self.model.is_empty(),
            "Queue emptiness mismatch after operation."
        );
        assert_eq!(
            self.queue.is_full(),
            self.model.len() == max_payload,
            "Queue fullness mismatch after operation."
        );
    }
}

fn main() {
    println!("=== CLRS Queue Narration ===");
    println!("We follow the circular-array queue and track head/tail wrap-around.\n");

    let mut queue = NarratedQueue::new(6);

    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    queue.dequeue();
    queue.enqueue(40);
    queue.enqueue(50);
    queue.enqueue(60); // should fill the structure
    queue.enqueue(70); // overflow attempt
    queue.dequeue();
    queue.dequeue();
    queue.enqueue(80); // demonstrates wrap-around reuse
    queue.enqueue(90); // still room after wrap
    queue.dequeue();
    queue.dequeue();
    queue.dequeue();
    queue.dequeue();
    queue.dequeue(); // underflow attempt

    println!(
        "\nNarration complete. Remaining payload (front → back): {:?}",
        queue.model
    );
}
