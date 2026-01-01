use std::{error::Error, process::{Command, ExitStatus, Stdio}};

use crate::{Cli, scanner::{VideoFile, VideoStatus}, utils};

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

fn verify_successfull_compression(original: &mut VideoFile, compressed: &VideoFile, cli: &Cli) -> Result<(), String> {
    if compressed.is_greater_than(&original) {
        original.set_status(VideoStatus::Failed);
        let original_size = original.size_mb().ok_or(format!("Error reading {} file size", original.path().display()));
        let compressed_size = compressed.size_mb().ok_or(format!("Error reading {} file size", compressed.path().display()));
        eprintln!("Compressed is greater than original");
        eprintln!("Original: {} MB, compressed: {} MB", original_size?, compressed_size?);
        // TODO: Show error message
        let _ = utils::delete_file(&compressed);
    } else {
        // TODO: also show error here and handle the time zone properly
        let _ = utils::set_creation_date(&compressed, String::from("-06:00"));
        cli.delete.then(|| { utils::delete_file(original) });
    }
    
    Ok(())
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