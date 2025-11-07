//! Stacks (Section 10.1)
//!
//! A stack is a dynamic set in which the element deleted from the set
//! is the one most recently inserted (LIFO - Last In, First Out).

/// Stack implemented using an array
///
/// This corresponds to the stack implementation from CLRS Section 10.1.
/// The stack uses an array with a top pointer.
///
/// # Example
/// ```
/// use clrs::chapter_10::Stack;
/// let mut stack = Stack::new(6);
/// stack.push(4);
/// stack.push(1);
/// stack.push(3);
/// assert_eq!(stack.pop(), Some(3));
/// ```
#[derive(Debug, Clone)]
pub struct Stack<T> {
    arr: Vec<Option<T>>,
    top: usize,
    capacity: usize,
}

impl<T> Stack<T> {
    /// Creates a new empty stack with given capacity
    ///
    /// # Arguments
    /// * `capacity` - Maximum number of elements the stack can hold
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_10::Stack;
    /// let stack: Stack<i32> = Stack::new(10);
    /// ```
    pub fn new(capacity: usize) -> Self {
        let mut arr = Vec::with_capacity(capacity);
        arr.resize_with(capacity, || None);
        Stack {
            arr,
            top: 0,
            capacity,
        }
    }

    /// Checks if the stack is empty
    ///
    /// This corresponds to STACK-EMPTY from CLRS.
    ///
    /// # Returns
    /// `true` if the stack is empty, `false` otherwise
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_10::Stack;
    /// let stack: Stack<i32> = Stack::new(10);
    /// assert!(stack.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    /// Pushes an element onto the stack
    ///
    /// This corresponds to PUSH from CLRS Section 10.1.
    ///
    /// # Arguments
    /// * `x` - The element to push
    ///
    /// # Returns
    /// `Ok(())` on success, `Err` if stack overflow
    ///
    /// # Complexity
    /// - Time: O(1)
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_10::Stack;
    /// let mut stack = Stack::new(5);
    /// stack.push(42).unwrap();
    /// ```
    pub fn push(&mut self, x: T) -> Result<(), &'static str> {
        if self.top >= self.capacity {
            return Err("stack overflow");
        }
        self.arr[self.top] = Some(x);
        self.top += 1;
        Ok(())
    }

    /// Pops an element from the stack
    ///
    /// This corresponds to POP from CLRS Section 10.1.
    ///
    /// # Returns
    /// `Some(element)` if stack is not empty, `None` if stack underflow
    ///
    /// # Complexity
    /// - Time: O(1)
    ///
    /// # Example
    /// ```
    /// use clrs::chapter_10::Stack;
    /// let mut stack = Stack::new(5);
    /// stack.push(42).unwrap();
    /// assert_eq!(stack.pop(), Some(42));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.top -= 1;
        self.arr[self.top].take()
    }
}

/// Two stacks in one array (Exercise 10.1-2)
///
/// Implements two stacks in a single array where neither overflows
/// unless the total number of elements exceeds the array size.
///
/// # Example
/// ```
/// use clrs::chapter_10::TwoStacks;
/// let mut stacks = TwoStacks::new(10);
/// stacks.push1(1).unwrap();
/// stacks.push2(2).unwrap();
/// assert_eq!(stacks.pop1(), Some(1));
/// ```
#[derive(Debug, Clone)]
pub struct TwoStacks<T> {
    arr: Vec<Option<T>>,
    top1: usize,
    top2: usize,
    capacity: usize,
}

impl<T> TwoStacks<T> {
    /// Creates a new two-stack structure
    ///
    /// # Arguments
    /// * `capacity` - Total capacity for both stacks
    pub fn new(capacity: usize) -> Self {
        let mut arr = Vec::with_capacity(capacity);
        arr.resize_with(capacity, || None);
        TwoStacks {
            arr,
            top1: 0,
            top2: capacity,
            capacity,
        }
    }

    /// Checks if stack 1 is empty
    pub fn is_empty1(&self) -> bool {
        self.top1 == 0
    }

    /// Checks if stack 2 is empty
    pub fn is_empty2(&self) -> bool {
        self.top2 == self.capacity
    }

    /// Pushes element onto stack 1
    pub fn push1(&mut self, x: T) -> Result<(), &'static str> {
        if self.top1 >= self.top2 {
            return Err("stack overflow");
        }
        self.arr[self.top1] = Some(x);
        self.top1 += 1;
        Ok(())
    }

    /// Pushes element onto stack 2
    pub fn push2(&mut self, x: T) -> Result<(), &'static str> {
        if self.top1 >= self.top2 {
            return Err("stack overflow");
        }
        self.top2 -= 1;
        self.arr[self.top2] = Some(x);
        Ok(())
    }

    /// Pops element from stack 1
    pub fn pop1(&mut self) -> Option<T> {
        if self.is_empty1() {
            return None;
        }
        self.top1 -= 1;
        self.arr[self.top1].take()
    }

    /// Pops element from stack 2
    pub fn pop2(&mut self) -> Option<T> {
        if self.is_empty2() {
            return None;
        }
        let result = self.arr[self.top2].take();
        self.top2 += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_basic() {
        let mut stack = Stack::new(5);
        assert!(stack.is_empty());
        
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();
        
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_clrs_example() {
        // Example from CLRS 10.1-1
        let mut stack = Stack::new(6);
        stack.push(4).unwrap();
        stack.push(1).unwrap();
        stack.push(3).unwrap();
        assert_eq!(stack.pop(), Some(3));
        stack.push(8).unwrap();
        assert_eq!(stack.pop(), Some(8));
    }

    #[test]
    fn test_stack_overflow() {
        let mut stack = Stack::new(2);
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        assert!(stack.push(3).is_err());
    }

    #[test]
    fn test_stack_underflow() {
        let mut stack: Stack<i32> = Stack::new(5);
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_two_stacks() {
        let mut stacks = TwoStacks::new(10);
        stacks.push1(1).unwrap();
        stacks.push1(2).unwrap();
        stacks.push2(10).unwrap();
        stacks.push2(9).unwrap();
        
        assert_eq!(stacks.pop1(), Some(2));
        assert_eq!(stacks.pop2(), Some(9));
        assert_eq!(stacks.pop1(), Some(1));
        assert_eq!(stacks.pop2(), Some(10));
    }

    #[test]
    fn test_two_stacks_overflow() {
        let mut stacks = TwoStacks::new(3);
        stacks.push1(1).unwrap();
        stacks.push1(2).unwrap();
        stacks.push2(3).unwrap();
        // Total is 3, so next push should overflow
        assert!(stacks.push1(4).is_err());
    }
}

