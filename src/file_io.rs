/* file_io.rs: File I/O */

use std::process;
use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use crate::fingerprint::Fingerprint;

// Sub represents a student submission.
// Depending on whether input submissions are directories or
// indiv. files, the dir_name field will be Some or None
pub struct Sub<'a> {
    dir_name: Option<&'a Path>,
    documents: Vec<Doc<'a>>
}

// Doc represents a file within a Sub.
// Docs are initialized before their fingerprints are computed,
// so None for fingerprints indicates they have not yet been computed.
pub struct Doc<'a> {
    file_name: &'a Path,
    fingerprints: Option<Vec<Fingerprint>>
}

// construct a vector of PathBufs to all files in a given directory 
// that pass the given predicate
fn paths_in_dir<F>(dir: &Path, keep: F) -> io::Result<Vec<PathBuf>> 
    where F: Fn(&PathBuf) -> bool {
    let mut paths = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if keep(&path) {
            paths.push(path);
        }
    }

    Ok(paths)
}

// get paths to all .arr files in a given directory
pub fn arr_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let is_arr = |p: &PathBuf| {
        match p.extension() {
            Some(ext) => ext == "arr",
            None => false,
        }
    };
    match paths_in_dir(dir, is_arr) {
        Ok(paths) => paths,
        Err(e) => {
            println!("Error: Failed to read .arr files in `{}`", dir.display());
            process::exit(1);
        },
    }
}