use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum ClinvarXMLTabError {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error(transparent)]
    QuickXMLError(#[from] quick_xml::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    CSVError(#[from] csv::Error),

    #[error(transparent)]
    SerdeError(#[from] quick_xml::DeError),

    #[error("No record")]
    NoRecord,

    #[error("Unknown error")]
    Unknown,
}
