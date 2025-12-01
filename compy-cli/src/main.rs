use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// A simple CLI to batch compress videos using FFmpeg
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input file or directory to process
    #[arg(required = true)]
    input: PathBuf,

    /// Delete the original file after successful conversion
    #[arg(short, long)]
    delete: bool,

    /// List files that will be processed without running the conversion
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Validate Input
    if !args.input.exists() {
        eprintln!("{} Error: Input path does not exist.", "X".red());
        std::process::exit(1);
    }

    // 2. Collect Files
    let video_files = get_video_files(&args.input)?;

    if video_files.is_empty() {
        println!("{}", "No video files found.".yellow());
        return Ok(());
    }

    // 3. Handle 'List' Flag
    if args.list {
        println!("{}", "The following files will be processed:".blue().bold());
        for file in &video_files {
            println!(" - {}", file.display());
        }
        if args.delete {
            println!("\n{}", "WARNING: Original files will be DELETED after processing.".red().bold());
        }
        return Ok(());
    }

    // 4. Process Files
    println!("Found {} video(s). Starting processing...", video_files.len());

    for (index, file) in video_files.iter().enumerate() {
        println!(
            "\n[{}/{}] Processing: {}",
            index + 1,
            video_files.len(),
            file.file_name().unwrap_or_default().to_string_lossy().cyan()
        );

        match process_video(file, args.delete) {
            Ok(_) => println!("{}", "Success!".green()),
            Err(e) => eprintln!("{} Failed: {}", "X".red(), e),
        }
    }

    Ok(())
}

/// Recursively finds video files in a directory or returns the file if it's a single video.
fn get_video_files(path: &Path) -> Result<Vec<PathBuf>> {
    let supported_extensions = ["mp4", "mkv", "avi", "mov", "flv", "wmv"];
    let mut files = Vec::new();

    if path.is_file() {
        // If it's a file, check extension
        if let Some(ext) = path.extension() {
            if supported_extensions.contains(&ext.to_str().unwrap_or("").to_lowercase().as_str()) {
                files.push(path.to_path_buf());
            }
        }
    } else if path.is_dir() {
        // If it's a directory, walk it
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if supported_extensions.contains(&ext.to_str().unwrap_or("").to_lowercase().as_str()) {
                        files.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    Ok(files)
}

/// Wraps the FFmpeg system command
fn process_video(input: &Path, delete_original: bool) -> Result<()> {
    // Generate output filename: "video.mp4" -> "video compressed.mp4"
    let file_stem = input.file_stem().context("No file name")?.to_str().context("Invalid UTF-8")?;
    let extension = input.extension().context("No extension")?.to_str().context("Invalid UTF-8")?;
    
    let parent_dir = input.parent().unwrap_or_else(|| Path::new("."));
    let output_filename = format!("{} compressed.{}", file_stem, extension);
    let output_path = parent_dir.join(output_filename);

    // Check if output already exists to avoid FFmpeg interactive overwrite prompt blocking the CLI
    if output_path.exists() {
        println!("{}", "Output file already exists, skipping...".yellow());
        return Ok(());
    }

    // Build the Command
    // We use -crf 23 for a balance of quality and size (default for x264)
    // We use -c:v libx264 for video and -c:a copy to preserve audio quality/codec
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        // .args(["-v", "warning"])
        .args(["-hide_banner"])
        .args(["stats"])
        .args(["-c:v", "libx264", "-crf", "23"]) 
        .args(["-c:a", "aac"])
        .args(["-b:a", "128k"])
        .args(["-map_metadata", "0"])
        .arg(&output_path)
        .stderr(std::process::Stdio::inherit()) // Show FFmpeg output in terminal
        .stdout(std::process::Stdio::null())    // Hide standard stdout (ffmpeg usually prints stats to stderr)
        .status()
        .context("Failed to execute ffmpeg command. Is ffmpeg installed?")?;

    // Error Handling: Check the exit code
    if !status.success() {
        // Clean up the partial output file if it failed
        if output_path.exists() {
            let _ = fs::remove_file(output_path);
        }
        anyhow::bail!("FFmpeg exited with error status");
    }

    // Delete original logic
    if delete_original {
        println!("Deleting original file...");
        fs::remove_file(input).context("Failed to delete original file")?;
    }

    Ok(())
}