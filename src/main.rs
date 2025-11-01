use std::io::{self};
use std::path::PathBuf;
use std::fs::create_dir_all;

mod file_organizer;

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};

use crate::file_organizer::get_list_of_files;

/// File organization utility
#[derive(Parser)]
#[command(name = "file-organizer")]
#[command(about = "A tool for organizing and managing files", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Organize files into subdirectories based on creation date
    OrganizeByCreationDate {
        /// Path to the directory to organize
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
    /// Separate potential live photo videos from images
    SeparateLivePhotoVideos {
        /// Path to the directory to process
        #[arg(value_name = "PATH")]
        path: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.action {
        Action::OrganizeByCreationDate { path } => organize_by_creation_date(path),
        Action::SeparateLivePhotoVideos { path: _ } => {
            Err("Separate live photo videos functionality is not yet implemented yet".into())
        }
    }
}

fn organize_by_creation_date(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {

    if !path.is_dir() {
        return Err(format!("The path {:?} provided is not a directory",path).into());
    }

    let mut list_files = get_list_of_files(&path)?;

    if list_files.is_empty() {
        return Err("No files found".into());
    }

    list_files.sort_by_key(|file| file.path.clone());

    let max_preview_files = 10;

    // Calculate the slice end index
    let num_preview_files = std::cmp::min(list_files.len(), max_preview_files);

    println!("Total number of files: {}", list_files.len());
    println!("First {} files:",num_preview_files);

    for file in &list_files[..num_preview_files] {
        let local_datetime: DateTime<Local> = DateTime::from(file.created);
        println!("{:?} created at {}", file.path, local_datetime);
    }

    if list_files.len() > num_preview_files {
        println!("...");
    }

    println!("enter y to continue, anything else to exit");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim() != "y" {
        return Ok(());
    }

    for file in &list_files {
        if file.path.is_symlink() {
            return Err(format!("Symlinks are not supported for {}", file.path.display()).into());
        } else if file.path.is_dir() {
            return Err(format!("Directories are not supported for {}", file.path.display()).into());
        }
        let (_, new_file) = file_organizer::get_new_pair_path(&path,&file)?;
        if new_file.exists() {
            return Err(format!("The file {:?} already exists in the destination", new_file).into());
        }
    }
    for file in &list_files {
        let (new_dir, new_file) = file_organizer::get_new_pair_path(&path,&file)?;
        create_dir_all(&new_dir)?;
        println!("Moving {:?} to {:?}", file.path, new_file);
        // need to be the same filesystem otherwise might alter timestamps
        std::fs::rename(&file.path, &new_file)?;
    }

    Ok(())
}