use serde::Deserialize;

use crate::cli::Genome;

#[derive(Debug, Deserialize)]
pub struct ClinVarSet {
    #[serde(rename = "ReferenceClinVarAssertion")]
    pub reference_clinvar_assertion: ReferenceClinVarAssertion,

    #[serde(rename = "Title")]
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct ReferenceClinVarAssertion {
    #[serde(rename = "@ID")]
    pub id: String,

    #[serde(rename = "@DateLastUpdated")]
    pub date_last_updated: String,

    #[serde(rename = "@DateCreated")]
    pub date_created: String,

    #[serde(rename = "ClinVarAccession")]
    pub clinvar_accession: ClinVarAccession,

    #[serde(rename = "Classifications")]
    pub classifications: Classifications,

    #[serde(rename = "MeasureSet")]
    pub measure_set: MeasureSet,
}

#[derive(Debug, Deserialize)]
pub struct Classifications {
    #[serde(rename = "GermlineClassification")]
    pub germline: Vec<GermlineClassification>,
}

#[derive(Debug, Deserialize)]
pub struct ClinVarAccession {
    #[serde(rename = "@Acc")]
    pub rcv: String,
}

#[derive(Debug, Deserialize)]
pub struct GermlineClassification {
    // #[serde(rename = "Description")]
    // pub description: String,
    #[serde(rename = "Description")]
    pub description: GermlineClassificationDescription,
}

#[derive(Debug, Deserialize)]
pub struct GermlineClassificationDescription {
    #[serde(rename = "$text")]
    pub description: String,

    #[serde(rename = "@DateLastEvaluated")]
    pub date_last_evaluated: String,
}

#[derive(Debug, Deserialize)]
pub struct MeasureSet {
    #[serde(rename = "Measure")]
    pub measure: Vec<Measure>,

    #[serde(rename = "@Acc")]
    pub acc: String,
}

#[derive(Debug, Deserialize)]
pub struct Measure {
    #[serde(rename = "@Type")]
    pub measure_type: String,

    #[serde(rename = "@ID")]
    pub id: String,

    #[serde(rename = "SequenceLocation")]
    pub sequence_location: Vec<Option<SequenceLocation>>,
}

#[derive(Debug, Deserialize)]
pub struct SequenceLocation {
    #[serde(rename = "@Assembly")]
    pub assembly: String,

    #[serde(rename = "@Chr")]
    pub chr: String,

    #[serde(rename = "@Accession")]
    pub accession: String,

    #[serde(rename = "@positionVCF")]
    pub position_vcf: String,

    #[serde(rename = "@referenceAlleleVCF")]
    pub reference_allele_vcf: String,

    #[serde(rename = "@alternateAlleleVCF")]
    pub alternate_allele_vcf: String,
}

impl ClinVarSet {
    pub fn print_chrom(&self, g: &Genome) -> Option<String> {
        let mut chrom = None;
        for measure in &self.reference_clinvar_assertion.measure_set.measure {
            for sequence_location in &measure.sequence_location {
                if let Some(sequence_location) = sequence_location {
                    match g {
                        Genome::Hg19 => {
                            if sequence_location.assembly == "GRCh37" {
                                chrom = Some(sequence_location.chr.clone());
                            }
                        }
                        Genome::Hg38 => {
                            if sequence_location.assembly == "GRCh38" {
                                chrom = Some(sequence_location.chr.clone());
                            }
                        }
                    }
                }
            }
        }
        chrom
    }

    pub fn print_pos(&self, g: &Genome) -> Option<String> {
        let mut pos = None;
        for measure in &self.reference_clinvar_assertion.measure_set.measure {
            for sequence_location in &measure.sequence_location {
                if let Some(sequence_location) = sequence_location {
                    match g {
                        Genome::Hg19 => {
                            if sequence_location.assembly == "GRCh37" {
                                pos = Some(sequence_location.position_vcf.clone());
                            }
                        }
                        Genome::Hg38 => {
                            if sequence_location.assembly == "GRCh38" {
                                pos = Some(sequence_location.position_vcf.clone());
                            }
                        }
                    }
                }
            }
        }
        pos
    }

    pub fn print_ref(&self, g: &Genome) -> Option<String> {
        let mut ref_allele = None;
        for measure in &self.reference_clinvar_assertion.measure_set.measure {
            for sequence_location in &measure.sequence_location {
                if let Some(sequence_location) = sequence_location {
                    match g {
                        Genome::Hg19 => {
                            if sequence_location.assembly == "GRCh37" {
                                ref_allele = Some(sequence_location.reference_allele_vcf.clone());
                            }
                        }
                        Genome::Hg38 => {
                            if sequence_location.assembly == "GRCh38" {
                                ref_allele = Some(sequence_location.reference_allele_vcf.clone());
                            }
                        }
                    }
                }
            }
        }
        ref_allele
    }

    pub fn print_alt(&self, g: &Genome) -> Option<String> {
        let mut alt_allele = None;
        for measure in &self.reference_clinvar_assertion.measure_set.measure {
            for sequence_location in &measure.sequence_location {
                if let Some(sequence_location) = sequence_location {
                    match g {
                        Genome::Hg19 => {
                            if sequence_location.assembly == "GRCh37" {
                                alt_allele = Some(sequence_location.alternate_allele_vcf.clone());
                            }
                        }
                        Genome::Hg38 => {
                            if sequence_location.assembly == "GRCh38" {
                                alt_allele = Some(sequence_location.alternate_allele_vcf.clone());
                            }
                        }
                    }
                }
            }
        }
        alt_allele
    }

    pub fn print_rcv(&self) -> String {
        self.reference_clinvar_assertion
            .clinvar_accession
            .rcv
            .clone()
    }

    pub fn print_date_last_evaluated(&self) -> String {
        self.reference_clinvar_assertion.classifications.germline[0]
            .description
            .date_last_evaluated
            .clone()
    }

    pub fn print_description(&self) -> String {
        self.reference_clinvar_assertion.classifications.germline[0]
            .description
            .description
            .clone()
    }
    pub fn print_info(&self) -> String {
        [
            format!("CLNACC={}", self.print_rcv()),
            format!(
                "CLNSIG={}",
                self.print_description().to_lowercase().replace(" ", "_")
            ),
        ]
        .join(";")
    }
}
