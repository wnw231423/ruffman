use bitvec::prelude::*;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::hash::Hash;

// PartialEq:
// To build a huffman tree, we'll use a min heap,
// which requires `Ord` trait.
// the dependency of `Ord` is:
//     `Ord` -> `PartialOrd` -> `Eq` -> `PartialEq`
// the `T` as token should be `Eq` to impl `Ord`, `PartialOrd` and `Eq`
#[derive(PartialEq)]
pub enum HuffmanTree<T> {
    // With `u64`, a token's frequency can be at most 2^64 - 1,
    // which should be enough for a directory of files of GiB size
    Leaf {
        frequency: u64,
        token: T,
    },
    Node {
        frequency: u64,
        left: Box<HuffmanTree<T>>,
        right: Box<HuffmanTree<T>>,
    },
}

#[allow(unused)]
impl<T: Clone> HuffmanTree<T> {
    pub fn frequency(&self) -> u64 {
        match self {
            Self::Leaf { frequency, .. } => *frequency,
            Self::Node { frequency, .. } => *frequency,
        }
    }

    pub fn token(&self) -> Option<T> {
        match self {
            Self::Leaf { token, .. } => Some(token.clone()),
            Self::Node { .. } => None,
        }
    }

    pub fn left(&self) -> Option<&HuffmanTree<T>> {
        match self {
            Self::Leaf { .. } => None,
            // deref coercion
            Self::Node { left, .. } => Some(left),
        }
    }

    pub fn right(&self) -> Option<&HuffmanTree<T>> {
        match self {
            Self::Leaf { .. } => None,
            // deref coercion
            Self::Node { right, .. } => Some(right),
        }
    }
}

impl<T: Clone + Eq> Ord for HuffmanTree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency().cmp(&other.frequency())
    }
}

impl<T: Clone + Eq> PartialOrd for HuffmanTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq> Eq for HuffmanTree<T> {}

/// This function takes in a BTreeMap of tokens -> frequency (taf, tokens and frequency)
/// returns a Option of HuffmanTree
///
/// tokens `T` are required to be `Clone` and `Eq`
///
/// A empty map `taf` will result in `None`:
pub fn build_huffman_tree<T: Clone + Eq>(taf: &BTreeMap<T, u64>) -> Option<HuffmanTree<T>> {
    if taf.is_empty() {
        return None;
    }

    let mut min_heap = BinaryHeap::new();
    for pair in taf {
        let leaf = HuffmanTree::Leaf {
            frequency: *pair.1,
            token: pair.0.clone(),
        };
        min_heap.push(Reverse(leaf));
    }

    while min_heap.len() > 1 {
        let right = min_heap.pop().unwrap().0;
        let left = min_heap.pop().unwrap().0;
        let node = HuffmanTree::Node {
            frequency: left.frequency() + right.frequency(),
            left: Box::new(left),
            right: Box::new(right),
        };
        min_heap.push(Reverse(node));
    }

    Some(min_heap.pop().unwrap().0)
}

/// Generates the Huffman coding table from the given Huffman tree
///
/// Returns a `BTreeMap` of `token -> binary sequence`
///
/// # Note
/// Use `BitVec<u8, Msb0>` to align with the standard root-to-leaf traversal path.
/// Pusing `0` (left) or `1` sequentially into an `Msb0` container ensures that
/// the resulting byte stream matches the logical bit order (left to right)
pub fn get_coding_table<T: Clone + Ord + Hash>(
    huf_tree: &HuffmanTree<T>,
) -> BTreeMap<T, BitVec<u8, Msb0>> {
    // TODO:
    // 1. use (alpha, belta) to pattern match a tuple, rather than cur
    // 2. too many `clone`s
    let mut res = BTreeMap::new();

    let mut stack: Vec<(&HuffmanTree<T>, BitVec<u8, Msb0>)> = vec![(huf_tree, BitVec::new())];
    while !stack.is_empty() {
        let cur = stack.pop().unwrap();
        match cur.0 {
            HuffmanTree::Leaf { token: t, .. } => {
                res.insert(t.clone(), cur.1.clone());
            }
            HuffmanTree::Node {
                left: l, right: r, ..
            } => {
                let mut l_code = cur.1.clone();
                l_code.push(false);
                stack.push((&l, l_code));

                let mut r_code = cur.1.clone();
                r_code.push(true);
                stack.push((&r, r_code));
            }
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token_codes() {
        // -- a: 30
        //    ----- b: 15
        //    ----- c: 10
        // a should be 0
        // b should be 10
        // c should be 11
        let t = HuffmanTree::Node {
            frequency: 55,
            left: Box::new(HuffmanTree::Leaf {
                token: String::from("a"),
                frequency: 30,
            }),
            right: Box::new(HuffmanTree::Node {
                frequency: 25,
                left: Box::new(HuffmanTree::Leaf {
                    frequency: 15,
                    token: String::from("b"),
                }),
                right: Box::new(HuffmanTree::Leaf {
                    frequency: 10,
                    token: String::from("c"),
                }),
            }),
        };

        let map = get_coding_table(&t);

        assert_eq!(map.len(), 3, "Should have 3 codes");

        assert_eq!(
            map.get("a"),
            Some(&bits![u8, Msb0; 0].to_bitvec()),
            "Token 'a' code mismatch"
        );
        // let mut expected_a = BitVec::<u8, Msb0>::new();
        // expected_a.push(false); // 0
        // assert_eq!(map.get("a"), Some(&expected_a), "Token 'a' mismatch");

        assert_eq!(
            map.get("b"),
            Some(&bits![u8, Msb0; 1, 0].to_bitvec()),
            "Token 'b' code mismatch"
        );

        assert_eq!(
            map.get("c"),
            Some(&bits![u8, Msb0; 1, 1].to_bitvec()),
            "Token 'c' code mismatch"
        );
    }
}
