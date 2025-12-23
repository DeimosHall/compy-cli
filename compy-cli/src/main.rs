use std::{error::Error, process};
use std::path::{PathBuf};
use clap::{Parser};

use crate::scanner::{FileScanner, FileScannerConfig};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to a single video or a directory
    input: Option<PathBuf>,
    
    /// Delete original file only after successful compression
    #[arg(short, long)]
    delete: bool,
    
    /// List the files to be processed and exits without compressing
    #[arg(short, long)]
    list: bool
}

fn delete_original() {
    println!("Deleting original file...");
}

fn list_files() {
    println!("Listing compatible files...");
}

mod scanner;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    let input = match cli.input {
        Some(input) => input,
        None => {
            eprintln!("No input provided");
            process::exit(1);
        }
    };
    println!("Input: {:?}", input);
    
    let white_list: Vec<&'static str> = vec!["mp4", "mkv"];
    let config = FileScannerConfig::new(input, white_list);
    let mut file_scanner = FileScanner::new(config);
    
    let assets = file_scanner.scan();
    
    match assets {
        Ok(assets) => {
            println!("Files:");
            for asset in assets {
                println!("Path: {}", asset.path().display());
            }
        },
        Err(e) => {
            eprintln!("Operation failed: {}", e);
            process::exit(1);
        }
    }
    
    cli.delete.then(|| { delete_original() });
    cli.list.then(|| { list_files() });
    
    Ok(())
}