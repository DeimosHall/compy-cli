use std::{error::Error, fs, io, path::PathBuf, process::{Command, ExitStatus, Stdio}};

use crate::{Cli, scanner::{VideoFile, VideoStatus}, utils};

fn compress_asset(asset: &mut VideoFile, compressed_file_name: &PathBuf) -> Result<ExitStatus, Box<dyn Error>> {
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

// TODO: set time zone
fn set_creation_date(asset: &VideoFile) -> Result<ExitStatus, Box<dyn Error>> {
    Ok(Command::new("exiftool")
        .arg("-Keys:CreationDate<${CreateDate}-06:00")
        .arg("-overwrite_original")
        .arg(asset.path())
        .stderr(Stdio::null())
        .status()?
    )
}

fn delete_file(asset: &VideoFile) -> Result<(), io::Error> {
    println!("Deleting {}", asset.path().display());
    fs::remove_file(asset.path())?;
    Ok(())
}

fn verify_successfull_compression(original: &mut VideoFile, compressed_file: PathBuf, cli: &Cli) -> Result<(), String> {
    let compressed_video = VideoFile::new(compressed_file);
    if compressed_video.is_greater_than(&original) {
        original.set_status(VideoStatus::Failed);
        eprintln!("Compressed is greater than original");
        let original_size = original.size_mb().ok_or(format!("Error reading {} file size", original.path().display()));
        let compressed_size = compressed_video.size_mb().ok_or(format!("Error reading {} file size", compressed_video.path().display()));
        eprintln!("Original: {}, compressed: {}", original_size?, compressed_size?);
        // TODO: Show error message
        let _ = delete_file(&compressed_video);
    } else {
        // TODO: also show error here
        let _ = set_creation_date(&compressed_video);
        cli.delete.then(|| { delete_file(original) });
    }
    
    Ok(())
}

pub fn process_asset(asset: &mut VideoFile, cli: &Cli) -> Result<(), Box<dyn Error>> {
    asset.set_status(VideoStatus::Processing);
    let compressed_file_name = utils::get_compressed_file_name(&asset.path())?;
    
    if compressed_file_name.exists() {
        println!("{} is already compressed", asset.path().display());
        asset.set_status(VideoStatus::Skipped);
        return Ok(());
    }
    
    println!("Compressing {}", asset.path().display());
    let status = compress_asset(asset, &compressed_file_name);
    
    if status?.success() {
        asset.set_status(VideoStatus::Completed);
        verify_successfull_compression(asset, compressed_file_name, cli)?;
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