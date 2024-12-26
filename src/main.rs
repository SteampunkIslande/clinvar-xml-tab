use clap::Parser;

pub mod cli;
pub mod error;
pub mod utils;

use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};

use clap::CommandFactory;
use clap_complete::aot::generate;

use std::io::BufRead;
use std::str::FromStr;

// Thanks https://capnfabs.net/posts/parsing-huge-xml-quickxml-rust-serde/
fn read_to_end_into_buffer<R: BufRead>(
    reader: &mut Reader<R>,
    start_tag: &BytesStart,
    junk_buf: &mut Vec<u8>,
) -> Result<Vec<u8>, quick_xml::Error> {
    let mut depth = 0;
    let mut output_buf: Vec<u8> = Vec::new();
    let mut w = Writer::new(&mut output_buf);
    let tag_name = start_tag.name();
    w.write_event(Event::Start(start_tag.clone()))?;
    loop {
        junk_buf.clear();
        let event = reader.read_event_into(junk_buf)?;
        w.write_event(event.clone())?;

        match event {
            Event::Start(e) if e.name() == tag_name => depth += 1,
            Event::End(e) if e.name() == tag_name => {
                if depth == 0 {
                    return Ok(output_buf);
                }
                depth -= 1;
            }
            Event::Eof => {
                panic!("Truncated or invalid XML input file!")
            }
            _ => {}
        }
    }
}

fn write_flatten_node_to(
    node: &roxmltree::Node,
    current_path: &mut Vec<String>,
    writer: &mut dyn std::io::Write,
) -> std::io::Result<()> {
    if node.is_element() {
        current_path.push(node.tag_name().name().to_string());
        writeln!(
            writer,
            "{} - {} - {}",
            current_path.join("."),
            node.text().unwrap_or("No text"),
            if node.attributes().len() == 0 {
                "No attributes".to_string()
            } else {
                node.attributes()
                    .map(|f| format!("{}={}", f.name(), f.value()))
                    .collect::<Vec<String>>()
                    .join(" ")
            }
        )?;
    }
    for child in node.children() {
        write_flatten_node_to(&child, current_path, writer)?;
    }
    current_path.pop();
    Ok(())
}

fn convert(params: &cli::Cli, _subparams: &cli::Convert) -> Result<(), error::ClinvarXMLTabError> {
    let in_stream = utils::file_reader(params.input())?;
    let mut out_stream = utils::file_writer(params.output())?;

    let mut reader = Reader::from_reader(in_stream);
    reader.config_mut().trim_text(true);

    // let mut total_counter = 0;
    // let mut ignored_counter = 0;

    let writer = csv::Writer::from_writer(out_stream.as_mut());
    let mut buf = Vec::new();
    let mut junk_buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"ClinVarSet" => {
                    // load entire tag into buffer
                    let elem_bytes = read_to_end_into_buffer(&mut reader, &e, &mut junk_buf)?;
                    let str = std::str::from_utf8(&elem_bytes)?.to_string();
                    // out_stream.write(str.as_ref())?;
                    let doc: roxmltree::Document = roxmltree::Document::parse(&str)?;

                    let mut current_path = vec![];
                    write_flatten_node_to(
                        &doc.root(),
                        &mut current_path,
                        writer.into_inner().expect("Cannot get writer"),
                    )?;
                    break;
                }
                _ => (),
            },
            // Other Events are not important for us
            _ => (),
        }
        // clear buffer to prevent memory leak
        buf.clear();
    }
    Ok(())
}

fn debug(params: &cli::Cli, _subparams: &cli::Debug) -> Result<(), error::ClinvarXMLTabError> {
    let in_stream = utils::file_reader(params.input())?;
    let mut out_stream = utils::file_writer(params.output())?;

    let mut reader = Reader::from_reader(in_stream);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut junk_buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"ClinVarSet" => {
                        // load entire element into buffer
                        let elem_bytes = read_to_end_into_buffer(&mut reader, &e, &mut junk_buf)?;
                        let str = std::str::from_utf8(&elem_bytes)?.to_string();
                        // out_stream.write(str.as_ref())?;
                        let doc: roxmltree::Document = roxmltree::Document::parse(&str)?;

                        let mut current_path = vec![];

                        //Only write the first element
                        write_flatten_node_to(&doc.root(), &mut current_path, out_stream.as_mut())?;
                        break;
                    }
                    _ => (),
                }
            }
            // Other Events are not important for us
            _ => (),
        }
        // clear buffer to prevent memory leak
        buf.clear();
    }
    Ok(())
}

fn auto_complete(
    _args: &cli::Cli,
    subparams: &cli::AutoComplete,
) -> Result<(), error::ClinvarXMLTabError> {
    let mut command = cli::Cli::command();
    let command_name = command.get_name().to_string();
    let gen = subparams.shell();

    let mut output: Box<dyn std::io::Write> = match gen {
        clap_complete::Shell::Zsh => Box::new(std::fs::File::create(
            home::home_dir()
                .and_then(|h| {
                    Some(
                        h.join(".oh-my-zsh")
                            .join("completions")
                            .join("_clinvar-xml-tab_ZSH"),
                    )
                })
                .unwrap_or(
                    std::path::PathBuf::from_str("_clinvar-xml-tab_ZSH")
                        .expect("Invalid file name"),
                ),
        )?),
        _ => Box::new(std::io::stdout()),
    };

    generate(gen, &mut command, command_name.to_string(), &mut output);
    Ok(())
}

fn main() -> Result<(), error::ClinvarXMLTabError> {
    let args = cli::Cli::parse();

    match args.get_command() {
        cli::Command::Convert(subparams) => convert(&args, subparams)?,
        cli::Command::Debug(subparams) => debug(&args, subparams)?,
        cli::Command::AutoComplete(subparams) => auto_complete(&args, subparams)?,
    }

    Ok(())
}
