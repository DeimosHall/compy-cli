use std::process;
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

mod scanner;
mod utils;
mod processor;
pub mod errors;

fn main() {
    let cli = Cli::parse();
    
    let input = match cli.input {
        Some(ref input) => input,
        None => {
            eprintln!("No input provided");
            process::exit(1);
        }
    };
    
    if !utils::is_ffmpeg_installed() {
        eprintln!("FFmpeg is not installed");
        process::exit(1);
    }
    
    let white_list: Vec<&'static str> = vec!["mp4", "mkv"];
    let config = FileScannerConfig::new(input.to_path_buf(), white_list);
    let mut file_scanner = FileScanner::new(config);
    
    let mut assets = file_scanner.scan().unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });
    
    if cli.list {
        utils::list_files(&assets);
        return;
    }
    
    if let Err(e) = processor::process_assets(&mut assets, &cli) {
        eprintln!("Application error: {}", e);
    }
    
    utils::report_summary(&assets);
}