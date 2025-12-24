use std::process::ExitCode;
use std::{error::Error, process};
use std::path::{PathBuf};
use clap::{Parser};

use crate::scanner::{FileScanner, FileScannerConfig, VideoFile};

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

fn list_files(assets: &Vec<VideoFile>) {
    println!("Videos to compress:");
    for asset in assets {
        println!("{}", asset.path().display());
    }
}

mod scanner;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    let input = match cli.input {
        Some(input) => input,
        None => return Err("No input provided".into())
    };
    
    let white_list: Vec<&'static str> = vec!["mp4", "mkv"];
    let config = FileScannerConfig::new(input, white_list);
    let mut file_scanner = FileScanner::new(config);
    
    let assets = file_scanner.scan()?;
    
    if cli.list {
        list_files(&assets);
        return Ok(());
    }
    
    cli.delete.then(|| { delete_original() });
    
    Ok(())
}