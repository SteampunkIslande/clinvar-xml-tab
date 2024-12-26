use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ClinVarSet {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ClinVarAssertion")]
    pub clin_var_assertion: ClinVarAssertion,
    #[serde(rename = "RecordStatus")]
    pub record_status: String,
    #[serde(rename = "ReferenceClinVarAssertion")]
    pub reference_clin_var_assertion: ReferenceClinVarAssertion,
    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertion {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Assertion")]
    pub assertion: ClinVarAssertionAssertion,
    #[serde(rename = "Classification")]
    pub classification: Classification,
    #[serde(rename = "ClinVarAccession")]
    pub clin_var_accession: ClinVarAssertionClinVarAccession,
    #[serde(rename = "ClinVarSubmissionID")]
    pub clin_var_submission_id: ClinVarSubmissionId,
    #[serde(rename = "ExternalID")]
    pub external_id: ExternalId,
    #[serde(rename = "MeasureSet")]
    pub measure_set: ClinVarAssertionMeasureSet,
    #[serde(rename = "ObservedIn")]
    pub observed_in: ClinVarAssertionObservedIn,
    #[serde(rename = "RecordStatus")]
    pub record_status: String,
    #[serde(rename = "TraitSet")]
    pub trait_set: ClinVarAssertionTraitSet,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionAssertion {
    #[serde(rename = "@Type")]
    pub assertion_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct Classification {
    #[serde(rename = "@DateLastEvaluated")]
    pub date_last_evaluated: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "GermlineClassification")]
    pub germline_classification: String,
    #[serde(rename = "ReviewStatus")]
    pub review_status: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionClinVarAccession {
    #[serde(rename = "@Acc")]
    pub acc: String,
    #[serde(rename = "@DateCreated")]
    pub date_created: String,
    #[serde(rename = "@DateUpdated")]
    pub date_updated: String,
    #[serde(rename = "@OrgID")]
    pub org_id: String,
    #[serde(rename = "@OrgType")]
    pub org_type: String,
    #[serde(rename = "@OrganizationCategory")]
    pub organization_category: String,
    #[serde(rename = "@Type")]
    pub clin_var_accession_type: String,
    #[serde(rename = "@Version")]
    pub version: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSubmissionId {
    #[serde(rename = "@localKey")]
    pub local_key: String,
    #[serde(rename = "@submitter")]
    pub submitter: String,
    #[serde(rename = "@submitterDate")]
    pub submitter_date: String,
    #[serde(rename = "@title")]
    pub title: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExternalId {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub external_id_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionMeasureSet {
    #[serde(rename = "@Type")]
    pub measure_set_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Measure")]
    pub measure: ClinVarAssertionMeasureSetMeasure,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionMeasureSetMeasure {
    #[serde(rename = "@Type")]
    pub measure_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AttributeSet")]
    pub attribute_set: ClinVarAssertionMeasureSetMeasureAttributeSet,
    #[serde(rename = "MeasureRelationship")]
    pub measure_relationship: ClinVarAssertionMeasureSetMeasureMeasureRelationship,
    #[serde(rename = "Name")]
    pub name: ClinVarAssertionMeasureSetMeasureName,
    #[serde(rename = "XRef")]
    pub xref: ClinVarAssertionMeasureSetMeasureXref,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionMeasureSetMeasureAttributeSet {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Attribute")]
    pub attribute: ClinVarAssertionMeasureSetMeasureAttributeSetAttribute,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionMeasureSetMeasureAttributeSetAttribute {
    #[serde(rename = "@Type")]
    pub attribute_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionMeasureSetMeasureMeasureRelationship {
    #[serde(rename = "@Type")]
    pub measure_relationship_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Symbol")]
    pub symbol: ClinVarAssertionMeasureSetMeasureMeasureRelationshipSymbol,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionMeasureSetMeasureMeasureRelationshipSymbol {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElementValue")]
    pub element_value:
        ClinVarSetClinVarAssertionMeasureSetMeasureMeasureRelationshipSymbolElementValue,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetClinVarAssertionMeasureSetMeasureMeasureRelationshipSymbolElementValue {
    #[serde(rename = "@Type")]
    pub element_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionMeasureSetMeasureName {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElementValue")]
    pub element_value: ClinVarSetClinVarAssertionMeasureSetMeasureNameElementValue,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetClinVarAssertionMeasureSetMeasureNameElementValue {
    #[serde(rename = "@Type")]
    pub element_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionMeasureSetMeasureXref {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub xref_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionObservedIn {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Method")]
    pub method: ClinVarAssertionObservedInMethod,
    #[serde(rename = "ObservedData")]
    pub observed_data: ClinVarAssertionObservedInObservedData,
    #[serde(rename = "Sample")]
    pub sample: ClinVarAssertionObservedInSample,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionObservedInMethod {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "MethodType")]
    pub method_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionObservedInObservedData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Attribute")]
    pub attribute: ClinVarSetClinVarAssertionObservedInObservedDataAttribute,
    #[serde(rename = "Citation")]
    pub citation: ClinVarAssertionObservedInObservedDataCitation,
    #[serde(rename = "XRef")]
    pub xref: ClinVarAssertionObservedInObservedDataXref,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetClinVarAssertionObservedInObservedDataAttribute {
    #[serde(rename = "@Type")]
    pub attribute_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionObservedInObservedDataCitation {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ID")]
    pub id: ClinVarAssertionObservedInObservedDataCitationId,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionObservedInObservedDataCitationId {
    #[serde(rename = "@Source")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionObservedInObservedDataXref {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub xref_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionObservedInSample {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AffectedStatus")]
    pub affected_status: String,
    #[serde(rename = "Origin")]
    pub origin: String,
    #[serde(rename = "Species")]
    pub species: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionTraitSet {
    #[serde(rename = "@Type")]
    pub trait_set_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Trait")]
    pub trait_set_trait: ClinVarAssertionTraitSetTrait,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionTraitSetTrait {
    #[serde(rename = "@Type")]
    pub trait_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: ClinVarAssertionTraitSetTraitName,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarAssertionTraitSetTraitName {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElementValue")]
    pub element_value: ClinVarSetClinVarAssertionTraitSetTraitNameElementValue,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetClinVarAssertionTraitSetTraitNameElementValue {
    #[serde(rename = "@Type")]
    pub element_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertion {
    #[serde(rename = "@DateCreated")]
    pub date_created: String,
    #[serde(rename = "@DateLastUpdated")]
    pub date_last_updated: String,
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Assertion")]
    pub assertion: ReferenceClinVarAssertionAssertion,
    #[serde(rename = "Classifications")]
    pub classifications: Classifications,
    #[serde(rename = "ClinVarAccession")]
    pub clin_var_accession: ReferenceClinVarAssertionClinVarAccession,
    #[serde(rename = "MeasureSet")]
    pub measure_set: ReferenceClinVarAssertionMeasureSet,
    #[serde(rename = "ObservedIn")]
    pub observed_in: ReferenceClinVarAssertionObservedIn,
    #[serde(rename = "RecordStatus")]
    pub record_status: String,
    #[serde(rename = "TraitSet")]
    pub trait_set: ReferenceClinVarAssertionTraitSet,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionAssertion {
    #[serde(rename = "@Type")]
    pub assertion_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct Classifications {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "GermlineClassification")]
    pub germline_classification: ClassificationsGermlineClassification,
}

#[derive(Deserialize, Serialize)]
pub struct ClassificationsGermlineClassification {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Description")]
    pub description: Description,
    #[serde(rename = "ReviewStatus")]
    pub review_status: String,
}

#[derive(Deserialize, Serialize)]
pub struct Description {
    #[serde(rename = "@DateLastEvaluated")]
    pub date_last_evaluated: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionClinVarAccession {
    #[serde(rename = "@Acc")]
    pub acc: String,
    #[serde(rename = "@DateCreated")]
    pub date_created: String,
    #[serde(rename = "@DateUpdated")]
    pub date_updated: String,
    #[serde(rename = "@Type")]
    pub clin_var_accession_type: String,
    #[serde(rename = "@Version")]
    pub version: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionMeasureSet {
    #[serde(rename = "@Acc")]
    pub acc: String,
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub measure_set_type: String,
    #[serde(rename = "@Version")]
    pub version: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Measure")]
    pub measure: ReferenceClinVarAssertionMeasureSetMeasure,
    #[serde(rename = "Name")]
    pub name: ClinVarSetReferenceClinVarAssertionMeasureSetName,
    #[serde(rename = "XRef")]
    pub xref: ClinVarSetReferenceClinVarAssertionMeasureSetXref,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionMeasureSetMeasure {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub measure_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AttributeSet")]
    pub attribute_set: Vec<ReferenceClinVarAssertionMeasureSetMeasureAttributeSet>,
    #[serde(rename = "CanonicalSPDI")]
    pub canonical_spdi: String,
    #[serde(rename = "CytogeneticLocation")]
    pub cytogenetic_location: String,
    #[serde(rename = "MeasureRelationship")]
    pub measure_relationship: ReferenceClinVarAssertionMeasureSetMeasureMeasureRelationship,
    #[serde(rename = "Name")]
    pub name: ReferenceClinVarAssertionMeasureSetMeasureName,
    #[serde(rename = "SequenceLocation")]
    pub sequence_location: Vec<MeasureSequenceLocation>,
    #[serde(rename = "XRef")]
    pub xref: Vec<ReferenceClinVarAssertionMeasureSetMeasureXref>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionMeasureSetMeasureAttributeSet {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Attribute")]
    pub attribute: ReferenceClinVarAssertionMeasureSetMeasureAttributeSetAttribute,
    #[serde(rename = "XRef")]
    pub xref: Vec<MeasureSetMeasureAttributeSetXref>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionMeasureSetMeasureAttributeSetAttribute {
    #[serde(rename = "@Accession")]
    pub accession: Option<String>,
    #[serde(rename = "@Change")]
    pub change: Option<String>,
    #[serde(rename = "@MANESelect")]
    pub maneselect: Option<String>,
    #[serde(rename = "@Type")]
    pub attribute_type: String,
    #[serde(rename = "@Version")]
    pub version: Option<String>,
    #[serde(rename = "@integerValue")]
    pub integer_value: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MeasureSetMeasureAttributeSetXref {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionMeasureSetMeasureMeasureRelationship {
    #[serde(rename = "@Type")]
    pub measure_relationship_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: MeasureSetMeasureMeasureRelationshipName,
    #[serde(rename = "SequenceLocation")]
    pub sequence_location: Vec<MeasureRelationshipSequenceLocation>,
    #[serde(rename = "Symbol")]
    pub symbol: ReferenceClinVarAssertionMeasureSetMeasureMeasureRelationshipSymbol,
    #[serde(rename = "XRef")]
    pub xref: Vec<MeasureSetMeasureMeasureRelationshipXref>,
}

#[derive(Deserialize, Serialize)]
pub struct MeasureSetMeasureMeasureRelationshipName {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElementValue")]
    pub element_value:
        ClinVarSetReferenceClinVarAssertionMeasureSetMeasureMeasureRelationshipNameElementValue,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionMeasureSetMeasureMeasureRelationshipNameElementValue {
    #[serde(rename = "@Type")]
    pub element_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MeasureRelationshipSequenceLocation {
    #[serde(rename = "@Accession")]
    pub accession: String,
    #[serde(rename = "@Assembly")]
    pub assembly: String,
    #[serde(rename = "@AssemblyAccessionVersion")]
    pub assembly_accession_version: String,
    #[serde(rename = "@AssemblyStatus")]
    pub assembly_status: String,
    #[serde(rename = "@Chr")]
    pub chr: String,
    #[serde(rename = "@Strand")]
    pub strand: String,
    #[serde(rename = "@display_start")]
    pub display_start: String,
    #[serde(rename = "@display_stop")]
    pub display_stop: String,
    #[serde(rename = "@start")]
    pub start: String,
    #[serde(rename = "@stop")]
    pub stop: String,
    #[serde(rename = "@variantLength")]
    pub variant_length: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionMeasureSetMeasureMeasureRelationshipSymbol {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElementValue")]
    pub element_value:
        ClinVarSetReferenceClinVarAssertionMeasureSetMeasureMeasureRelationshipSymbolElementValue,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionMeasureSetMeasureMeasureRelationshipSymbolElementValue
{
    #[serde(rename = "@Type")]
    pub element_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MeasureSetMeasureMeasureRelationshipXref {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub xref_type: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionMeasureSetMeasureName {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElementValue")]
    pub element_value: ClinVarSetReferenceClinVarAssertionMeasureSetMeasureNameElementValue,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionMeasureSetMeasureNameElementValue {
    #[serde(rename = "@Type")]
    pub element_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MeasureSequenceLocation {
    #[serde(rename = "@Accession")]
    pub accession: String,
    #[serde(rename = "@Assembly")]
    pub assembly: String,
    #[serde(rename = "@AssemblyAccessionVersion")]
    pub assembly_accession_version: String,
    #[serde(rename = "@AssemblyStatus")]
    pub assembly_status: String,
    #[serde(rename = "@Chr")]
    pub chr: String,
    #[serde(rename = "@alternateAlleleVCF")]
    pub alternate_allele_vcf: String,
    #[serde(rename = "@display_start")]
    pub display_start: String,
    #[serde(rename = "@display_stop")]
    pub display_stop: String,
    #[serde(rename = "@positionVCF")]
    pub position_vcf: String,
    #[serde(rename = "@referenceAlleleVCF")]
    pub reference_allele_vcf: String,
    #[serde(rename = "@start")]
    pub start: String,
    #[serde(rename = "@stop")]
    pub stop: String,
    #[serde(rename = "@variantLength")]
    pub variant_length: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionMeasureSetMeasureXref {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub xref_type: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionMeasureSetName {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElementValue")]
    pub element_value: ClinVarSetReferenceClinVarAssertionMeasureSetNameElementValue,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionMeasureSetNameElementValue {
    #[serde(rename = "@Type")]
    pub element_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionMeasureSetXref {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionObservedIn {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Method")]
    pub method: ReferenceClinVarAssertionObservedInMethod,
    #[serde(rename = "ObservedData")]
    pub observed_data: ReferenceClinVarAssertionObservedInObservedData,
    #[serde(rename = "Sample")]
    pub sample: ReferenceClinVarAssertionObservedInSample,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionObservedInMethod {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "MethodType")]
    pub method_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionObservedInObservedData {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Attribute")]
    pub attribute: ClinVarSetReferenceClinVarAssertionObservedInObservedDataAttribute,
    #[serde(rename = "Citation")]
    pub citation: ReferenceClinVarAssertionObservedInObservedDataCitation,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionObservedInObservedDataAttribute {
    #[serde(rename = "@Type")]
    pub attribute_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionObservedInObservedDataCitation {
    #[serde(rename = "@Type")]
    pub citation_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ID")]
    pub id: ReferenceClinVarAssertionObservedInObservedDataCitationId,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionObservedInObservedDataCitationId {
    #[serde(rename = "@Source")]
    pub source: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionObservedInSample {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "AffectedStatus")]
    pub affected_status: String,
    #[serde(rename = "Origin")]
    pub origin: String,
    #[serde(rename = "Species")]
    pub species: ReferenceClinVarAssertionObservedInSampleSpecies,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionObservedInSampleSpecies {
    #[serde(rename = "@TaxonomyId")]
    pub taxonomy_id: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionTraitSet {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub trait_set_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Trait")]
    pub trait_set_trait: ReferenceClinVarAssertionTraitSetTrait,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionTraitSetTrait {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub trait_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Name")]
    pub name: Vec<ReferenceClinVarAssertionTraitSetTraitName>,
    #[serde(rename = "Symbol")]
    pub symbol: Vec<ClinVarSetReferenceClinVarAssertionTraitSetTraitSymbol>,
    #[serde(rename = "XRef")]
    pub xref: Vec<ReferenceClinVarAssertionTraitSetTraitXref>,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionTraitSetTraitName {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElementValue")]
    pub element_value: ClinVarSetReferenceClinVarAssertionTraitSetTraitNameElementValue,
    #[serde(rename = "XRef")]
    pub xref: Vec<TraitSetTraitNameXref>,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionTraitSetTraitNameElementValue {
    #[serde(rename = "@Type")]
    pub element_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TraitSetTraitNameXref {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionTraitSetTraitSymbol {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "ElementValue")]
    pub element_value: ClinVarSetReferenceClinVarAssertionTraitSetTraitSymbolElementValue,
    #[serde(rename = "XRef")]
    pub xref: TraitSetTraitSymbolXref,
}

#[derive(Deserialize, Serialize)]
pub struct ClinVarSetReferenceClinVarAssertionTraitSetTraitSymbolElementValue {
    #[serde(rename = "@Type")]
    pub element_value_type: String,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TraitSetTraitSymbolXref {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub xref_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReferenceClinVarAssertionTraitSetTraitXref {
    #[serde(rename = "@DB")]
    pub db: String,
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Type")]
    pub xref_type: Option<String>,
}
