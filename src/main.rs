use clap::{Parser, Subcommand};
use std::{
    fs::File, path::PathBuf,
};

mod core;
mod huffman;
mod service;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// compress a file
    Compress {
        /// The source file that you want to compress.
        src: PathBuf,
        /// The dest file path to store compressed file
        dest: PathBuf,
    },
    /// extract a ruf-compressed file
    Extract {
        /// The source file that you want to extract
        src: PathBuf,
        /// The dest file path to store extracted file
        dest: PathBuf,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Compress { 
            src, 
            dest 
        } => {
            let src_f = File::open(src).unwrap();
            let mut dest_f = File::create_new(dest).unwrap();
            service::compress_file(&src_f, &mut dest_f);
        },
        Commands::Extract { 
            src, 
            dest 
        } => {
            let src_f = File::open(src).unwrap();
            let mut dest_f = File::create_new(dest).unwrap();
            service::extract_file(&src_f, &mut dest_f);
        }
    }
}
