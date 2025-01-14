use crate::error::ClinvarXMLTabError;
use std::collections::HashMap;

pub trait EventHandler {
    fn handle(
        &mut self,
        node: &roxmltree::Node,
        current_path: &Vec<String>,
        attributes: &HashMap<String, String>,
        depth: u32,
    ) -> Result<(), ClinvarXMLTabError>;

    fn end_record(&mut self) -> Result<(), ClinvarXMLTabError>;
}

pub struct BasicNodeWriter<T: std::io::Write> {
    writer: T,
}

impl<W: std::io::Write> BasicNodeWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

/// This BasicNodeWriter implementation simply writes every node with their complete path, along with their attributes if any.
/// This `EventHandler` can be instantiated from anything that implements `std::io::Write`.
impl<T: std::io::Write> EventHandler for BasicNodeWriter<T> {
    fn handle(
        &mut self,
        node: &roxmltree::Node,
        current_path: &Vec<String>,
        attributes: &HashMap<String, String>,
        depth: u32,
    ) -> Result<(), ClinvarXMLTabError> {
        self.writer.write(
            format!(
                "{}{} - {} - {}\n",
                "\t".repeat((depth as usize).saturating_sub(1)),
                current_path.join("."),
                node.text().unwrap_or("No text").trim(),
                if attributes.len() == 0 {
                    "No attributes".to_string()
                } else {
                    attributes
                        .iter()
                        .map(|f| format!("{}={}", f.0, f.1))
                        .collect::<Vec<String>>()
                        .join(" ")
                }
            )
            .as_bytes(),
        )?;
        Ok(())
    }
    fn end_record(&mut self) -> Result<(), ClinvarXMLTabError> {
        Ok(())
    }
}
