use std::fmt::Display;
use std::io::Read;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub enum MDError {
    FileNotFound,
    IoError,
    UknownExtension,
    FileNotMarkdown
}

// Not needed because the errors will have custom gui messages 
// but its needed for it to be an error type 
impl Display for MDError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MDError::FileNotFound => write!(f, "File not found"),
            MDError::IoError => write!(f, "Uknown error with IO on file"),
            MDError::UknownExtension => write!(f, "Uknown extension"),
            MDError::FileNotMarkdown => write!(f, "File not markdown")
        }
    }
}

impl std::error::Error for MDError {}

pub fn read(filename: String) -> Result<Vec<String>, MDError> {
    if !Path::new(&filename).exists() {
        return Err(MDError::FileNotFound);
    }

    let extension = match Path::new(&filename).extension().and_then(std::ffi::OsStr::to_str) {
        Some(ext) => ext,
        None => return Err(MDError::UknownExtension)
    };

    if extension != "md" {
        return Err(MDError::FileNotMarkdown);
    }


    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err(MDError::IoError)
    };

    let mut data = String::new();

    if file.read_to_string(&mut data).is_err() {
        return Err(MDError::IoError);
    }

    Ok(data.lines().map(|s| s.to_string()).collect())
}