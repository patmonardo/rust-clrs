//! Queues (Section 10.1)
//!
//! A queue is a dynamic set in which the element deleted is always the one
//! that has been in the set for the longest time (FIFO - First In, First Out).

/// Queue implemented using a circular array
///
/// This corresponds to the queue implementation from CLRS Section 10.1.
/// The queue uses a circular array with head and tail pointers.
///
/// # Example
/// ```
/// use clrs::chapter_10::Queue;
/// let mut queue = Queue::new(6);
/// queue.enqueue(4).unwrap();
/// queue.enqueue(1).unwrap();
/// assert_eq!(queue.dequeue(), Some(4));
/// ```
#[derive(Debug, Clone)]
pub struct Queue<T> {
    arr: Vec<Option<T>>,
    head: usize,
    tail: usize,
    capacity: usize,
}

impl<T> Queue<T> {
    /// Creates a new empty queue with given capacity
    ///
    /// # Arguments
    /// * `capacity` - Maximum number of elements the queue can hold
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_10::Queue;
    /// let queue: Queue<i32> = Queue::new(10);
    /// ```
    pub fn new(capacity: usize) -> Self {
        let mut arr = Vec::with_capacity(capacity);
        arr.resize_with(capacity, || None);
        Queue {
            arr,
            head: 0,
            tail: 0,
            capacity,
        }
    }

    /// Checks if the queue is empty
    ///
    /// This corresponds to QUEUE-EMPTY from CLRS Exercise 10.1-4.
    ///
    /// # Returns
    /// `true` if the queue is empty, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /// Checks if the queue is full
    ///
    /// This corresponds to QUEUE-FULL from CLRS Exercise 10.1-4.
    ///
    /// # Returns
    /// `true` if the queue is full, `false` otherwise
    pub fn is_full(&self) -> bool {
        (self.tail + 1) % self.capacity == self.head
    }

    /// Enqueues an element into the queue
    ///
    /// This corresponds to ENQUEUE from CLRS Section 10.1.
    ///
    /// # Arguments
    /// * `x` - The element to enqueue
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` if queue overflow
    ///
    /// # Complexity
    /// - Time: O(1)
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_10::Queue;
    /// let mut queue = Queue::new(5);
    /// queue.enqueue(42).unwrap();
    /// ```
    pub fn enqueue(&mut self, x: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("queue overflow");
        }
        self.arr[self.tail] = Some(x);
        self.tail = (self.tail + 1) % self.capacity;
        Ok(())
    }

    /// Dequeues an element from the queue
    ///
    /// This corresponds to DEQUEUE from CLRS Section 10.1.
    ///
    /// # Returns
    /// `Some(element)` if queue is not empty, `None` if queue underflow
    ///
    /// # Complexity
    /// - Time: O(1)
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_10::Queue;
    /// let mut queue = Queue::new(5);
    /// queue.enqueue(42).unwrap();
    /// assert_eq!(queue.dequeue(), Some(42));
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let result = self.arr[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        result
    }
}

/// Deque (double-ended queue) (Exercise 10.1-5)
///
/// A deque allows insertion and deletion at both ends.
///
/// # Example
/// ```
/// use clrs::chapter_10::Deque;
/// let mut deque = Deque::new(10);
/// deque.head_enqueue(1).unwrap();
/// deque.tail_enqueue(2).unwrap();
/// assert_eq!(deque.head_dequeue(), Some(1));
/// ```
#[derive(Debug, Clone)]
pub struct Deque<T> {
    arr: Vec<Option<T>>,
    head: usize,
    tail: usize,
    capacity: usize,
}

impl<T> Deque<T> {
    /// Creates a new empty deque
    ///
    /// # Arguments
    /// * `capacity` - Maximum number of elements
    pub fn new(capacity: usize) -> Self {
        let mut arr = Vec::with_capacity(capacity);
        arr.resize_with(capacity, || None);
        Deque {
            arr,
            head: 0,
            tail: 0,
            capacity,
        }
    }

    /// Checks if the deque is empty
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /// Checks if the deque is full
    pub fn is_full(&self) -> bool {
        (self.tail + 1) % self.capacity == self.head
    }

