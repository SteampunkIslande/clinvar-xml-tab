use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClinVarSet {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "RecordStatus")]
    pub record_status: String,
    #[serde(rename = "Replaces")]
    pub replaces: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "ReferenceClinVarAssertion")]
    pub reference_clin_var_assertion: ReferenceClinVarAssertion,
    #[serde(rename = "ClinVarAssertion")]
    pub clin_var_assertion: Vec<ClinVarAssertion>,
}

#[derive(Serialize, Deserialize)]
pub struct ReferenceClinVarAssertion {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@DateLastUpdated")]
    pub date_last_updated: String,
    #[serde(rename = "@DateCreated")]
    pub date_created: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClinVarAssertion {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@DateLastUpdated")]
    pub date_last_updated: String,
    #[serde(rename = "@DateCreated")]
    pub date_created: String,
}
