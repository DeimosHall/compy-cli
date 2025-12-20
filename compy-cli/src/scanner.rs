use std::path::{PathBuf};

use walkdir::WalkDir;

#[derive(Debug, Clone)]
enum VideoStatus {
    Pending,
    Processing,
    Completed,
    Skipped,
    Failed,
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
    
    pub fn path(self) -> PathBuf {
        self.path
    }
    
    pub fn status(self) -> VideoStatus {
        self.status
    }
    
    pub fn set_status(mut self, status: VideoStatus) {
        self.status = status;
    }
}

pub struct FileScannerConfig<'a> {
    input: PathBuf,
    white_list: Vec<&'a str>
}

impl<'a> FileScannerConfig<'a> {
    pub fn new(input: PathBuf, white_list: Vec<&'a str>) -> FileScannerConfig<'a> {
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
    
    fn is_video_file(&self, path: &PathBuf) -> bool {
        if !path.exists() {
            return false;
        }
        
        // logic here
        
        true
    }
    
    fn add_asset(&mut self, path: PathBuf) {
        if !self.is_video_file(&path) {
            return;
        }
        
        // Skip check white list for now
        let asset = VideoFile::new(path);
        self.assets.push(asset);
    }
    
    pub fn scan(&mut self) -> Vec<VideoFile> {
        let input = &self.config.input;
        
        if !input.exists() {
            return vec![]
        }
        
        if input.is_file() {
            self.add_asset(self.config.input.clone());
        } else if input.is_dir() {
            let directory: WalkDir = WalkDir::new(input.as_os_str());
            for entry in directory {
                match entry {
                    Ok(entry) => self.add_asset(entry.into_path()),
                    Err(_) => eprintln!(""),
                }
            }
        }
        
        self.assets.clone()
    }
}