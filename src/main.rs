use std::{
    env,
    fs::File,
};

mod core;
mod huffman;
mod service;

fn main() {
    let args: Vec<_> = env::args().collect();
    let op = &args[1];
    let src = &args[2];
    let dest = &args[3];
    let src_f = File::open(src).unwrap();
    let mut dest_f = File::create_new(dest).unwrap();
    match op.as_str() {
        "compress" => {
            service::compress_file(&src_f, &mut dest_f);
        }
        "extract" => {
            service::extract_file(&src_f, &mut dest_f);
        }
        _ => {
            panic!("unknown operation.");
        }
    }
}
