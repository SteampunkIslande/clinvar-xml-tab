use std::mem::take;

use crate::error::ClinvarXMLTabError;
use crate::xml::handler::EventHandler;
use serde::Serialize;
use serde::Serializer;

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ClinVarRecord {
    status: Option<String>,
    replaces: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rcv: Option<String>,
    vcv: Option<String>,
    clnsig: Option<String>,
    #[serde(rename = "DESCRIPTION")]
    description: Option<String>,
    date_last_updated: Option<String>,
}

/// Just a convenient function that can be used to deserialize a vec of strings into any serializer that doesn't support nested serialization
#[allow(dead_code)]
fn vec_as_string_pipe<S>(v: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&v.join("|"))
}

pub struct RecordHandler<W: std::io::Write> {
    record: ClinVarRecord,
    serializer: csv::Writer<W>,
}

impl<W: std::io::Write> EventHandler for RecordHandler<W> {
    fn handle(
        &mut self,
        node: &roxmltree::Node,
        current_path: &Vec<String>,
        _attributes: &std::collections::HashMap<String, String>,
        depth: u32,
    ) -> Result<(), ClinvarXMLTabError> {
        if depth == 2 {
            if node.has_tag_name("RecordStatus") {
                self.record.status = Some(node.text().unwrap_or("").to_string());
            }
            if node.has_tag_name("Replaces") {
                self.record.replaces = Some(node.text().unwrap_or("").to_string());
            }
            if node.has_tag_name("Title") {
                self.record.description = Some(node.text().unwrap_or("").to_string());
            }
            if node.has_tag_name("ReferenceClinVarAssertion") {
                self.record.date_last_updated =
                    Some(node.attribute("DateLastUpdated").unwrap_or("").to_string());
            }
        }
        if depth == 3 {
            if node.has_tag_name("ClinVarAccession") {
                if let Some(acc_type) = node.attribute("Type") {
                    if acc_type == "RCV" {
                        self.record.rcv = Some(node.attribute("Acc").unwrap_or("").to_string());
                    }
                }
            }
            if current_path == &["ClinVarSet", "ReferenceClinVarAssertion", "MeasureSet"] {
                self.record.vcv = Some(node.attribute("Acc").unwrap_or("").to_string());
            }
        }
        if depth == 5 {
            if current_path
                == &[
                    "ClinVarSet",
                    "ReferenceClinVarAssertion",
                    "Classifications",
                    "GermlineClassification",
                    "Description",
                ]
            {
                self.record.clnsig = Some(
                    node.text()
                        .iter()
                        .map(|s| s.replace(" ", "_").to_lowercase())
                        .collect(),
                );
            }
        }

        Ok(())
    }

    fn end_record(&mut self) -> Result<(), ClinvarXMLTabError> {
        let record = take(&mut self.record);
        self.serializer.serialize(record)?;
        Ok(())
    }
}

impl<W: std::io::Write> RecordHandler<W> {
    pub fn new_from_writer(writer: W) -> Self {
        Self {
            serializer: csv::Writer::from_writer(writer),
            record: ClinVarRecord::default(),
        }
    }
}
