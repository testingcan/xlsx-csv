extern crate calamine;
extern crate clap;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};
use std::process;
use std::path::Path;

mod file;
mod convert;
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
            an input (and output) file to convert.")
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
                .takes_value(true)
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Show verbose debug output"),
        )
        
        .get_matches();

    let s = if !matches.is_present("input") {
        Settings::new().expect(
            "Could not read s. Either supply a config-file 
                or specify an input file with '-i'",
        )
    } else {
        let mut s = config::Config::new();
        s.set(
            "source.path",
            matches.value_of("input").unwrap().to_string(),
        ).unwrap();
        s.set(
            "archive.path",
            matches.value_of("output").unwrap_or(matches.value_of("input").unwrap()).to_string(),
        ).unwrap();
        if matches.is_present("verbose") {
            s.set("debug", true).unwrap();
        } else {
            s.set("debug", false).unwrap();
        };
        s.try_into().unwrap()
    };

    if s.debug {
        println!("{:?}", s);
    };

    let vec = if Path::new(&s.source.path).is_dir() {
        file::list_paths(&s.source.path).unwrap()
    } else {
        vec![(&s.source.path).to_owned()]
    };


    for file in vec {
        if s.debug {
            println!("Converting file: {:?}", file)
        };
        match convert(&file) {
            Ok(()) => {}
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        }

        if s.debug {
            println!("Moving file '{:?}' to archive", file)
        };
        match file::move_file(&file, &s.archive.path, &s.source.path) {
            Ok(()) => {}
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        }
    }
}