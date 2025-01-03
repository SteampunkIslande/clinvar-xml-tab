use std::mem::take;

use crate::error::ClinvarXMLTabError;
use crate::xml::handler::EventHandler;
use noodles_core::Position;
use noodles_vcf::variant::io::Write;
use serde::Serialize;
use serde::Serializer;

use noodles_vcf as vcf;

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

    chromosome: Option<String>,
    position: Option<usize>,
    reference: Option<String>,
    alternate: Option<String>,
}

/// Just a convenient function that can be used to deserialize a vec of strings into any serializer that doesn't support nested serialization
#[allow(dead_code)]
fn vec_as_string_pipe<S>(v: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&v.join("|"))
}

pub struct CSVRecordHandler<W: std::io::Write> {
    record: ClinVarRecord,
    serializer: csv::Writer<W>,
}

impl<W: std::io::Write> EventHandler for CSVRecordHandler<W> {
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

impl<W: std::io::Write> CSVRecordHandler<W> {
    pub fn new_from_writer(writer: W) -> Self {
        Self {
            serializer: csv::Writer::from_writer(writer),
            record: ClinVarRecord::default(),
        }
    }
}

pub struct VCFRecordHandler<W: std::io::Write> {
    record: ClinVarRecord,
    vcf_writer: vcf::io::Writer<W>,
    vcf_header: vcf::Header,
    assembly: &'static str,
}

impl<W: std::io::Write> EventHandler for VCFRecordHandler<W> {
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
            if current_path
                == &[
                    "ClinVarSet",
                    "ReferenceClinVarAssertion",
                    "MeasureSet",
                    "Measure",
                    "SequenceLocation",
                ]
            {
                if let Some(genome_assembly) = node.attribute("Assembly") {
                    if genome_assembly == self.assembly {
                        self.record.chromosome = node.attribute("Chr").map(|s| format!("chr{s}"));
                        self.record.position = node
                            .attribute("positionVCF")
                            .map(|s| s.parse().ok())
                            .flatten();
                        self.record.reference =
                            node.attribute("referenceAlleleVCF").map(|s| s.to_string());
                        self.record.alternate =
                            node.attribute("alternateAlleleVCF").map(|s| s.to_string());
                    }
                }
            }
        }

        Ok(())
    }

    fn end_record(&mut self) -> Result<(), ClinvarXMLTabError> {
        use vcf::variant::record_buf::{info::field::Value, Info};
        let info: Info = [
            (
                String::from("CLNACC"),
                self.record
                    .rcv
                    .as_ref()
                    .map(|s| Value::String(s.to_string())),
            ),
            (
                String::from("CLNSIG"),
                self.record
                    .clnsig
                    .as_ref()
                    .map(|s| Value::String(s.to_string())),
            ),
        ]
        .into_iter()
        .collect();
        match (
            &self.record.chromosome,
            &self.record.position,
            &self.record.reference,
            &self.record.alternate,
        ) {
            (Some(chrom), Some(pos), Some(reference), Some(alternate)) => {
                let vcf_record = vcf::variant::RecordBuf::builder()
                    .set_reference_sequence_name(chrom)
                    .set_variant_start(Position::new(*pos).expect("Invalid position"))
                    .set_reference_bases(reference)
                    .set_alternate_bases(vcf::variant::record_buf::AlternateBases::from(vec![
                        alternate.to_string(),
                    ]))
                    .set_variant_start(noodles_core::Position::MAX)
                    .set_info(info)
                    .build();
                self.vcf_writer
                    .write_variant_record(&self.vcf_header, &vcf_record)?;
            }
            _ => {}
        }

        // We've just wrote
        self.record = ClinVarRecord::default();

        Ok(())
    }
}

impl<W: std::io::Write> VCFRecordHandler<W> {
    pub fn new_from_writer_unchecked(
        writer: W,
        vcf_header: vcf::Header,
        assembly: &'static str,
    ) -> Self {
        let mut res = Self {
            vcf_writer: vcf::io::Writer::new(writer),
            record: ClinVarRecord::default(),
            vcf_header,
            assembly,
        };
        res.vcf_writer.write_header(&res.vcf_header).unwrap();
        res
    }

    pub fn new_from_writer(
        writer: W,
        vcf_header: vcf::Header,
        assembly: &'static str,
    ) -> Result<Self, ClinvarXMLTabError> {
        let mut res = Self {
            vcf_writer: vcf::io::Writer::new(writer),
            record: ClinVarRecord::default(),
            vcf_header,
            assembly,
        };
        res.vcf_writer.write_header(&res.vcf_header)?;
        Ok(res)
    }
}
