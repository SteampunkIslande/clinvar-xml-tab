use crate::handler::EventHandler;

pub use crate::clinvar::record::ClinVarRecord;

pub struct RecordHandler {
    record: ClinVarRecord,
}

impl EventHandler for RecordHandler {
    fn handle(
        &mut self,
        node: &roxmltree::Node,
        current_path: &Vec<String>,
        attributes: &std::collections::HashMap<String, String>,
        depth: i32,
    ) -> Result<(), crate::error::ClinvarXMLTabError> {
        if depth == 2 {
            if node.has_tag_name("RecordStatus") {
                self.record.status = node.text().unwrap_or("").to_string();
            }
        }
        Ok(())
    }
}

impl RecordHandler {}
