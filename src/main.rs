use clap::Parser;

pub mod cli;
pub mod error;
pub mod utils;

use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};

use std::io::BufRead;

pub mod clinvar_set_serde;

use cli::Genome;

use clinvar_set_serde::ClinVarSet;

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

fn csv_write_header(out_stream: &mut dyn std::io::Write) -> Result<(), error::ClinvarXMLTabError> {
    out_stream.write("CHROM\tPOS\tREF\tALT\tCLNACC\tCLNSIG\n".as_bytes())?;
    Ok(())
}

fn csv_write_record(
    out_stream: &mut Box<dyn std::io::Write + Send>,
    record: Option<&ClinVarSet>,
    genome: &Genome,
) -> Result<(), error::ClinvarXMLTabError> {
    if let Some(rec) = record {
        if let Some(rec_repr) = rec.get_repr(genome) {
            out_stream.write(rec_repr.as_bytes())?;
            return Ok(());
        }
    }
    Err(error::ClinvarXMLTabError::NoRecord)
}

fn main() -> Result<(), error::ClinvarXMLTabError> {
    let args = cli::Command::parse();

    let in_stream = utils::file_reader(args.input())?;
    let mut out_stream = utils::file_writer(args.output())?;

    let mut reader = Reader::from_reader(in_stream);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut junk_buf = Vec::new();

    let mut total_counter = 0;
    let mut ignored_counter = 0;

    csv_write_header(&mut out_stream)?;

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                match e.name().as_ref() {
                    b"ClinVarSet" => {
                        // load entire tag into buffer
                        let clinvar_set_bytes =
                            read_to_end_into_buffer(&mut reader, &e, &mut junk_buf)
                                .expect("Unrecoverable error while reading XML file!");
                        if let Ok(str) = std::str::from_utf8(&clinvar_set_bytes) {
                            // deserialize from buffer
                            let clinvar_set: Result<ClinVarSet, error::ClinvarXMLTabError> =
                                clinvar_set_serde::ClinVarSet::new_from_str(str);
                            if let Ok(_) = csv_write_record(
                                &mut out_stream,
                                clinvar_set.ok().as_ref(),
                                &args.genome(),
                            ) {
                                total_counter += 1;
                            } else {
                                ignored_counter += 1;
                            }
                        } else {
                            ignored_counter += 1;
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
    eprintln!(
        "Ignored {} entries out of {} (roughly {}%)",
        ignored_counter,
        total_counter,
        ignored_counter * 100 / total_counter
    );

    Ok(())
}
