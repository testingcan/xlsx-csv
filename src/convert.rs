extern crate calamine;
extern crate csv;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::error::Error;


pub fn convert(file: &str) -> Result<(), Box<Error>> {
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