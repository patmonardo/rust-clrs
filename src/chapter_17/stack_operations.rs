//! Stack Operations (Sections 17.1, 17.2, 17.3)
//!
//! Demonstrates amortized analysis using stack operations.
//! Shows aggregate analysis, accounting method, and potential method.

use std::collections::VecDeque;

/// A stack that tracks operations for amortized analysis
///
/// This demonstrates the three methods of amortized analysis:
/// - Aggregate analysis (Section 17.1)
/// - Accounting method (Section 17.2)
/// - Potential method (Section 17.3)
#[derive(Debug, Clone)]
pub struct AmortizedStack<T> {
    data: VecDeque<T>,
    operation_count: usize,
    total_cost: usize,
}

impl<T> AmortizedStack<T> {
    /// Creates a new empty stack
    pub fn new() -> Self {
        AmortizedStack {
            data: VecDeque::new(),
            operation_count: 0,
            total_cost: 0,
        }
    }
    
    /// Pushes an element onto the stack
    ///
    /// Actual cost: O(1)
    /// Amortized cost: O(1)
    ///
    /// # Arguments
    /// * `item` - Item to push
    pub fn push(&mut self, item: T) {
        self.data.push_back(item);
        self.operation_count += 1;
        self.total_cost += 1; // Actual cost: 1
    }
    
    /// Pops an element from the stack
    ///
    /// Actual cost: O(1)
    /// Amortized cost: O(1)
    ///
    /// # Returns
    /// The popped element, or None if stack is empty
    pub fn pop(&mut self) -> Option<T> {
        self.operation_count += 1;
        self.total_cost += 1; // Actual cost: 1
        self.data.pop_back()
    }
    
    /// Performs MULTIPOP operation: pops k elements
    ///
    /// Actual cost: O(min(k, s)) where s is stack size
    /// Amortized cost: O(1) per element
    ///
    /// # Arguments
    /// * `k` - Number of elements to pop
    ///
    /// # Returns
    /// Vector of popped elements
    pub fn multipop(&mut self, k: usize) -> Vec<T> {
        let mut result = Vec::new();
        let actual_cost = k.min(self.data.len());
        
        for _ in 0..actual_cost {
            if let Some(item) = self.data.pop_back() {
                result.push(item);
            }
        }
        
        self.operation_count += 1;
        self.total_cost += actual_cost;
        
        result
    }
    
    /// Returns the number of elements in the stack
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Returns true if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Returns the total number of operations performed
    pub fn operation_count(&self) -> usize {
        self.operation_count
    }
    
    /// Returns the total actual cost of all operations
    pub fn total_cost(&self) -> usize {
        self.total_cost
    }
    
    /// Returns the amortized cost per operation
    ///
    /// For stack operations, this should be O(1)
    pub fn amortized_cost_per_operation(&self) -> f64 {
        if self.operation_count == 0 {
            0.0
        } else {
            self.total_cost as f64 / self.operation_count as f64
        }
    }
}

impl<T> Default for AmortizedStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Demonstrates aggregate analysis for stack operations
///
/// Shows that n PUSH, POP, and MULTIPOP operations take O(n) time total.
pub fn demonstrate_aggregate_analysis() {
    let mut stack: AmortizedStack<i32> = AmortizedStack::new();
    
    // Perform a sequence of operations
    for i in 0..10 {
        stack.push(i);
    }
    
    // MULTIPOP can be expensive, but amortized cost is O(1) per element
    let _popped = stack.multipop(5);
    
    // Aggregate analysis: total cost should be O(n) where n is number of operations
    assert!(stack.total_cost() <= 2 * stack.operation_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_push_pop() {
        let mut stack = AmortizedStack::new();
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_stack_multipop() {
        let mut stack = AmortizedStack::new();
        
        for i in 0..10 {
            stack.push(i);
        }
        
        let popped = stack.multipop(5);
        assert_eq!(popped.len(), 5);
        assert_eq!(stack.len(), 5);
        
        let popped2 = stack.multipop(10); // Only 5 left
        assert_eq!(popped2.len(), 5);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_amortized_cost() {
        let mut stack = AmortizedStack::new();
        
        // Perform many operations
        for i in 0..100 {
            stack.push(i);
        }
        
        // MULTIPOP operations
        for _ in 0..10 {
            stack.multipop(5);
        }
        
        // Amortized cost per operation should be O(1)
        let amortized = stack.amortized_cost_per_operation();
        assert!(amortized <= 2.0); // Should be close to 1
    }
}

