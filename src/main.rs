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

    let matches = App::new("proj")
        .version(crate_version!())
        .author("Alex LaFroscia <alex@lafroscia.com>")
        .about("List projects under a given directory")
        .arg(
            Arg::with_name("WITHIN")
                .help("The directory to locate projects within [default: $PWD]"),
        )
        .arg(
            Arg::with_name("absolute")
                .short("a")
                .long("absolute")
                .help("Return absolute paths instead of relative ones"),
        )
        .get_matches();

    let within = match matches.is_present("WITHIN") {
        true => PathBuf::from(
            matches
                .value_of("WITHIN")
                .expect("Could not read `within` argument"),
        ),
        false => env::current_dir().expect("Could not determine the current directory"),
    };

    for entry in list_projects(&within) {
        let mut path = entry.path();

        if !matches.is_present("absolute") {
            path = PathBuf::from(path.strip_prefix(&within).expect("Could not strip prefix"));
        }

        stdout_handle
            .write_fmt(format_args!("{}\n", path.display()))
            .expect("Could not write to STDOUT")
    }
}
