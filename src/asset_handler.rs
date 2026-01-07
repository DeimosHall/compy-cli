use std::{fmt, io, path::PathBuf, process::{Command, ExitStatus, Stdio}};

use ffprobe::ffprobe;

#[derive(Debug, Clone)]
pub enum AssetStatus {
    Pending,
    Processing,
    Completed,
    Skipped,
    Failed,
    PostProcessingFailed,
}

impl fmt::Display for AssetStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetStatus::Pending => write!(f, "Pending"),
            AssetStatus::Processing => write!(f, "Processing"),
            AssetStatus::Completed => write!(f, "Completed"),
            AssetStatus::Skipped => write!(f, "Skipped"),
            AssetStatus::Failed => write!(f, "Failed"),
            AssetStatus::PostProcessingFailed => write!(f, "Post processing failed"),
        }
    }
}

pub trait MediaAsset {
    fn path(&self) -> &PathBuf;
    
    fn status(&self) -> &AssetStatus;
    
    fn set_status(&mut self, status: AssetStatus);
    
    fn size(&self) -> Option<u64>;
    
    fn size_mb(&self) -> Option<u64>;
    
    fn is_greater_than(&self, video: &VideoFile) -> bool;
}

#[derive(Clone)]
pub struct VideoFile {
    path: PathBuf,
    status: AssetStatus
}

impl MediaAsset for VideoFile {
    fn path(&self) -> &PathBuf {
        &self.path
    }
    
    fn status(&self) -> &AssetStatus {
        &self.status
    }
    
    fn set_status(&mut self, status: AssetStatus) {
        self.status = status;
    }
    
    fn size(&self) -> Option<u64> {
        match self.path.metadata() {
            Ok(metadata) => Some(metadata.len()),
            Err(_) => None
        }
    }
    
    fn size_mb(&self) -> Option<u64> {
        match self.size() {
            Some(size) => Some(size / 1024 / 1024),
            None => None
        }
    }
    
    fn is_greater_than(&self, video: &VideoFile) -> bool {
        self.size() >= video.size()
    }
}

impl VideoFile {
    pub fn new(path: PathBuf) -> VideoFile {
        VideoFile { path, status: AssetStatus::Pending }
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
