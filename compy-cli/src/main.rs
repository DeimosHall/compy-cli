use std::{error::Error, process};
use std::path::{PathBuf};
use clap::{Parser};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to a single video or a directory
    input: Option<PathBuf>,
    
    /// Delete original file only after successful compression
    #[arg(short, long)]
    delete: bool,
    
    /// List the files to be processed and exits without compressing
    #[arg(short, long)]
    list: bool
}

fn delete_original() {
    println!("Deleting original file...");
}

fn list_files() {
    println!("Listing compatible files...");
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    
    let input = match cli.input {
        Some(input) => input,
        None => {
            eprintln!("No input provided");
            process::exit(1);
        }
    };
    println!("Input: {:?}", input);
    
    let mut files: Vec<PathBuf> = vec![];
    
    if input.exists() && input.is_file() {
        files.push(input);
    } else if input.exists() && input.is_dir() {
        let directory: WalkDir = WalkDir::new(input.as_os_str());
        for file in directory {
            files.push(file?.into_path());
        }
    }
    
    println!("Files list:");
    for file in files {
        println!("File: {:?}", file);
    }
    
    cli.delete.then(|| { delete_original() });
    cli.list.then(|| { list_files() });
    
    Ok(())
}