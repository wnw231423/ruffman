/// This module contains the core logic of compression and extraction.
use crate::huffman::*;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::hash::Hash;

#[derive(Serialize, Deserialize)]
struct CompressedData<T: Ord + Hash> {
    encoder: BTreeMap<T, u64>,  // the frequency table
    
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,              // the data type is Vec<u8>, we use `serde_bytes` crate to improve the storage efficiency.
    
    bit_len: usize,             // the data may be not aligned to 8bit, so record the bit length. 2^64bit = 2^61Byte, should be enough.
}

/// Compress a File into a Vec<u8>
/// you can decide what token you want to use
pub fn compress<T>(tokens: &Vec<T>) -> Vec<u8>
where
    T: Clone + Ord + Hash + Serialize + Send + Sync,
{
    // generate frequency table

    // let mut frequency_table: BTreeMap<T, u64> = BTreeMap::new();
    // for token in tokens {
    //     let freq = frequency_table.get_mut(&token);
    //     match freq {
    //         Some(f) => {
    //             *f = *f + 1;
    //         }
    //         None => {
    //             frequency_table.insert(token.clone(), 1);
    //         }
    //     }
    // }

    // this piece of code make use of `rayon` crate for parallelism
    // to coping with par_iter, use functional programming style.
    let frequency_table = tokens.par_iter()
            .fold(|| BTreeMap::new(), |mut map: BTreeMap<T, u64>, token: &T| {
                *map.entry(token.clone()).or_insert(0) += 1;
                map
            })
            .reduce(|| BTreeMap::new(), |mut map1, map2| {
                map2.into_iter()
                    .for_each(|(t, f)| {
                        *map1.entry(t).or_insert(0) += f;
                    });
                map1
            });

    // build huffman tree
    let tree = build_huffman_tree(&frequency_table).unwrap();
    let code_table = get_coding_table(&tree);

    // generate compressed data

    // let mut data = bitvec![u8, Msb0;];
    // for token in tokens {
    //     let token_code = code_table.get(&token).unwrap();
    //     data.extend(token_code);
    // }

    let data = tokens.par_iter()
            .fold(|| bitvec![u8, Msb0;], |mut bv, token| {
                let token_code = code_table.get(token).unwrap();
                bv.extend(token_code);
                bv    
            })
            .reduce(|| bitvec![u8, Msb0;], |mut bv1, bv2| {
                bv1.extend(bv2);
                bv1
            });

    let len = data.len();
    let data = data.into_vec();
    
    let compressed_data = CompressedData {
        encoder: frequency_table,
        data: data,
        bit_len: len,
    };

    rmp_serde::to_vec(&compressed_data).unwrap()
}

pub fn extract<'de, T>(buf: &'de Vec<u8>) -> Vec<T>
where
    T: Clone + Ord + Hash + Deserialize<'de>,
{
    let compressed_data: CompressedData<T> = rmp_serde::from_slice(buf).unwrap();

    // restore the huffman tree from the coding table
    let tree = build_huffman_tree(&compressed_data.encoder).unwrap();

    // restore original token vector by walking on the huffman tree
    let data: BitVec<u8, Msb0> = BitVec::from_slice(&compressed_data.data);
    let mut tokens = Vec::new();
    let mut current_walk = &tree;
    for i in 0..compressed_data.bit_len {
        if data[i] == false {
            current_walk = current_walk.left().unwrap();
        } else {
            current_walk = current_walk.right().unwrap();
        }

        match current_walk {
            HuffmanTree::Leaf {token, .. } => {
                tokens.push(token.clone());
                current_walk = &tree;
            }
            HuffmanTree::Node { .. } => {
                // do nothing
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_to_hello() -> Vec<u8> {
        let s = String::from("Hello, world!");
        let data = s.as_bytes();
        data.to_vec()
    }

    #[test]
    fn test_hello_world() {
        let hello = input_to_hello();
        let compressed_data = compress(&hello);
        let restored_data: Vec<u8> = extract(&compressed_data);
        assert_eq!(hello, restored_data);
    }
}
