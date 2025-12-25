use std::process::Command;

pub fn is_ffmpeg_installed() -> bool {
    let status = Command::new("ffmpeg").arg("-version").status();
    match status {
        Ok(status) => status.success(),
        Err(_) => false
    }
}