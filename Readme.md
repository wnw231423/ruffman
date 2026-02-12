# Ruffman
A naive Rust implementation of Huffman encoding algorithm. This is a self-practice project.

## Usage
- `ruf compress <src> <dest>`: compress file `src` into file `dest`.
- `ruf extract <src> <dest>`: extract file `src` into file `dest`.

## TODO list
- [x] Improve CLI with `clap`.
- [x] Improve compression ratio.
- [ ] Speed up compression speed by making use of concurrency.
- [ ] Support directory level compression and extraction. 

## Note and Idea
### Bits' things
Since `u8`(byte) is the least unit for memory access, I need something to manipulating bits for Huffman Tree encoding. We can achieve this by write a 'Bit Buffer' manually or use existing crate. I chose to use crate `bitvec` because I'm curious about the process of reading doc and putting existing wheels into use.

I picked `rmp-serde` to serialize the compressed data. During serializing,  the `BitVec` would costs 50% more storage and so do the `Vec<u8>`. But we can do [optimization](https://docs.rs/rmp-serde/latest/rmp_serde/#efficient-storage-of-u8-types) on `Vec<u8>` so I chose it.

### Traits' things
#### from `PartialEq` to `Ord`
I save the encoder in the form of frequency table rather than the huffman tree to save space. During extracting, I read the frequency table and rebuild the huffman tree. To make the tree of compressing process the same tree of extracting process, I use a `BTreeMap` to store the frequency and a `MinHeap` to build the tree, which requires the `Ord` trait.

The logic of implementing `Ord` is: `PartialEq` -> `Eq` -> `PartialOrd` -> `Ord`. The mathematical concepts within is interesting for warming up.

#### Advanced traits
I tried some free and functional programming style code and encounter tons of difficulty. The "trait bound" syntax and its syntactic sugar `impl <trait>` seems to have different behaviors. There are 8 cases:
- use "trait bound"/`impl <trait>` in a function's argument/"return type" position.
- use "trait bound"/`impl <trait>` in the argument/"return type" position of a `Fn` trait-like (`Fn`, `FnMut` and `FnOnce`).

The involved things are far beyond my initial intention, so I refactored and use a much more naive and simpler implementation. 

