use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

use crate::core::*;

/// This module contains all the service logic
/// of this `ruffman` program.

fn file_to_bytes(file: &File) -> Vec<u8> {
    let mut reader = BufReader::new(file);
    let mut res: Vec<u8> = Vec::new();
    let _ = reader.read_to_end(&mut res);
    res
}

pub fn compress_file(src: &File, dest: &mut File) {
    let tokens = file_to_bytes(src);
    let buf = compress(&tokens);
    let _ = dest.write(&buf);
}

pub fn extract_file(src: &File, dest: &mut File) {
    let mut reader = BufReader::new(src);
    let mut buf = Vec::new();
    let _ = reader.read_to_end(&mut buf);
    let data: Vec<u8> = extract(&buf);
    let _ = dest.write(&data);
}
