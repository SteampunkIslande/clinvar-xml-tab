use clap::Parser;

pub mod cli;
pub mod error;
pub mod utils;

use quick_xml::de::Deserializer;
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use serde::{Deserialize, Serialize};

use std::io::BufRead;

pub mod clinvar_set_serde_auto_new;

use clinvar_set_serde_auto_new::ClinVarSet;

impl ClinVarSet {
    pub fn new_from_str(s: &str) -> Result<Self, error::ClinvarXMLTabError> {
        let mut deserializer = Deserializer::from_str(s);
        let clinvar_set: Result<Self, error::ClinvarXMLTabError> =
            Deserialize::deserialize(&mut deserializer).map_err(|e| e.into());
        clinvar_set
    }
}

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
                panic!("oh no")
            }
            _ => {}
        }
    }
}

fn convert(params: &cli::Cli, subparams: &cli::Convert) -> Result<(), error::ClinvarXMLTabError> {
    let in_stream = utils::file_reader(params.input())?;
    let out_stream = utils::file_writer(subparams.output())?;

    let mut reader = Reader::from_reader(in_stream);
    reader.config_mut().trim_text(true);

    // let mut total_counter = 0;
    // let mut ignored_counter = 0;

    let mut writer = serde_json::Serializer::new(out_stream);
    let mut buf = Vec::new();
    let mut junk_buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"ClinVarSet" => {
                        eprintln!("Found ClinvarSet element");
                        // load entire tag into buffer
                        let clinvar_set_bytes =
                            read_to_end_into_buffer(&mut reader, &e, &mut junk_buf).unwrap();
                        let str = std::str::from_utf8(&clinvar_set_bytes)?;
                        // deserialize from buffer
                        let clinvar_set: Result<ClinVarSet, error::ClinvarXMLTabError> =
                            clinvar_set_serde_auto_new::ClinVarSet::new_from_str(str);
                        // if let Ok(clinvar_set) = clinvar_set {
                        //     clinvar_set.serialize(&mut writer)?;
                        //     eprintln!("Hello!");
                        //     break;
                        // } else {
                        //     eprintln!("Ignored");
                        //     ignored_counter += 1;
                        // }
                        match clinvar_set {
                            Ok(set) => {
                                let _ = set.serialize(&mut writer);
                                break;
                            }
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }
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

fn debug(params: &cli::Cli, subparams: &cli::Debug) -> Result<(), error::ClinvarXMLTabError> {
    let in_stream = utils::file_reader(params.input())?;
    let mut out_stream = utils::file_writer(subparams.output())?;

    let mut reader = Reader::from_reader(in_stream);
    reader.config_mut().trim_text(true);

    // let mut total_counter = 0;
    // let mut ignored_counter = 0;

    let mut buf = Vec::new();
    let mut junk_buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"ClinVarSet" => {
                        eprintln!("Found ClinvarSet element");
                        // load entire tag into buffer
                        let clinvar_set_bytes =
                            read_to_end_into_buffer(&mut reader, &e, &mut junk_buf).unwrap();
                        let str = std::str::from_utf8(&clinvar_set_bytes)?;
                        // deserialize from buffer
                        out_stream.write(str.as_ref())?;
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

fn main() -> Result<(), error::ClinvarXMLTabError> {
    let args = cli::Cli::parse();

    match args.command() {
        cli::Command::Convert(subparams) => convert(&args, subparams)?,
        cli::Command::Debug(subparams) => debug(&args, subparams)?,
    }

    Ok(())
}
