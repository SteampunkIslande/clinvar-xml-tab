use clap_complete::Shell;

#[derive(clap::Parser, std::fmt::Debug)]
#[command(
    name = "clinvar-xml-tab",
    author = "Charles Monod-Broca",
    version,
    about = "Convert XML clinvar files to tab separated values"
)]
pub struct Cli {
    /// Input XML file
    #[clap(short = 'i', long = "input")]
    input: Option<std::path::PathBuf>,

    /// Output TSV file
    #[clap(short = 'o', long = "output")]
    output: Option<std::path::PathBuf>,

    /// Genome build
    #[clap(flatten)]
    genome: GenomeOption,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    /// Convert XML to TSV
    #[clap(name = "convert")]
    Convert(Convert),

    /// Only print out the very first XML element of input
    #[clap(name = "debug")]
    Debug(Debug),

    /// Generate Autocompletion
    #[clap(name = "autocomplete")]
    AutoComplete(AutoComplete),
}

#[derive(clap::Parser, Debug)]
pub struct AutoComplete {
    /// The shell to use
    #[clap(short = 's', long = "shell")]
    shell: clap_complete::Shell,
}

#[derive(clap::Parser, Debug)]
pub struct Convert {}

#[derive(clap::Parser, Debug)]
pub struct Debug {}

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

impl Cli {
    pub fn command(&self) -> &Command {
        return &self.command;
    }

    pub fn input(&self) -> Option<&std::path::PathBuf> {
        self.input.as_ref()
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

    pub fn output(&self) -> Option<&std::path::PathBuf> {
        self.output.as_ref()
    }
}

impl AutoComplete {
    pub fn shell(&self) -> Shell {
        self.shell
    }
}
