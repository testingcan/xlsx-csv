extern crate calamine;
extern crate clap;
extern crate config;
extern crate csv;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use calamine::{open_workbook, DataType, Reader, Xlsx};
use clap::{App, Arg};
use std::error::Error;
use std::process;
use std::path::Path;
use std::{ffi::OsStr, fs, io};

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
        list_paths(&s.source.path).unwrap()
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
        match move_file(&file, &s.archive.path, &s.source.path) {
            Ok(()) => {}
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        }
    }
}

fn move_file(file: &str, archive: &str, source: &str) -> Result<(), Box<Error>> {
    fs::rename(file, str::replace(file, source, archive))?;
    Ok(())
}

fn list_paths(root: &str) -> io::Result<Vec<String>> {
    let mut result = vec![];

    for path in fs::read_dir(root)? {
        let path = path?.path();
        if let Some("xlsx") = path.extension().and_then(OsStr::to_str) {
            result.push(path.to_str().unwrap().to_owned());
        }
    }
    Ok(result)
}

fn convert(file: &str) -> Result<(), Box<Error>> {
    let mut workbook: Xlsx<_> = open_workbook(&file).expect("Cannot open file!");

    let sheet = workbook
        .sheet_names()
        .first()
        .expect("Could not find any Sheets")
        .to_owned();
    if let Some(Ok(range)) = workbook.worksheet_range(&sheet) {
        let mut wtr = csv::WriterBuilder::new()
            .from_path(format!("{}.csv", str::replace(file, ".xlsx", "")))
            .unwrap();
        let rows = range.rows();

        for (_i, row) in rows.enumerate() {
            let cols: Vec<String> = row.iter()
                .map(|c| match c {
                    DataType::Int(c) => format!("{}", c),
                    DataType::Float(c) => format!("{}", c),
                    DataType::String(c) => format!("{}", c),
                    DataType::Bool(c) => format!("{}", c),
                    _ => "".to_string(),
                })
                .collect();
            wtr.write_record(&cols).unwrap();
        }
        wtr.flush().unwrap();
    }
    Ok(())
}
