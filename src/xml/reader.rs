use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, Writer};
use std::collections::HashMap;
use std::io::BufRead;

use crate::error::ClinvarXMLTabError;
use crate::handler;

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

fn note_flatten_treat(
    node: &roxmltree::Node,
    current_path: &mut Vec<String>,
    handler: &mut impl handler::EventHandler,
    depth: u32,
) -> Result<(), ClinvarXMLTabError> {
    if node.is_element() {
        current_path.push(node.tag_name().name().to_string());
        let attributes: HashMap<String, String> = node
            .attributes()
            .map(|att| (att.name().to_string(), att.value().to_string()))
            .collect();
        handler.handle(node, current_path, &attributes, depth)?;
        for child in node.children() {
            note_flatten_treat(&child, current_path, handler, depth + 1)?;
        }
        current_path.pop();
    } else {
        for child in node.children() {
            note_flatten_treat(&child, current_path, handler, depth + 1)?;
        }
    }
    Ok(())
}

pub fn read_xml(
    reader: impl std::io::BufRead,
    handler: &mut impl handler::EventHandler,
    limit: Option<u64>,
) -> Result<(), ClinvarXMLTabError> {
    let mut reader = Reader::from_reader(reader);

    let mut count = 0;

    let mut buf = Vec::new();
    let mut junk_buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => {
                break;
            }
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"ClinVarSet" => {
                    // load entire tag into buffer
                    let elem_bytes = read_to_end_into_buffer(&mut reader, &e, &mut junk_buf)?;

                    // out_stream.write(str.as_ref()).ok()?;

                    let mut current_path = Vec::new();

                    let str = std::str::from_utf8(&elem_bytes)?.to_string();
                    let doc = roxmltree::Document::parse(&str)?;
                    note_flatten_treat(&doc.root(), &mut current_path, handler, 0)?;
                    count += 1;
                    if let Some(limit) = limit {
                        if count >= limit {
                            break;
                        }
                    }
                }
                _ => {}
            },
            // Other Events are not important for us
            _ => (),
        }
        buf.clear();
    }
    Ok(())
}
