use std::{error::Error, io, path::PathBuf};

use walkdir::{DirEntry, WalkDir};

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
    
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    
    pub fn status(&self) -> &VideoStatus {
        &self.status
    }
    
    pub fn set_status(&mut self, status: VideoStatus) {
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
    
    fn add_asset(&mut self, path: PathBuf) {
        if !self.is_in_white_list(&path) {
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
        
        Ok(self.assets.clone())
    }
}