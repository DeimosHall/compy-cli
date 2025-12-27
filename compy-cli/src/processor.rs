use std::{error::Error, path::PathBuf, process::{Command, ExitStatus, Stdio}};

use crate::{scanner::{VideoFile, VideoStatus}, utils};

fn compress_asset(asset: &mut VideoFile, compressed_file_name: PathBuf) -> Result<ExitStatus, Box<dyn Error>> {
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
        .arg(compressed_file_name)
        .arg("-v")
        .arg("warning")
        .arg("-hide_banner")
        .arg("-stats")
        .stderr(Stdio::null())
        .status()?)
}

pub fn process_asset(asset: &mut VideoFile) -> Result<(), Box<dyn Error>> {
    asset.set_status(VideoStatus::Processing);
    let compressed_file_name = utils::get_compressed_file_name(&asset.path())?;
    
    if compressed_file_name.exists() {
        println!("{} is already compressed", asset.path().display());
        asset.set_status(VideoStatus::Skipped);
        return Ok(());
    }
    
    println!("Compressing {}", asset.path().display());
    let status = compress_asset(asset, compressed_file_name);
    
    if status?.success() {
        asset.set_status(VideoStatus::Completed);
    } else {
        eprintln!("Error compressing {}", asset.path().display());
        asset.set_status(VideoStatus::Failed);
    }
    
    Ok(())
}

pub fn process_assets(assets: &mut Vec<VideoFile>) -> Result<(), Box<dyn Error>> {
    for asset in assets {
        process_asset(asset)?;
    }
    
    Ok(())
}