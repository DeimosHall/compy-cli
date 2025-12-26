use std::{error::Error};
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    let input = match cli.input {
        Some(input) => input,
        None => return Err("No input provided".into())
    };
    
    if !utils::is_ffmpeg_installed() {
        return Err("FFmpeg is not installed".into());
    }
    
    let white_list: Vec<&'static str> = vec!["mp4", "mkv"];
    let config = FileScannerConfig::new(input, white_list);
    let mut file_scanner = FileScanner::new(config);
    
    let mut assets = file_scanner.scan()?;
    
    if cli.list {
        utils::list_files(&assets);
        return Ok(());
    }
    
    processor::compress_assets(&mut assets)?;
    utils::report_summary(&assets);
    Ok(())
}