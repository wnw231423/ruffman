/// This module contains the core logic of compression and extraction.
use crate::huffman::*;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;

#[derive(Serialize, Deserialize)]
struct CompressedData<T: Ord + Hash> {
    encoder: BTreeMap<T, u64>,
    data: BitVec<u8, Msb0>,
}

/// Compress a File into a Vec<u8>
/// you can decide what token you want to use
pub fn compress<T>(tokens: &Vec<T>) -> Vec<u8>
where
    T: Clone + Ord + Hash + Serialize,
{
    // TODO: Serialze data represented by `Vec<u8>` rather than `bitvec`, 
    // saving space of metadata of `bitvec`, to get a better compression ratio.
    let mut frequency_table: BTreeMap<T, u64> = BTreeMap::new();

    // generate frequency table
    for token in tokens {
        let freq = frequency_table.get_mut(&token);
        match freq {
            Some(f) => {
                *f = *f + 1;
            }
            None => {
                frequency_table.insert(token.clone(), 1);
            }
        }
    }

    // build huffman tree
    let tree = build_huffman_tree(&frequency_table).unwrap();
    let code_table = get_coding_table(&tree);

    // generate compressed data
    let mut data = bitvec![u8, Msb0;];
    for token in tokens {
        let token_code = code_table.get(&token).unwrap();
        data.extend(token_code);
    }

    let compressed_data = CompressedData {
        encoder: frequency_table,
        data: data,
    };

    rmp_serde::to_vec(&compressed_data).unwrap()
}

pub fn extract<'de, T>(buf: &'de Vec<u8>) -> Vec<T>
where
    T: Clone + Ord + Hash + Deserialize<'de>,
{
    let compressed_data: CompressedData<T> = rmp_serde::from_slice(buf).unwrap();

    // restore the huffman tree and the coding table
    let tree = build_huffman_tree(&compressed_data.encoder).unwrap();
    let coding_table = get_coding_table(&tree);

    // get decoder, a BTreeMap of code -> token
    let decoder = {
        let mut decoder = BTreeMap::new();
        for (token, code) in coding_table {
            decoder.insert(code, token);
        }
        decoder
    };

    // restore original token vector
    let data = compressed_data.data;
    let mut tokens = Vec::new();
    let mut temp = bitvec![u8, Msb0;];
    for i in 0..data.len() {
        temp.push(data[i]);
        if let Some(token) = decoder.get(&temp) {
            tokens.push(token.clone());
            temp = bitvec![u8, Msb0;];
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
