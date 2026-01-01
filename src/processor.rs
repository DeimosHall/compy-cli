use std::{error::Error, process::{Command, ExitStatus, Stdio}};

use crate::{Cli, errors::CompressionError, scanner::{VideoFile, VideoStatus}, utils};

fn compress_asset(asset: &mut VideoFile, asset_to_compress: &VideoFile) -> Result<ExitStatus, Box<dyn Error>> {
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
        .arg(asset_to_compress.path())
        .arg("-v")
        .arg("warning")
        .arg("-hide_banner")
        .arg("-stats")
        .stderr(Stdio::null())
        .status()?)
}

fn verify_successfull_compression(original: &mut VideoFile, compressed: &VideoFile, cli: &Cli) -> Result<(), CompressionError> {
    if compressed.is_greater_than(&original) {
        original.set_status(VideoStatus::Failed);
        let original_size = original.size_mb().ok_or(CompressionError::FileSizeError("Error reading original file size".to_string()))?;
        let compressed_size = compressed.size_mb().ok_or(CompressionError::FileSizeError("Error reading compressed file size".to_string()))?;
        
        if let Err(e) = utils::delete_file(&compressed) {
            let err_msg = format!("Error deleting {}", &compressed.path().display(), );
            return Err(CompressionError::IoError(err_msg, e));
        }
        
        let err_msg = format!("Compressed is greater than original. Original: {} MB, compressed: {} MB", original_size, compressed_size);
        return Err(CompressionError::CompressionFailed(err_msg));
    } else {
        // TODO: handle the time zone properly
        if let Err(e) = utils::set_creation_date(&compressed, String::from("-06:00")) {
            let err_msg = format!("Error setting creation date to {}", &compressed.path().display());
            return Err(CompressionError::DateError(err_msg, e));
        }
        
        if cli.delete {
            if let Err(e) = utils::delete_file(original) {
                let err_msg = format!("Error deleting {}", &original.path().display());
                return Err(CompressionError::IoError(err_msg, e));
            }
        }
        
        Ok(())
    }
}

pub fn process_asset(asset: &mut VideoFile, cli: &Cli) -> Result<(), Box<dyn Error>> {
    asset.set_status(VideoStatus::Processing);
    let compressed_file_name = utils::get_compressed_file_name(&asset.path())?;
    let asset_to_compress = VideoFile::new(compressed_file_name);
    
    if asset_to_compress.path().exists() {
        println!("{} is already compressed", asset.path().display());
        asset.set_status(VideoStatus::Skipped);
        return Ok(());
    }
    
    println!("Compressing {}", asset.path().display());
    let status = compress_asset(asset, &asset_to_compress);
    
    if status?.success() {
        asset.set_status(VideoStatus::Completed);
        verify_successfull_compression(asset, &asset_to_compress, cli)?;
    } else {
        eprintln!("Error compressing {}", asset.path().display());
        asset.set_status(VideoStatus::Failed);
    }
    
    Ok(())
}

pub fn process_assets(assets: &mut Vec<VideoFile>, cli: &Cli) -> Result<(), Box<dyn Error>> {
    for asset in assets {
        process_asset(asset, cli)?;
    }
    
    Ok(())
}