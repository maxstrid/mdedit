use std::io::Read;
use std::fs::File;
use std::path::Path;

pub fn read(filename: String) -> Result<Vec<String>, String> {
    if !Path::new(&filename).exists() {
        return Err(format!("File {filename} not found"));
    }

    let extension = match Path::new(&filename).extension().and_then(std::ffi::OsStr::to_str) {
        Some(ext) => ext,
        None => return Err(format!("Uknown extension on file {filename}"))
    };

    if extension != "md" {
        return Err(format!("file {filename} not a markdown file"));
    }


    let mut file = match File::open(filename.clone()) {
        Ok(file) => file,
        Err(_) => return Err(format!("uknown error with opening file {filename}"))
    };

    let mut data = String::new();

    if file.read_to_string(&mut data).is_err() {
        return Err(format!("Uknown error with reading file {filename}"));
    }

    Ok(data.lines().map(|s| s.to_string()).collect())
}