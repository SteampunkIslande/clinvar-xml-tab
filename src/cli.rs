#[derive(clap::Parser, std::fmt::Debug)]
#[command(
    name = "clinvar-xml-tab",
    author = "Charles Monod-Broca",
    version,
    about = "Convert XML clinvar files to tab separated values"
)]
pub struct Command {
    /// Input XML file
    #[clap(short = 'i', long = "input")]
    input: Option<std::path::PathBuf>,

    /// Output TSV file
    #[clap(short = 'o', long = "output")]
    output: Option<std::path::PathBuf>,

    /// Genome build
    #[clap(flatten)]
    genome: GenomeOption,
}

/// Options for the genome build (mutually exclusive)
#[derive(clap::Parser, std::fmt::Debug)]
#[group(multiple = false)]
struct GenomeOption {
    /// Human genome build 19 (incompatible with hg38)
    #[clap(long = "hg19")]
    hg19: bool,

    /// Human genome build 38 (incompatible with hg19)
    #[clap(long = "hg38")]
    hg38: bool,
}

pub enum Genome {
    Hg19,
    Hg38,
}

impl Command {
    pub fn input(&self) -> Option<&std::path::PathBuf> {
        self.input.as_ref()
    }

    pub fn output(&self) -> Option<&std::path::PathBuf> {
        self.output.as_ref()
    }

    pub fn genome(&self) -> Genome {
        match self.genome {
            GenomeOption {
                hg19: true,
                hg38: false,
            } => Genome::Hg19,
            GenomeOption {
                hg19: false,
                hg38: true,
            } => Genome::Hg38,
            _ => Genome::Hg38,
        }
    }
}
