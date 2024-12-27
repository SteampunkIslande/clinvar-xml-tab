// This crate's CLI definition module
pub mod cli;

// Clap usage
use clap::Parser;

// Clap-complete usage
use clap_complete::aot::generate;
use clap_complete::Shell;

// Use this crate's lib
use clinvar_xml_tab;
use clinvar_xml_tab::reader;
use clinvar_xml_tab::utils;

fn convert(
    params: &cli::Cli,
    _subparams: &cli::Convert,
) -> Result<(), clinvar_xml_tab::error::ClinvarXMLTabError> {
    let in_stream = utils::file_reader(params.input())?;
    let out_stream = utils::file_writer(params.output())?;

    let mut handler = clinvar_xml_tab::handler::BasicNodeWriter::new(out_stream);

    reader::read_xml(in_stream, &mut handler, None)?;

    Ok(())
}

fn debug(
    params: &cli::Cli,
    _subparams: &cli::Debug,
) -> Result<(), clinvar_xml_tab::error::ClinvarXMLTabError> {
    let in_stream = utils::file_reader(params.input())?;
    let out_stream = utils::file_writer(params.output())?;

    let mut handler = clinvar_xml_tab::handler::BasicNodeWriter::new(out_stream);

    reader::read_xml(in_stream, &mut handler, Some(1))?;

    Ok(())
}

fn auto_complete(
    _args: &cli::Cli,
    subparams: &cli::AutoComplete,
) -> Result<(), clinvar_xml_tab::error::ClinvarXMLTabError> {
    let mut command = <cli::Cli as clap::CommandFactory>::command();
    let command_name = command.get_name().to_string();
    let gen = subparams.shell();

    let output_dir = match gen {
        Shell::Zsh => {
            if let Some(d) =
                home::home_dir().and_then(|d| Some(d.join(".oh-my-zsh").join("completions")))
            {
                std::fs::DirBuilder::new().recursive(true).create(&d)?;
                d
            } else {
                std::env::current_dir()?
            }
        }
        _ => std::env::current_dir()?,
    };

    let mut output: Box<dyn std::io::Write> = match gen {
        clap_complete::Shell::Zsh => Box::new(std::fs::File::create(
            output_dir.join("_clinvar-xml-tab_ZSH"),
        )?),
        _ => Box::new(std::io::stdout()),
    };
    eprintln!("Writing completions for {:?}...", &gen);
    generate(gen, &mut command, command_name.to_string(), &mut output);
    eprintln!("Done!");
    Ok(())
}

fn main() -> Result<(), clinvar_xml_tab::error::ClinvarXMLTabError> {
    let args = cli::Cli::parse();

    match args.command() {
        cli::Command::Convert(subparams) => convert(&args, subparams)?,
        cli::Command::Debug(subparams) => debug(&args, subparams)?,
        cli::Command::AutoComplete(subparams) => auto_complete(&args, subparams)?,
    }

    Ok(())
}
