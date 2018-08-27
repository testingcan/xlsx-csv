extern crate calamine;
extern crate csv;

use self::calamine::{open_workbook, DataType, Reader, Xlsx};
use settings;
use std::error::Error;
use std::{ffi::OsStr, fs, io};

pub fn move_file(file: &str, archive: &str, source: &str) -> Result<(), Box<Error>> {
    fs::create_dir_all(archive)?;
    fs::rename(file, str::replace(file, source, archive))?;
    Ok(())
}

pub fn list_paths(root: &str) -> io::Result<Vec<String>> {
    let mut result = vec![];

    for path in fs::read_dir(root)? {
        let path = path?.path();
        if let Some("xlsx") = path.extension().and_then(OsStr::to_str) {
            result.push(path.to_str().unwrap().to_owned());
        }
    }
    Ok(result)
}

pub fn convert(
    file: &str,
    sheet: &Option<Vec<String>>,
    settings: &settings::Settings,
) -> Result<(), Box<Error>> {
    let mut workbook: Xlsx<_> = open_workbook(&file).expect("Cannot open file!");

    let wb_sheets = vec![workbook.sheet_names().first().expect("c").to_owned()];

    let sheets: &Vec<String> = match sheet {
        Some(t) => t,
        None => &wb_sheets,
    };

    println!("{:?}", sheets);

    for sheet in sheets {
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet) {
            let mut wtr = csv::WriterBuilder::new()
                .delimiter(settings.delimiter.as_bytes().first().unwrap().to_owned())
                .from_path(format!(
                    "{}-{}.csv",
                    str::replace(file, ".xlsx", ""),
                    str::replace(sheet, " ", "_")
                ))
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
        } else {
            println!("Could not find sheet {:?} in file {}", &sheet, &file);
        }
    }

    Ok(())
}
