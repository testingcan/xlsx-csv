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
        .version("0.2.2")
        .author("Raphael Wuillemier <raphael.wuillemier@protonmail.com>")
        .about("Converts XLSX-files to CSV")
        .long_about(
            "Use either with config-files or specify 
            an input (and output) file or dir to convert.",
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("Sets the input file or directory [default: .]")
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
        .arg(
            Arg::with_name("sheet")
                .short("s")
                .help("Specifiy the sheet(s) to convert")
                .takes_value(true)
                .multiple(true)
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .help("Set the delimiter for the CSV-output [default: b',']")
                .takes_value(true)
        )
        .get_matches();

    let mut settings = Settings::new().unwrap();

    if matches.is_present("input") {
        settings.source.path = matches.value_of("input").unwrap().to_string()
    }
    if matches.is_present("output") {
        settings.archive.path = matches.value_of("output").unwrap().to_string()
    }
    if matches.is_present("delimiter") {
        settings.delimiter = matches.value_of("delimiter").unwrap().to_string()
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

    let sheets: Option<Vec<_>> = if matches.is_present("sheet") {
        Some(matches.values_of("sheet")
                .map(|sheet| sheet
                    .map(|s| s.to_string())
                    .collect())
                .unwrap())
    } else {
        None
    };

    for file in vec {
        if settings.debug {
            println!("Converting file: {:?}", file)
        };
        match convert(&file, &sheets, &settings) {
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
