use serde::Serialize;

#[derive(Serialize)]
pub struct ClinVarRecord {
    pub clinvar_id: u64,
    pub status: String,
    pub rcv: String,
    pub vcv: String,
    pub clnsig: String,
    pub description: String,
    pub date_last_updated: String,
}
