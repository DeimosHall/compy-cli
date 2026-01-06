use std::{error::Error, io, path::PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::asset_handler::VideoFile;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    
    fn get_white_list() -> Vec<&'static str> {
        vec!["mp4", "mov", "mkv"]
    }

    fn create_empty_dir(dir: &str) {
        fs::create_dir(dir).unwrap_or_else(|_| {delete_dir(dir);})
    }
    
    fn delete_dir(dir: &str) {
        fs::remove_dir_all(dir).unwrap_or_else(|_| {});
    }
    
    fn create_data_test(files: &Vec<&str>, dir: &str) {
        for file in files {
            let file = format!("{}/{}", dir, file);
            File::create(file).unwrap();
        }
    }
    
    #[test]
    fn test_empty_dir() {
        let dir = "assets";
        let path = PathBuf::from(dir);
        let white_list = get_white_list();
        
        create_empty_dir(dir);
        
        let config = FileScannerConfig::new(path, &white_list);
        let mut scanner = FileScanner::new(config);
        
        if let Err(error) = scanner.scan() {
            delete_dir(dir);
            assert!(error.is::<io::Error>());
            return;
        }
        delete_dir(dir);
        assert!(false);  // It shouldn't reach here
    }

    #[test]
    fn test_dir_with_data() {
        let dir = "assets2";
        let files = vec!["video1.mp4", "video2.mov", "video3.mkv", "document.txt", "image.png", "script.sh"];
        let path = PathBuf::from(dir);
        let white_list = get_white_list();
        
        create_empty_dir(dir);
        create_data_test(&files, &dir);
        
        let config = FileScannerConfig::new(path, &white_list);
        let mut scanner = FileScanner::new(config);
        
        let assets = scanner.scan().unwrap();
        delete_dir(dir);
        
        assert_eq!(assets.len(), 3);
    }
}