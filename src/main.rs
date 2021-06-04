mod list_projects;

use std::env;
use std::io::{self, Write};

use list_projects::*;

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
