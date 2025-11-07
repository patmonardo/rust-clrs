//! Huffman Codes (Section 16.3)
//!
//! Huffman coding is a lossless data compression algorithm that assigns
//! variable-length codes to characters based on their frequencies.

use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Represents a character with its frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharFreq {
    pub character: char,
    pub frequency: usize,
}

impl CharFreq {
    /// Creates a new character-frequency pair
    pub fn new(character: char, frequency: usize) -> Self {
        CharFreq { character, frequency }
    }
}

/// Node in the Huffman tree
#[derive(Debug, Clone)]
pub enum HuffmanNode {
    Leaf {
        character: char,
        frequency: usize,
    },
    Internal {
        frequency: usize,
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
}

impl HuffmanNode {
    fn frequency(&self) -> usize {
        match self {
            HuffmanNode::Leaf { frequency, .. } => *frequency,
            HuffmanNode::Internal { frequency, .. } => *frequency,
        }
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency() == other.frequency()
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.frequency().cmp(&self.frequency())
    }
}

/// Builds a Huffman tree from character frequencies
///
/// This corresponds to HUFFMAN from CLRS Section 16.3.
///
/// # Arguments
/// * `char_freqs` - Vector of character-frequency pairs
///
/// # Returns
/// The root of the Huffman tree
///
/// # Complexity
/// - Time: O(n log n) where n is the number of characters
/// - Space: O(n)
///
/// # Example
/// ```
/// use clrs::chapter_16::{CharFreq, build_huffman_tree};
/// let freqs = vec![
///     CharFreq::new('a', 45),
///     CharFreq::new('b', 13),
///     CharFreq::new('c', 12),
///     CharFreq::new('d', 16),
///     CharFreq::new('e', 9),
///     CharFreq::new('f', 5),
/// ];
/// let tree = build_huffman_tree(&freqs);
/// assert!(tree.frequency() > 0);
/// ```
pub fn build_huffman_tree(char_freqs: &[CharFreq]) -> HuffmanNode {
    if char_freqs.is_empty() {
        panic!("Cannot build Huffman tree from empty frequency list");
    }
    
    if char_freqs.len() == 1 {
        return HuffmanNode::Leaf {
            character: char_freqs[0].character,
            frequency: char_freqs[0].frequency,
        };
    }
    
    let mut heap = BinaryHeap::new();
    
    // Initialize heap with leaf nodes
    for &cf in char_freqs {
        heap.push(HuffmanNode::Leaf {
            character: cf.character,
            frequency: cf.frequency,
        });
    }
    
    // Build the tree
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        
        let freq = left.frequency() + right.frequency();
        let internal = HuffmanNode::Internal {
            frequency: freq,
            left: Box::new(left),
            right: Box::new(right),
        };
        
        heap.push(internal);
    }
    
    heap.pop().unwrap()
}

/// Generates Huffman codes from a Huffman tree
///
/// # Arguments
/// * `tree` - Root of the Huffman tree
///
/// # Returns
/// A map from characters to their binary codes (as strings of '0' and '1')
///
/// # Complexity
/// - Time: O(n) where n is the number of leaves
/// - Space: O(n)
pub fn generate_codes(tree: &HuffmanNode) -> std::collections::HashMap<char, String> {
    let mut codes = std::collections::HashMap::new();
    generate_codes_recursive(tree, String::new(), &mut codes);
    codes
}

fn generate_codes_recursive(
    node: &HuffmanNode,
    prefix: String,
    codes: &mut std::collections::HashMap<char, String>,
) {
    match node {
        HuffmanNode::Leaf { character, .. } => {
            codes.insert(*character, prefix);
        }
        HuffmanNode::Internal { left, right, .. } => {
            generate_codes_recursive(left, format!("{}0", prefix), codes);
            generate_codes_recursive(right, format!("{}1", prefix), codes);
        }
    }
}

/// Encodes a string using Huffman codes
///
/// # Arguments
/// * `text` - Text to encode
/// * `codes` - Huffman code map
///
/// # Returns
/// Encoded binary string
pub fn encode(text: &str, codes: &std::collections::HashMap<char, String>) -> String {
    text.chars()
        .map(|c| codes.get(&c).unwrap_or(&String::new()).clone())
        .collect()
}

/// Decodes a binary string using a Huffman tree
///
/// # Arguments
/// * `encoded` - Binary string to decode
/// * `tree` - Root of the Huffman tree
///
/// # Returns
/// Decoded text
pub fn decode(encoded: &str, tree: &HuffmanNode) -> String {
    let mut result = String::new();
    let mut current = tree;
    let mut bits = encoded.chars();
    
    loop {
        match current {
            HuffmanNode::Leaf { character, .. } => {
                result.push(*character);
                current = tree;
            }
            HuffmanNode::Internal { left, right, .. } => {
                match bits.next() {
                    Some('0') => current = left,
                    Some('1') => current = right,
                    Some(_) => continue, // Skip invalid characters
                    None => break,
                }
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_huffman_tree() {
        let freqs = vec![
            CharFreq::new('a', 45),
            CharFreq::new('b', 13),
            CharFreq::new('c', 12),
            CharFreq::new('d', 16),
            CharFreq::new('e', 9),
            CharFreq::new('f', 5),
        ];
        let tree = build_huffman_tree(&freqs);
        assert_eq!(tree.frequency(), 100);
    }

    #[test]
    fn test_generate_codes() {
        let freqs = vec![
            CharFreq::new('a', 45),
            CharFreq::new('b', 13),
            CharFreq::new('c', 12),
        ];
        let tree = build_huffman_tree(&freqs);
        let codes = generate_codes(&tree);
        
        // All characters should have codes
        assert!(codes.contains_key(&'a'));
        assert!(codes.contains_key(&'b'));
        assert!(codes.contains_key(&'c'));
        
        // More frequent characters should have shorter codes
        let a_code_len = codes.get(&'a').unwrap().len();
        let b_code_len = codes.get(&'b').unwrap().len();
        let c_code_len = codes.get(&'c').unwrap().len();
        assert!(a_code_len <= b_code_len);
        assert!(a_code_len <= c_code_len);
    }

    #[test]
    fn test_encode_decode() {
        let freqs = vec![
            CharFreq::new('a', 45),
            CharFreq::new('b', 13),
            CharFreq::new('c', 12),
        ];
        let tree = build_huffman_tree(&freqs);
        let codes = generate_codes(&tree);
        
        let text = "abc";
        let encoded = encode(text, &codes);
        let decoded = decode(&encoded, &tree);
        
        assert_eq!(decoded, text);
    }
}

