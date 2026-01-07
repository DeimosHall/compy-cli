use std::process;
use std::path::{PathBuf};
use clap::{Parser};

use crate::processor::VideoProcessor;
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
    list: bool,
    
    /// Show a list of valid file formats
    #[arg(short, long)]
    allowed: bool
}

struct AppConfig {
    delete: bool,
}

impl AppConfig {
    pub fn new(cli: &Cli) -> AppConfig {
        AppConfig { delete: cli.delete }
    }
}

mod scanner;
mod utils;
mod processor;
pub mod asset_handler;
pub mod errors;

fn main() {
    let cli = Cli::parse();
    let white_list: Vec<&'static str> = vec!["mp4", "mov", "mkv"];
    
    if cli.allowed {
        for (index, item) in white_list.iter().enumerate() {
            println!("{} - {}", index + 1, item);
        }
        return;
    }
    
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
    
    let config = FileScannerConfig::new(input.to_path_buf(), &white_list);
    let mut file_scanner = FileScanner::new(config);
    
    let mut assets = file_scanner.scan().unwrap_or_else(|e| {
        eprintln!("{}", e);
        eprintln!("Run `compy --allowed` to see a list of valid video formats");
        process::exit(1);
    });
    
    if cli.list {
        utils::list_files(&assets);
        return;
    }
    
    let app_config = AppConfig::new(&cli);
    let video_processor = VideoProcessor::new();
    processor::process_videos(&video_processor, &mut assets, &app_config);
    utils::report_summary(&assets);
}
