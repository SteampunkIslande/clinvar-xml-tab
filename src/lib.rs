//! # XML Clinvar to TSV
//! This is a conversion utility to...
pub mod error;
pub mod utils;
mod xml;
pub use xml::handler;
pub use xml::reader;

pub mod clinvar;
