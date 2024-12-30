use std::io::{self};
use std::{env, path::PathBuf};
use std::fs::create_dir_all;

mod file_organizer;



use crate::file_organizer::get_list_of_files;


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let path = PathBuf::from(env::args().nth(1).unwrap());

    if !path.is_dir() {
        return Err("The path provided is not a directory".into());
    }

    let mut list_files = get_list_of_files(&path)?;

    if list_files.is_empty() {
        return Err("No files found".into());
    }

    list_files.sort_by_key(|file| file.path.clone());

    let max_preview_files = 10;

    // Calculate the slice end index
    let num_preview_files = std::cmp::min(list_files.len(), max_preview_files);

    println!("First {} files:",num_preview_files);

    for file in &list_files[..num_preview_files] {
        println!("{:?} created at {}", file.path, file.created);
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