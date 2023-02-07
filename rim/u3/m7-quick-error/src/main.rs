#[macro_use]
extern crate quick_error;

// use std::error::Error;
// use std::fmt;
use quick_error::ResultExt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::result;

const MAX_DOCS_CREATED_PER_MINUTES: u8 = 100;

pub type Result<T> = result::Result<T, DocumentServiceError>;

quick_error! {
    #[derive(Debug)]
    pub enum DocumentServiceError {
        RateLimitExceeded {
            display("You have exceeded the allowed number of document per minute")
        }
        Io(filename: String, cause: io::Error) {
            display("I/O error: {} for filename {}", cause, filename)
            context(filename: &'a str, cause: io::Error)
                -> (filename.to_string(), cause)
        }
    }
}

// impl Error for DocumentServiceError {
//     fn description(&self) -> &str {
//         use DocumentServiceError::*;
//         match *self {
//             RateLimitExceeded => "rate limit exceeded",
//             Io(_) => "I/O Error",
//         }
//     }
// }

// impl fmt::Display for DocumentServiceError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         use DocumentServiceError::*;
//         match *self {
//             RateLimitExceeded => write!(
//                 f,
//                 "You have exceeded the allowed number of documents per minute."
//             ),
//             Io(ref io) => write!(f, "I/O error: {}", io),
//         }
//     }
// }

// impl From<io::Error> for DocumentServiceError {
//     fn from(other: io::Error) -> Self {
//         DocumentServiceError::Io(other)
//     }
// }

fn num_documents_created_in_last_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTES {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .context(filename)?;

    Ok(file)
}

fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft1", project_name))?;
    create_document(&format!("{}-draft2", project_name))?;
    create_document(&format!("{}-revision1", project_name))?;
    create_document(&format!("{}-revision2", project_name))?;
    Ok(())
}

fn main() {
    match create_project("my-project") {
        Ok(()) => println!("Project created succesfully"),
        Err(e) => println!("Project creation failed: {}", e),
    }
}
