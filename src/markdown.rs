use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn read(filename: String) -> Result<String, String> {
    if !Path::new(&filename).exists() {
        return Err(format!("File {filename} not found"));
    }

    let extension = match Path::new(&filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
    {
        Some(ext) => ext,
        None => return Err(format!("Uknown extension on file {filename}")),
    };

    if extension != "md" {
        return Err(format!("file {filename} not a markdown file"));
    }

    let data = match std::fs::read_to_string(filename.clone()) {
        Ok(data) => data,
        Err(_) => return Err(format!("Uknown error with reading file {filename}")),
    };

    Ok(data)
}

pub fn write(filename: String, data: String) -> Result<(), String> {
    let data = data.as_bytes();

    let mut file = match File::create(filename.clone()) {
        Ok(data) => data,
        Err(_) => return Err(format!("Trouble writing to file {filename}")),
    };

    if file.write_all(data).is_err() {
        return Err(format!("Trouble writing to file {filename}"));
    }

    Ok(())
}
