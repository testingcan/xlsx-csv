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
