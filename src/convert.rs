extern crate calamine;
extern crate csv;
use calamine::{open_workbook, DataType, Reader, Xlsx};
use std::error::Error;

pub fn convert(file: &str, sheet: &Option<Vec<String>>) -> Result<(), Box<Error>> {
    let mut workbook: Xlsx<_> = open_workbook(&file).expect("Cannot open file!");

    let wb_sheets = vec![workbook.sheet_names()
                            .first()
                            .expect("c")
                            .to_owned()];

    let sheets: &Vec<String> = match sheet {
        Some(t) => t,
        None => &wb_sheets
    };

    println!("{:?}", sheets);

    for sheet in sheets {
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet) {
            let mut wtr = csv::WriterBuilder::new()
                .from_path(format!("{}-{}.csv", str::replace(file, ".xlsx", "")
                    , str::replace(sheet, " ", "_")))
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
