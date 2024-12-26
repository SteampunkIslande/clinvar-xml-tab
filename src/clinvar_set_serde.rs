use super::error::ClinvarXMLTabError;
use quick_xml::de::Deserializer;
use serde::Deserialize;

use crate::cli::Genome;

#[derive(Debug, Deserialize)]
pub struct ClinVarSet {
    #[serde(rename = "ReferenceClinVarAssertion")]
    reference_clinvar_assertion: ReferenceClinVarAssertion,

    #[serde(rename = "Title")]
    title: String,
}

#[derive(Debug, Deserialize)]
struct ReferenceClinVarAssertion {
    #[serde(rename = "@ID")]
    id: String,

    #[serde(rename = "@DateLastUpdated")]
    date_last_updated: String,

    #[serde(rename = "@DateCreated")]
    date_created: String,

    #[serde(rename = "ClinVarAccession")]
    clinvar_accession: ClinVarAccession,

    #[serde(rename = "Classifications")]
    classifications: Classifications,

    #[serde(rename = "MeasureSet")]
    measure_set: MeasureSet,
}

#[derive(Debug, Deserialize)]
struct Classifications {
    #[serde(rename = "GermlineClassification")]
    germline: Vec<GermlineClassification>,
}

#[derive(Debug, Deserialize)]
struct ClinVarAccession {
    #[serde(rename = "@Acc")]
    rcv: String,
}

#[derive(Debug, Deserialize)]
struct GermlineClassification {
    // #[serde(rename = "Description")]
    //  description: String,
    #[serde(rename = "Description")]
    description: GermlineClassificationDescription,
}

#[derive(Debug, Deserialize)]
struct GermlineClassificationDescription {
    #[serde(rename = "$text")]
    description: String,

    #[serde(rename = "@DateLastEvaluated")]
    date_last_evaluated: String,
}

#[derive(Debug, Deserialize)]
struct MeasureSet {
    #[serde(rename = "Measure")]
    measure: Vec<Measure>,

    #[serde(rename = "@Acc")]
    acc: String,
}

#[derive(Debug, Deserialize)]
struct Measure {
    #[serde(rename = "@Type")]
    measure_type: String,

    #[serde(rename = "@ID")]
    id: String,

    #[serde(rename = "SequenceLocation")]
    sequence_location: Vec<Option<SequenceLocation>>,
}

#[derive(Debug, Deserialize)]
struct SequenceLocation {
    #[serde(rename = "@Assembly")]
    assembly: String,

    #[serde(rename = "@Chr")]
    chr: String,

    #[serde(rename = "@Accession")]
    accession: String,

    #[serde(rename = "@positionVCF")]
    position_vcf: String,

    #[serde(rename = "@referenceAlleleVCF")]
    reference_allele_vcf: String,

    #[serde(rename = "@alternateAlleleVCF")]
    alternate_allele_vcf: String,
}

impl ClinVarSet {
    pub fn new_from_str(s: &str) -> Result<Self, ClinvarXMLTabError> {
        let mut deserializer = Deserializer::from_str(s);
        let clinvar_set: Result<Self, ClinvarXMLTabError> =
            Deserialize::deserialize(&mut deserializer).map_err(|e| e.into());
        clinvar_set
    }

    pub fn get_chrom(&self, g: &Genome) -> Option<&str> {
        let mut chrom = None;
        for measure in &self.reference_clinvar_assertion.measure_set.measure {
            for sequence_location in &measure.sequence_location {
                if let Some(sequence_location) = sequence_location {
                    match g {
                        Genome::Hg19 => {
                            if sequence_location.assembly == "GRCh37" {
                                chrom = Some(sequence_location.chr.as_ref());
                            }
                        }
                        Genome::Hg38 => {
                            if sequence_location.assembly == "GRCh38" {
                                chrom = Some(sequence_location.chr.as_ref());
                            }
                        }
                    }
                }
            }
        }
        chrom
    }

    pub fn get_pos(&self, g: &Genome) -> Option<&str> {
        let mut pos = None;
        for measure in &self.reference_clinvar_assertion.measure_set.measure {
            for sequence_location in &measure.sequence_location {
                if let Some(sequence_location) = sequence_location {
                    match g {
                        Genome::Hg19 => {
                            if sequence_location.assembly == "GRCh37" {
                                pos = Some(sequence_location.position_vcf.as_ref());
                            }
                        }
                        Genome::Hg38 => {
                            if sequence_location.assembly == "GRCh38" {
                                pos = Some(sequence_location.position_vcf.as_ref());
                            }
                        }
                    }
                }
            }
        }
        pos
    }

    pub fn get_ref(&self, g: &Genome) -> Option<&str> {
        let mut ref_allele = None;
        for measure in &self.reference_clinvar_assertion.measure_set.measure {
            for sequence_location in &measure.sequence_location {
                if let Some(sequence_location) = sequence_location {
                    match g {
                        Genome::Hg19 => {
                            if sequence_location.assembly == "GRCh37" {
                                ref_allele = Some(sequence_location.reference_allele_vcf.as_ref());
                            }
                        }
                        Genome::Hg38 => {
                            if sequence_location.assembly == "GRCh38" {
                                ref_allele = Some(sequence_location.reference_allele_vcf.as_ref());
                            }
                        }
                    }
                }
            }
        }
        ref_allele
    }

    pub fn get_alt(&self, g: &Genome) -> Option<&str> {
        let mut alt_allele = None;
        for measure in &self.reference_clinvar_assertion.measure_set.measure {
            for sequence_location in &measure.sequence_location {
                if let Some(sequence_location) = sequence_location {
                    match g {
                        Genome::Hg19 => {
                            if sequence_location.assembly == "GRCh37" {
                                alt_allele = Some(sequence_location.alternate_allele_vcf.as_ref());
                            }
                        }
                        Genome::Hg38 => {
                            if sequence_location.assembly == "GRCh38" {
                                alt_allele = Some(sequence_location.alternate_allele_vcf.as_ref());
                            }
                        }
                    }
                }
            }
        }
        alt_allele
    }

    pub fn get_rcv(&self) -> &str {
        &self.reference_clinvar_assertion.clinvar_accession.rcv
    }

    pub fn get_date_last_evaluated(&self) -> &str {
        &self.reference_clinvar_assertion.classifications.germline[0]
            .description
            .date_last_evaluated
    }

    pub fn get_description(&self) -> &str {
        &self.reference_clinvar_assertion.classifications.germline[0]
            .description
            .description
    }

    pub fn get_repr(&self, g: &Genome) -> Option<String> {
        match (
            self.get_chrom(g),
            self.get_pos(g),
            self.get_ref(g),
            self.get_alt(g),
        ) {
            (Some(chrom), Some(pos), Some(ref_allele), Some(alt_allele)) => match chrom {
                "MT" => Some(format!(
                    "chrM\t{}\t{}\t{}\t{}\t{}\n",
                    pos,
                    ref_allele,
                    alt_allele,
                    self.get_rcv(),
                    self.get_description()
                )),
                _ => Some(format!(
                    "chr{}\t{}\t{}\t{}\t{}\t{}\n",
                    chrom,
                    pos,
                    ref_allele,
                    alt_allele,
                    self.get_rcv(),
                    self.get_description()
                )),
            },
            _ => None,
        }
    }
}
