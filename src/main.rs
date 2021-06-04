use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn is_repo(entry: &DirEntry) -> bool {
    let name = entry.path();
    let mut git_dir = name.clone();
    git_dir.push(Path::new(".git"));

    git_dir.exists()
}

fn list_projects(path: PathBuf) -> Vec<DirEntry> {
    fs::read_dir(path)
        .expect("Could not read directory")
        .flat_map(|wrapped_entry| {
            let entry = wrapped_entry.expect("Could not read entry");

            if !entry
                .file_type()
                .expect("Could not determine file type")
                .is_dir()
            {
                return vec![];
            }

            match is_repo(&entry) {
                // if this is a repo, add it to our result
                true => vec![entry],
                // otherwise, descend into this directory
                false => list_projects(entry.path()),
            }
        })
        .collect::<Vec<DirEntry>>()
}

fn main() {
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    let cwd = env::current_dir().expect("Could not determine the current directory");

    for entry in list_projects(cwd) {
        stdout_handle
            .write_fmt(format_args!("{}\n", entry.path().display()))
            .expect("Could not write to STDOUT")
    }
}
