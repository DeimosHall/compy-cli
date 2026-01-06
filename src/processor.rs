use std::{error::Error, io::{self, BufRead, BufReader}, process::{self, Command, Stdio}, thread};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use regex::Regex;

use crate::{Cli, asset_handler::{VideoFile, AssetStatus}, errors::CompressionError, utils};
use crate::asset_handler::MediaAsset;

pub trait Processor {
    fn process(&self, index: &usize, total: &usize, asset: &mut VideoFile, cli: &Cli) -> Result<(), Box<dyn Error>>;
}

pub struct VideoProcessor { }

impl Processor for VideoProcessor {
    fn process(&self, index: &usize, total: &usize, asset: &mut VideoFile, cli: &Cli) -> Result<(), Box<dyn Error>> {
        asset.set_status(AssetStatus::Processing);
        
        let compressed_file_name = utils::get_compressed_file_name(&asset.path())?;
        let compressed_asset = VideoFile::new(compressed_file_name);
        
        if compressed_asset.path().exists() {
            asset.set_status(AssetStatus::Skipped);
            return Ok(());
        }
        
        // TODO: Fix strange logic here, only send asset, return a tuble with status and
        // compressed_asset
        let mut process = compress_asset(asset, &compressed_asset)?;
        let stderr = process.stderr.take().expect("Failed to capture stderr");
        
        let multi_progress_bar = MultiProgress::new();
        let duration = asset.duration_int().unwrap_or(0);
        let style = ProgressStyle::default_bar()
            .template("{msg}\n[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .expect("Failed to create progress bar style");
        let progress_bar = multi_progress_bar.add(ProgressBar::new(duration));
        progress_bar.set_style(style);
        progress_bar.set_message(format!("[{}/{}] Compressing - {}", index, total, asset.path().display()));
        
        let regex = Regex::new(r"time=(?P<hh>\d{2}):(?P<mm>\d{2}):(?P<ss>\d{2})\.(?P<ms>\d{2})").unwrap();
        
        thread::spawn(move || {
            let mut reader = BufReader::new(stderr);
            let mut buffer = Vec::new();
            
            while let Ok(n) = reader.read_until(b'\r', &mut buffer) {
                if n == 0 { break; }
                let line = String::from_utf8_lossy(&buffer);
                
                if let Some(caps) = regex.captures(&line) {
                    progress_bar.set_position(utils::captures_to_seconds(&caps));
                }
                
                buffer.clear();
            }
        });
        
        if process.wait().unwrap().success() {
            verify_successfull_compression(asset, &compressed_asset, cli)?;
        } else {
            asset.set_status(AssetStatus::Failed);
        }
        
        Ok(())
    }
}

fn compress_asset(asset: &mut VideoFile, destination_asset: &VideoFile) -> Result<process::Child, io::Error> {
    Ok(Command::new("ffmpeg")
        .arg("-i")
        .arg(asset.path())
        .arg("-vcodec")
        .arg("libx264")
        .arg("-crf")
        .arg("23")
        .arg("-acodec")
        .arg("aac")
        .arg("-b:a")
        .arg("128k")
        .arg("-map_metadata")
        .arg("0")
        .arg(destination_asset.path())
        .arg("-v")
        .arg("warning")
        .arg("-hide_banner")
        .arg("-stats")
        .stderr(Stdio::piped())
        .spawn()?)
}

fn verify_successfull_compression(original: &mut VideoFile, compressed: &VideoFile, cli: &Cli) -> Result<(), CompressionError> {
    if compressed.is_greater_than(&original) {
        original.set_status(AssetStatus::Failed);
        
        let original_size = original.size_mb()
            .ok_or(CompressionError::FileSizeError("Error reading original file size".to_string()))?;
        let compressed_size = compressed.size_mb()
            .ok_or(CompressionError::FileSizeError("Error reading compressed file size".to_string()))?;
        
        if let Err(e) = utils::delete_file(&compressed) {
            let err_msg = format!("Error deleting {}", &compressed.path().display(), );
            return Err(CompressionError::IoError(err_msg, e));
        }
        
        let err_msg = format!("Compressed is greater than original. Original: {} MB, compressed: {} MB", original_size, compressed_size);
        return Err(CompressionError::CompressionFailed(err_msg));
    } else {
        if let Err(e) = compressed.set_creation_date() {
            original.set_status(AssetStatus::PostProcessingFailed);
            let err_msg = format!("Error setting creation date to {},", &compressed.path().display());
            return Err(CompressionError::DateError(err_msg, e));
        }
        
        if cli.delete {
            if let Err(e) = utils::delete_file(original) {
                original.set_status(AssetStatus::PostProcessingFailed);
                let err_msg = format!("Error deleting {}", &original.path().display());
                return Err(CompressionError::IoError(err_msg, e));
            }
        }
        
        original.set_status(AssetStatus::Completed);
        
        Ok(())
    }
}

pub fn process_assets<P: Processor>(processor: &P, assets: &mut Vec<VideoFile>, cli: &Cli) {
    let total = assets.len();
    for (index, asset) in assets.iter_mut().enumerate() {
        processor.process(&(index + 1), &total, asset, cli).unwrap_or_else(|e| {
            eprintln!("{}", e);
        });
    }
}
