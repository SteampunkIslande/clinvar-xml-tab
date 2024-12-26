use serde::Serialize;

#[derive(Serialize)]
pub struct ClinvarRecord {
    clinvar_set_id: i64,
    record_status: String,
    record_title: String,
}

pub struct RecordBuilder {
    _records: Vec<ClinvarRecord>,
}

impl RecordBuilder {
    pub fn new() -> Self {
        RecordBuilder { _records: vec![] }
    }
}
