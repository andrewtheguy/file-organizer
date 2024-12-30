use std::collections::HashMap;
use std::{fs, path::PathBuf};

use walkdir::WalkDir;
use chrono::{DateTime, Local, Utc};
use std::hash::Hasher;
use std::io::{self, BufReader, Read, Write};
use std::fs::{create_dir_all, File};
use twox_hash::XxHash3_64;

pub struct FileEntry {
    pub path: std::path::PathBuf,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub file_size: u64,
}

pub fn get_list_of_files(orig_path: &PathBuf)-> Result<Vec<FileEntry>, Box<dyn std::error::Error>> {

    if !orig_path.is_dir() {
        return Err("The path provided is not a directory".into());
    }

    let mut file_list: Vec<FileEntry> = Vec::new();

    for entry in WalkDir::new(orig_path)
        .max_depth(1) // no subdirectories
        .follow_links(false) {
        let entry = entry?;
        //let path = entry.path();
        if entry.path().is_symlink() {
            // maybe skip instead in the future
            return Err(format!("Symlinks are not supported for {}",entry.path().display()).into());
        }
        //println!("entry: {:?}", entry.path());
        if let Ok(relative_path) = entry.path().strip_prefix(orig_path) {
            if let Some(first_component) = relative_path.components().next() {
                if first_component.as_os_str() == "organized" {
                    println!("Entry {:?} has 'organized' as the first subfolder, skipping", entry.path());
                    continue;
                }
            } 
            if relative_path.components().any(|c| c.as_os_str() == ".DS_Store") {
                println!("Entry {:?} has '.DS_Store' in the path, skipping", entry.path());
                continue;
            }
        }else{
            return Err("Error stripping prefix".into());
        }
        if entry.path().is_file() {
            //println!("{}", entry.path().display());
            
            let metadata = fs::metadata(entry.path())?;

            if let Ok(time) = metadata.created() {
                //println!("{time:?}");
                let datetime: DateTime<Utc> = time.into();
                //println!("{}",datetime.to_rfc3339());
                file_list.push(FileEntry {
                    path: entry.path().to_path_buf(),
                    created: datetime,
                    modified: metadata.modified()?.into(),
                    file_size: metadata.len(),
                });
            } else {
                return Err("creation date not supported on this platform or filesystem".into());
            }
        }
    }
    
    Ok(file_list)
}


fn hash_file_first_16k(path: &PathBuf) -> io::Result<u64> {
    // Open the file
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    // Create a new XXHash64 hasher
    let mut hasher = XxHash3_64::default();

    // Buffer for reading the file in chunks
    let mut buffer = [0; 16384]; // 16kb

    // Read the file in chunks and update the hasher
    
    let bytes_read = reader.read(&mut buffer)?;
    eprintln!("bytes_read: {}", bytes_read);
    if bytes_read != 0 {
        hasher.write(&buffer[..bytes_read]);
    }

    // Return the hash value
    Ok(hasher.finish())
}

pub fn backup_orig_file_location(orig_path: &PathBuf,list_files: &[FileEntry]) -> Result<(), Box<dyn std::error::Error>> {
    let mut orig_files = HashMap::new();


    for file in list_files {
        if file.path.is_symlink() {
            return Err(format!("Symlinks are not supported for {}", file.path.display()).into());
        } else if file.path.is_dir() {
            return Err(format!("Directories are not supported for {}", file.path.display()).into());
        }
        // simple filesize and first 16k hash check together with modified date, which collision
        // is not likely to happen for small input size, and if it does
        // will be handled by the duplicate check
        // TODO: skip duplication check for zero size files, but for my use case it is mostly for
        // media files, which are not zero size
        println!("Checking {:?}", file.path);
        let key = format!("{}:{}:{}",
                            hash_file_first_16k(&file.path)?,
                            file.file_size.clone(),
                            file.modified.timestamp());
        if orig_files.contains_key(&key) {
            return Err(format!("Found potential duplicate file: {:?}", file.path).into());
        } else {
            println!("key: {}", key);
            orig_files.insert(key, file.path.clone());
        }
        let backup_dir = orig_path.join("organized");
        create_dir_all(&backup_dir)?;

        let utc: DateTime<Utc> = Utc::now();
        let backup_file = backup_dir.join(format!("backup{}.json",
            utc.format("%Y%m%d%H%M%S%.3f").to_string()));
        let mut backup_file_json = File::create(&backup_file)?;
        backup_file_json.write_all(serde_json::to_string_pretty(&orig_files)?.as_bytes())?;    
    }
    Ok(())
}

pub fn get_new_pair_path(orig_path: &PathBuf, file: &FileEntry) -> Result<(PathBuf,PathBuf), Box<dyn std::error::Error>> {
    let datetime = file.created;
    let local_datetime: DateTime<Local> = DateTime::from(datetime);
    let new_dir = orig_path.join("organized")
        .join(local_datetime.format("%Y").to_string())
        .join(local_datetime.format("%m").to_string())
        .join(local_datetime.format("%d").to_string());
    let new_file = new_dir.join(file.path.file_name().unwrap());
    Ok((new_dir,new_file))
}