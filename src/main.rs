extern crate calamine;
extern crate clap;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};
use std::path::Path;
use std::process;

mod convert;
mod file;
use convert::convert;
mod settings;
use settings::Settings;

fn main() {
    let matches = App::new("XLSX-CSV")
        .version("0.2.1")
        .author("Raphael W. <raphael.wuillemier@protonmail.com")
        .about("Converts XLSX-files to CSV")
        .long_about(
            "Use either with config-files or specify 
            an input (and output) file to convert.",
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("Sets the input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Output directory [default: .]")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Show verbose debug output"),
        )
        .get_matches();

    let mut settings = Settings::new().unwrap();

    if matches.is_present("input") {
        settings.source.path = matches.value_of("input").unwrap().to_string()
    }
    if matches.is_present("output") {
        settings.archive.path = matches.value_of("output").unwrap().to_string()
    }
    if matches.is_present("verbose") {
        settings.debug = true
    }
    if settings.debug {
        println!("{:?}", settings)
    };

    let vec = if Path::new(&settings.source.path).is_dir() {
        file::list_paths(&settings.source.path).unwrap()
    } else {
        vec![(&settings.source.path).to_owned()]
    };

    for file in vec {
        if settings.debug {
            println!("Converting file: {:?}", file)
        };
        match convert(&file) {
            Ok(()) => {}
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        }

        if settings.debug {
            println!("Moving file '{:?}' to archive", file)
        };
        match file::move_file(&file, &settings.archive.path, &settings.source.path) {
            Ok(()) => {}
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        }
    }
}
