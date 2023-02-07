extern crate failure;

#[macro_use]
extern crate failure_derive;

use failure::{Backtrace, Fail};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::result;

const MAX_DOCS_CREATED_PER_MINUTES: u8 = 100;

pub type Result<T> = result::Result<T, DocumentServiceError>;

#[derive(Debug, Fail)]
pub enum DocumentServiceError {
    #[fail(display = "You have exceeded the allowed number of document per minute.")]
    RateLimitExceeded(Backtrace),
    #[fail(display = "I/O error: {}", _0)]
    Io(io::Error, Backtrace),
}

impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other, Backtrace::new())
    }
}

fn num_documents_created_in_last_minute() -> u8 {
    2
}

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTES {
        return Err(DocumentServiceError::RateLimitExceeded(Backtrace::new()));
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

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
        Err(e) => {
            println!("Project creation failed: {}", e);
            if let Some(backtrace) = e.backtrace() {
                if !backtrace.to_string().trim().is_empty() {
                    println!("backtrace: {:?}", backtrace);
                }
            }
        }
    }
}
