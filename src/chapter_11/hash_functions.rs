//! Hash Functions (Section 11.3)
//!
//! Various hash function implementations including division method,
//! multiplication method, and universal hashing.

/// Division method hash function
///
/// This corresponds to the division method from CLRS Section 11.3.1.
/// h(k) = k mod m
///
/// # Arguments
/// * `k` - The key
/// * `m` - The table size
///
/// # Returns
/// The hash value
pub fn division_hash(k: usize, m: usize) -> usize {
    k % m
}

/// Multiplication method hash function
///
/// This corresponds to the multiplication method from CLRS Section 11.3.2.
/// h(k) = floor(m * (kA mod 1)) where A is a constant
///
/// # Arguments
/// * `k` - The key
/// * `m` - The table size
/// * `a` - The constant A (typically (sqrt(5) - 1) / 2)
///
/// # Returns
/// The hash value
pub fn multiplication_hash(k: usize, m: usize, a: f64) -> usize {
    let fractional = (k as f64 * a) % 1.0;
    (m as f64 * fractional).floor() as usize
}

/// Multiplication hash with golden ratio constant
///
/// Uses A = (sqrt(5) - 1) / 2, which is approximately 0.6180339887
pub fn multiplication_hash_golden(k: usize, m: usize) -> usize {
    const GOLDEN_RATIO: f64 = 0.6180339887498949; // (sqrt(5) - 1) / 2
    multiplication_hash(k, m, GOLDEN_RATIO)
}

/// Hash function for strings using radix-128
///
/// This corresponds to the string hashing from CLRS Exercise 11.3-2.
/// Treats the string as a radix-128 number and uses the division method.
///
/// # Arguments
/// * `s` - The string to hash
/// * `m` - The table size
///
/// # Returns
/// The hash value
pub fn string_hash(s: &str, m: usize) -> usize {
    const RADIX: usize = 128;
    let mut sum = 0;
    for byte in s.bytes() {
        sum = (sum * RADIX + byte as usize) % m;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_division_hash() {
        assert_eq!(division_hash(10, 11), 10);
        assert_eq!(division_hash(22, 11), 0);
        assert_eq!(division_hash(5, 9), 5);
        assert_eq!(division_hash(28, 9), 1);
    }

    #[test]
    fn test_multiplication_hash_golden() {
        // Example from CLRS 11.3-4
        let m = 1000;
        let golden = (5.0_f64.sqrt() - 1.0) / 2.0;
        
        assert_eq!(multiplication_hash(61, m, golden), 700);
        assert_eq!(multiplication_hash(62, m, golden), 318);
        assert_eq!(multiplication_hash(63, m, golden), 936);
        assert_eq!(multiplication_hash(64, m, golden), 554);
        assert_eq!(multiplication_hash(65, m, golden), 172);
    }

    #[test]
    fn test_string_hash() {
        let m = 11;
        let hash1 = string_hash("hello", m);
        let hash2 = string_hash("world", m);
        
        // Should produce valid hash values
        assert!(hash1 < m);
        assert!(hash2 < m);
        
        // Same string should produce same hash
        assert_eq!(string_hash("test", m), string_hash("test", m));
    }
}

