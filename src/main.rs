extern crate clap;

mod list_projects;

use clap::{crate_version, App, Arg};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use list_projects::*;

fn main() {
    let stdout = io::stdout();
    let mut stdout_handle = stdout.lock();
    let cwd = env::current_dir().expect("Could not determine the current directory");

    let matches = App::new("proj")
        .version(crate_version!())
        .author("Alex LaFroscia <alex@lafroscia.com>")
        .about("List projects under a given directory")
        .arg(
            Arg::with_name("absolute")
                .short("a")
                .long("absolute")
                .help("Return absolute paths instead of relative ones"),
        )
        .get_matches();

    for entry in list_projects(&cwd) {
        let mut path = entry.path();

        if !matches.is_present("absolute") {
            path = PathBuf::from(path.strip_prefix(&cwd).expect("Could not strip prefix"));
        }

        stdout_handle
            .write_fmt(format_args!("{}\n", path.display()))
            .expect("Could not write to STDOUT")
    }
}
