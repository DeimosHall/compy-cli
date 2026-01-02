use std::{fs, io, path::PathBuf, process::{Command, ExitStatus, Stdio}};

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
        println!("{} - {:?}", &asset.path().display(), &asset.status());
    }
}

pub fn delete_file(asset: &VideoFile) -> Result<(), io::Error> {
    println!("Deleting {}", asset.path().display());
    fs::remove_file(asset.path())?;
    Ok(())
}

// TODO: status doesn't indicate if the command was able to modify the date
pub fn set_creation_date(asset: &VideoFile, time_zone: String) -> Result<ExitStatus, io::Error> {
    let creation_date_arg = format!(r#"-Keys:CreationDate<${{CreateDate;ShiftTime("{}")}}{}"#, time_zone, time_zone);
    println!("Creation time: {}", asset.creation_time().unwrap_or("unknown".to_string()));
    
    if asset.creation_time().is_none() {
        return Err(io::Error::new(io::ErrorKind::Other, "Creation time not available on video asset"));
    }
    
    let result = Command::new("exiftool")
        .arg(creation_date_arg)
        .arg("-overwrite_original")
        .arg(asset.path())
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .output()?;
    
    Ok(result.status)
}