use std::{fs, io, path::PathBuf, process::{Command, Stdio}};

use crate::scanner::VideoFile;

pub fn is_ffmpeg_installed() -> bool {
    let status = Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    
    match status {
        Ok(status) => status.success(),
        Err(_) => false
    }
}

pub fn list_files(assets: &Vec<VideoFile>) {
    println!("Videos to compress:");
    for asset in assets {
        println!("{}", asset.path().display());
    }
}

pub fn get_compressed_file_name(path: &PathBuf) -> Result<PathBuf, String> {
    let file_name = format!("{} compressed", path.file_stem().ok_or("Error reading file name")?.display());
    let extension = path.extension().ok_or("Error reading file extension")?;
    Ok(path.with_file_name(file_name).with_extension(extension))
}

pub fn report_summary(assets: &Vec<VideoFile>) {
    println!("\nSummary...");
    for asset in assets {
        println!("{} - {}", &asset.path().display(), &asset.status());
    }
}

pub fn delete_file(asset: &VideoFile) -> Result<(), io::Error> {
    println!("Deleting {}", asset.path().display());
    fs::remove_file(asset.path())?;
    Ok(())
}
