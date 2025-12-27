use std::{error::Error, process::{Command, Stdio}};

use crate::{scanner::{VideoFile, VideoStatus}, utils};

pub fn compress_asset(asset: &mut VideoFile) -> Result<(), Box<dyn Error>> {
    asset.set_status(VideoStatus::Processing);
    let new_file_name = utils::get_compressed_file_name(&asset.path())?;
    
    if new_file_name.exists() {
        println!("{} is already compressed", asset.path().display());
        asset.set_status(VideoStatus::Skipped);
        return Ok(());
    }
    
    println!("Compressing {}", asset.path().display());
    
    let status = Command::new("ffmpeg")
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
        .arg(new_file_name)
        .arg("-v")
        .arg("warning")
        .arg("-hide_banner")
        .arg("-stats")
        .stderr(Stdio::null())
        .status();
    
    if status?.success() {
        asset.set_status(VideoStatus::Completed);
    } else {
        eprintln!("Error compressing {}", asset.path().display());
        asset.set_status(VideoStatus::Failed);
    }
    
    Ok(())
}

pub fn compress_assets(assets: &mut Vec<VideoFile>) -> Result<(), Box<dyn Error>> {
    for asset in assets {
        compress_asset(asset)?;
    }
    
    Ok(())
}