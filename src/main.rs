use std::io::Write;

use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Reader, Writer,
};

fn main() -> quick_xml::Result<()> {
    let mut reader = Reader::from_reader(std::io::BufReader::new(std::io::stdin()));
    reader.trim_text(true);

    let mut writer = Writer::new_with_indent(std::io::BufWriter::new(std::io::stdout()), b' ', 2);

    let mut buf = Vec::new();

    loop {
        buf.clear();
        let event = reader.read_event_into(&mut buf)?;
        match event {
            Event::Decl(decl) => {
                writer.write_event(quick_xml::events::Event::Decl(decl))?;
            }
            Event::Start(tag) => {
                let new_start = BytesStart::new("element")
                    .with_attributes([(&b"tag"[..], tag.name().into_inner())]);
                writer.write_event(Event::Start(new_start))?;
                for attr in tag.attributes() {
                    let attr = attr?;
                    let element = BytesStart::new("attribute").with_attributes([
                        (&b"name"[..], attr.key.into_inner()),
                        (&b"value"[..], attr.value.as_ref()),
                    ]);
                    writer.write_event(Event::Empty(element))?;
                }
                writer.write_event(Event::Start(BytesStart::new("children")))?;
            }
            Event::End(_) => {
                writer.write_event(Event::End(BytesEnd::new("children")))?;
                writer.write_event(Event::End(BytesEnd::new("element")))?;
            }
            quick_xml::events::Event::Text(text) => {
                writer.write_event(Event::Text(text))?;
            }
            quick_xml::events::Event::Empty(tag) => {
                let new_start = BytesStart::new("element")
                    .with_attributes([(&b"tag"[..], tag.name().into_inner())]);

                if tag.attributes().next().is_some() {
                    writer.write_event(Event::Start(new_start))?;
                    for attr in tag.attributes() {
                        let attr = attr?;
                        let element = BytesStart::new("attribute").with_attributes([
                            (&b"name"[..], attr.key.into_inner()),
                            (&b"value"[..], attr.value.as_ref()),
                        ]);
                        writer.write_event(Event::Empty(element))?;
                    }
                    writer.write_event(Event::End(BytesEnd::new("element")))?;
                } else {
                    writer.write_event(Event::Empty(new_start))?;
                }
            }
            quick_xml::events::Event::Eof => break,
            Event::CData(_) | Event::Comment(_) | Event::PI(_) | Event::DocType(_) => {}
        }
    }

    writer.into_inner().flush()?;

    Ok(())
}
