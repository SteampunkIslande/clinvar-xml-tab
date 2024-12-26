use thiserror::Error;

use std::io;

#[derive(Error, Debug)]
pub enum ClinvarXMLTabError {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error(transparent)]
    QuickXMLError(#[from] quick_xml::Error),

    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    CSVError(#[from] csv::Error),

    // Quickxml is not used for serde anymore. Roxml is much easier to work with
    // #[error(transparent)]
    // SerdeError(#[from] quick_xml::DeError),
    #[error(transparent)]
    RoxmlError(#[from] roxmltree::Error),
}