    /// Enqueues an element at the head
    ///
    /// This corresponds to HEAD-ENQUEUE from CLRS Exercise 10.1-5.
    pub fn head_enqueue(&mut self, x: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("deque overflow");
        }
        self.head = if self.head == 0 {
            self.capacity - 1
        } else {
            self.head - 1
        };
        self.arr[self.head] = Some(x);
        Ok(())
    }

    /// Enqueues an element at the tail
    ///
    /// This corresponds to TAIL-ENQUEUE from CLRS Exercise 10.1-5.
    pub fn tail_enqueue(&mut self, x: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("deque overflow");
        }
        self.arr[self.tail] = Some(x);
        self.tail = (self.tail + 1) % self.capacity;
        Ok(())
    }

    /// Dequeues an element from the head
    ///
    /// This corresponds to HEAD-DEQUEUE from CLRS Exercise 10.1-5.
    pub fn head_dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let result = self.arr[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        result
    }

    /// Dequeues an element from the tail
    ///
    /// This corresponds to TAIL-DEQUEUE from CLRS Exercise 10.1-5.
    pub fn tail_dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.tail = if self.tail == 0 {
            self.capacity - 1
        } else {
            self.tail - 1
        };
        self.arr[self.tail].take()
    }
}

/// Queue implemented using two stacks (Exercise 10.1-6)
///
/// This demonstrates how to implement a queue using two stacks.
///
/// # Example
/// ```
/// use clrs::chapter_10::QueueFromStacks;
/// let mut queue = QueueFromStacks::new();
/// queue.enqueue(1);
/// queue.enqueue(2);
/// assert_eq!(queue.dequeue(), Some(1));
/// ```
pub struct QueueFromStacks<T> {
    stack_a: Vec<T>,
    stack_b: Vec<T>,
}

impl<T> QueueFromStacks<T> {
    /// Creates a new queue from two stacks
    pub fn new() -> Self {
        QueueFromStacks {
            stack_a: Vec::new(),
            stack_b: Vec::new(),
        }
    }

    /// Enqueues an element
    ///
    /// # Complexity
    /// - Time: O(1)
    pub fn enqueue(&mut self, x: T) {
        self.stack_b.push(x);
    }

    /// Dequeues an element
    ///
    /// # Complexity
    /// - Time: O(n) worst case, O(1) amortized
    pub fn dequeue(&mut self) -> Option<T> {
        if self.stack_a.is_empty() {
            // Transfer all elements from stack_b to stack_a
            while let Some(x) = self.stack_b.pop() {
                self.stack_a.push(x);
            }
        }
        self.stack_a.pop()
    }
}

impl<T> Default for QueueFromStacks<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_basic() {
        let mut queue = Queue::new(5);
        assert!(queue.is_empty());
        
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap();
        
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_queue_clrs_example() {
        // Example from CLRS 10.1-3
        let mut queue = Queue::new(6);
        queue.enqueue(4).unwrap();
        queue.enqueue(1).unwrap();
        queue.enqueue(3).unwrap();
        assert_eq!(queue.dequeue(), Some(4));
        queue.enqueue(8).unwrap();
        assert_eq!(queue.dequeue(), Some(1));
    }

    #[test]
    fn test_queue_overflow() {
        let mut queue = Queue::new(3);
        queue.enqueue(1).unwrap();
        queue.enqueue(2).unwrap();
        // Capacity 3 means max 2 elements (one slot reserved for full detection)
        assert!(queue.enqueue(3).is_err());
    }

    #[test]
    fn test_queue_underflow() {
        let mut queue: Queue<i32> = Queue::new(5);
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_deque() {
        let mut deque = Deque::new(10);
        deque.head_enqueue(1).unwrap();
        deque.tail_enqueue(2).unwrap();
        deque.head_enqueue(0).unwrap();
        
        assert_eq!(deque.head_dequeue(), Some(0));
        assert_eq!(deque.tail_dequeue(), Some(2));
        assert_eq!(deque.head_dequeue(), Some(1));
    }

    #[test]
    fn test_queue_from_stacks() {
        let mut queue = QueueFromStacks::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        queue.enqueue(4);
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
    }
}

