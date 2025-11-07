//! Binary Counter (Section 17.1)
//!
//! Demonstrates aggregate analysis using a k-bit binary counter.
//! The amortized cost per INCREMENT operation is O(1).

/// A k-bit binary counter that supports INCREMENT operations
///
/// This demonstrates aggregate analysis from CLRS Section 17.1.
/// The amortized cost per INCREMENT is O(1) even though some
/// operations flip many bits.
///
/// # Example
/// ```
/// use clrs::chapter_17::BinaryCounter;
/// let mut counter = BinaryCounter::new(4);
/// counter.increment();
/// assert_eq!(counter.value(), 1);
/// counter.increment();
/// assert_eq!(counter.value(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct BinaryCounter {
    bits: Vec<bool>,
}

impl BinaryCounter {
    /// Creates a new k-bit binary counter initialized to 0
    ///
    /// # Arguments
    /// * `k` - Number of bits
    pub fn new(k: usize) -> Self {
        BinaryCounter {
            bits: vec![false; k],
        }
    }

    /// Increments the counter by 1
    ///
    /// This corresponds to INCREMENT from CLRS Section 17.1.
    /// Amortized cost: O(1)
    ///
    /// # Returns
    /// The number of bits flipped (actual cost)
    pub fn increment(&mut self) -> usize {
        let mut i = 0;
        let mut flips = 0;

        // Find the rightmost 0 and flip it and all trailing 1s
        while i < self.bits.len() && self.bits[i] {
            self.bits[i] = false;
            flips += 1;
            i += 1;
        }

        if i < self.bits.len() {
            self.bits[i] = true;
            flips += 1;
        }

        flips
    }

    /// Returns the current value of the counter
    pub fn value(&self) -> usize {
        let mut result = 0;
        let mut power = 1;
        for &bit in &self.bits {
            if bit {
                result += power;
            }
            power *= 2;
        }
        result
    }

    /// Returns the number of bits in the counter
    pub fn bits(&self) -> usize {
        self.bits.len()
    }

    /// Returns a string representation of the counter (binary)
    pub fn to_string(&self) -> String {
        self.bits
            .iter()
            .rev()
            .map(|&b| if b { '1' } else { '0' })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_counter_increment() {
        let mut counter = BinaryCounter::new(4);

        assert_eq!(counter.value(), 0);
        assert_eq!(counter.increment(), 1); // 0 -> 1: flip bit 0
        assert_eq!(counter.value(), 1);

        assert_eq!(counter.increment(), 2); // 1 -> 2: flip bits 0,1
        assert_eq!(counter.value(), 2);

        assert_eq!(counter.increment(), 1); // 2 -> 3: flip bit 0
        assert_eq!(counter.value(), 3);

        assert_eq!(counter.increment(), 3); // 3 -> 4: flip bits 0,1,2
        assert_eq!(counter.value(), 4);
    }

    #[test]
    fn test_binary_counter_overflow() {
        let mut counter = BinaryCounter::new(3);

        // Fill to maximum value (7 = 111 in binary)
        for _ in 0..7 {
            counter.increment();
        }
        assert_eq!(counter.value(), 7);

        // Next increment should overflow and wrap to 0
        // All bits are 1, so incrementing flips all bits to 0
        counter.increment();
        assert_eq!(counter.value(), 0); // Wraps to 0 on overflow
    }

    #[test]
    fn test_binary_counter_amortized_analysis() {
        let mut counter = BinaryCounter::new(8);
        let n = 16;
        let mut total_flips = 0;

        for _ in 0..n {
            total_flips += counter.increment();
        }

        // Aggregate analysis: total flips should be O(n)
        // For n operations, we expect roughly 2n flips
        assert!(total_flips <= 2 * n);
    }
}
