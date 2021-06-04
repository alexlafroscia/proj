use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

fn is_repo(entry: &DirEntry) -> bool {
    let name = entry.path();
    let mut git_dir = name.clone();
    git_dir.push(Path::new(".git"));

    git_dir.exists()
}

pub fn list_projects(path: &PathBuf) -> Vec<DirEntry> {
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
                false => list_projects(&entry.path()),
            }
        })
        .collect::<Vec<DirEntry>>()
}

#[cfg(test)]
mod tests {
    use super::list_projects;

    use assert_fs::fixture::TempDir;
    use std::ffi::OsString;
    use std::fs::{create_dir_all, File};
    use std::io::prelude::*;
    use std::path::PathBuf;

    #[test]
    fn non_project_directories() {
        let tmp_dir = TempDir::new().unwrap();

        // Create a sub-directory that is a project
        let mut a = PathBuf::from(tmp_dir.path());
        a.push("a");
        a.push(".git");

        create_dir_all(a.as_path()).expect("Could not create project directory for testing");

        // Create a sub-directory that is not a project
        let mut b = PathBuf::from(tmp_dir.path());
        b.push("b");

        create_dir_all(b.as_path()).expect("Could not create project directory for testing");

        let result = list_projects(&PathBuf::from(tmp_dir.path()))
            .into_iter()
            .map(|entry| entry.file_name())
            .collect::<Vec<OsString>>();

        assert_eq!(result, vec![OsString::from("a")]);
    }

    #[test]
    fn filtering_files() {
        let tmp_dir = TempDir::new().unwrap();

        // Create a sub-directory that is a project
        let mut a = PathBuf::from(tmp_dir.path());
        a.push("a");
        a.push(".git");

        create_dir_all(a.as_path()).expect("Could not create project directory for testing");

        // Create a sub-directory that is not a project
        let mut b_path = PathBuf::from(tmp_dir.path());
        b_path.push("b");
        b_path.set_extension("txt");

        let mut b_file = File::create(b_path).expect("Could not create file");
        b_file
            .write_all(b"Testing")
            .expect("Could not write to file");

        let result = list_projects(&PathBuf::from(tmp_dir.path()))
            .into_iter()
            .map(|entry| entry.file_name())
            .collect::<Vec<OsString>>();

        assert_eq!(result, vec![OsString::from("a")]);
    }

    #[test]
    fn nested_directories() {
        let tmp_dir = TempDir::new().unwrap();

        // Create a sub-directory that is a project
        let mut a = PathBuf::from(tmp_dir.path());
        a.push("a");
        a.push("b");
        a.push(".git");

        create_dir_all(a.as_path()).expect("Could not create project directory for testing");

        let result = list_projects(&PathBuf::from(tmp_dir.path()))
            .into_iter()
            .map(|entry| entry.file_name())
            .collect::<Vec<OsString>>();

        assert_eq!(result, vec![OsString::from("b")]);
    }
}
