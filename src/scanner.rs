use std::{error::Error, fmt, io, path::PathBuf, process::{Command, ExitStatus, Stdio}};

use ffprobe::ffprobe;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Clone)]
pub enum VideoStatus {
    Pending,
    Processing,
    Completed,
    Skipped,
    Failed,
    PostProcessingFailed,
}

impl fmt::Display for VideoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VideoStatus::Pending => write!(f, "Pending"),
            VideoStatus::Processing => write!(f, "Processing"),
            VideoStatus::Completed => write!(f, "Completed"),
            VideoStatus::Skipped => write!(f, "Skipped"),
            VideoStatus::Failed => write!(f, "Failed"),
            VideoStatus::PostProcessingFailed => write!(f, "Post processing failed"),
        }
    }
}

#[derive(Clone)]
pub struct VideoFile {
    path: PathBuf,
    status: VideoStatus
}

impl VideoFile {
    pub fn new(path: PathBuf) -> VideoFile {
        VideoFile { path, status: VideoStatus::Pending }
    }
    
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    
    pub fn status(&self) -> &VideoStatus {
        &self.status
    }
    
    pub fn set_status(&mut self, status: VideoStatus) {
        self.status = status;
    }
    
    pub fn size(&self) -> Option<u64> {
        match self.path.metadata() {
            Ok(metadata) => Some(metadata.len()),
            Err(_) => None
        }
    }
    
    pub fn size_mb(&self) -> Option<u64> {
        match self.size() {
            Some(size) => Some(size / 1024 / 1024),
            None => None
        }
    }
    
    pub fn is_greater_than(&self, video: &VideoFile) -> bool {
        self.size() >= video.size()
    }
    
    pub fn creation_time(&self) -> Option<String> {
        match ffprobe::ffprobe(&self.path) {
            Ok(info) => Some(info.format.tags.and_then(|tags| tags.creation_time)?),
            Err(_) => None
        }
    }
    
    /// Sets the `CreationDate` tag (Apple/QuickTime) using the `CreateDate`
    /// tag (UTC) with the given offset.
    pub fn set_creation_date_with_time_zone(&self, time_zone: String) -> Result<ExitStatus, io::Error> {
        // CreateDate is stored in UTC, so we need the time zone to shift the time accordingly
        let creation_date_arg = format!(r#"-Keys:CreationDate<${{CreateDate;ShiftTime("{}")}}{}"#, time_zone, time_zone);
        
        if self.creation_time().is_none() {
            return Err(io::Error::new(io::ErrorKind::Other, "Creation time not available on video asset"));
        }
        
        Ok(Command::new("exiftool")
            .arg(creation_date_arg)
            .arg("-overwrite_original")
            .arg(self.path())
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()?)
    }
    
    /// Sets the `CreationDate` tag (Apple/QuickTime) using the `CreateDate`
    /// tag (UTC) with the current OS offset.
    pub fn set_creation_date(&self) -> Result<ExitStatus, io::Error> {
        let binding = chrono::Local::now();
        let time_zone = binding.offset();

        self.set_creation_date_with_time_zone(time_zone.to_string())
    }
    
    pub fn duration(&self) -> Option<f32> {
        match ffprobe(self.path()) {
            Ok(info) => {
                let duration_str = info.streams.get(0).and_then(|stream| stream.duration.clone());
                Some(duration_str?.parse::<f32>().unwrap_or(0.0))
            },
            Err(_) => None
        }
    }
    
    pub fn duration_int(&self) -> Option<u64> {
        Some(self.duration()?.round() as u64)
    }
}

pub struct FileScannerConfig<'a> {
    input: PathBuf,
    white_list: &'a Vec<&'a str>
}

impl<'a> FileScannerConfig<'a> {
    pub fn new(input: PathBuf, white_list: &'a Vec<&'a str>) -> FileScannerConfig<'a> {
        FileScannerConfig { input, white_list }
    }
}

pub struct FileScanner<'a> {
    config: FileScannerConfig<'a>,
    assets: Vec<VideoFile>
}

impl<'a> FileScanner<'a> {
    pub fn new(config: FileScannerConfig) -> FileScanner {
        FileScanner { config, assets: vec![] }
    }
    
    fn is_in_white_list(&self, path: &PathBuf) -> bool {
        let extension = path.extension();
        
        match extension {
            Some(extension) => {
                return self.config.white_list.iter().any(|&i| {
                    i == extension.to_ascii_lowercase()
                });
            }
            None => return false
        }
    }
    
    fn is_compressed(&self, path: &PathBuf) -> bool {
        let file_name = path.file_name();
        
        match file_name {
            Some(file_name) => {
                match file_name.to_str() {
                    Some(name) => name.contains("compressed"),
                    None => false
                }
            },
            None => return false
        }
    }
    
    fn add_asset(&mut self, path: PathBuf) {
        if !self.is_in_white_list(&path) {
            return;
        }
        
        if self.is_compressed(&path) {
            return;
        }
        
        let asset = VideoFile::new(path);
        self.assets.push(asset);
    }
    
    fn is_hidden(&mut self, entry: &DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map(|s| s.starts_with("."))
             .unwrap_or(false)
    }
    
    pub fn scan(&mut self) -> Result<Vec<VideoFile>, Box<dyn Error>> {
        let input = &self.config.input;
        
        if !input.exists() {
            let error = io::Error::new(io::ErrorKind::NotFound, "No such file or directory");
            return Err(Box::new(error));
        }
        
        if input.is_file() {
            self.add_asset(self.config.input.clone());
        } else if input.is_dir() {
            let directory: WalkDir = WalkDir::new(input.as_os_str());
            for entry in directory {
                let entry = entry?;
                if !self.is_hidden(&entry) {
                    self.add_asset(entry.into_path());
                }
            }
        }
        
        if self.assets.is_empty() {
            let error = io::Error::new(io::ErrorKind::NotFound, "No valid videos were found");
            return Err(Box::new(error));
        }
        
        Ok(self.assets.clone())
    }
}