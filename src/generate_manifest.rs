use quick_xml::{ Writer, { events::{ Event, BytesEnd, BytesStart, BytesText } } };
use std::io::Cursor;

use crate::{ Package, Source };

impl Package {
    pub(crate) fn generate_manifest(&self) {
        let handle_writer_error = |e| { panic!("XML_SERIALIZATION_ERROR: {e}") };

        let mut w = Writer::new(Cursor::new(vec![0u8; 0]));

        let mut package_start = BytesStart::new("package");
        package_start.push_attribute(("name", self.name));
        package_start.push_attribute(("version", self.version));
        let package_end = BytesEnd::new("package");

        w.write_event(Event::Start(package_start)).unwrap_or_else(handle_writer_error);
        w.write_event(Event::Text(BytesText::new("\n"))).unwrap_or_else(handle_writer_error);

        match &self.source {
            Source::Literal(sources) => {
                for (file, path) in sources {
                    let mut source_start = BytesStart::new("source");
                    source_start.push_attribute(("dst", path.as_str()));
                    source_start.push_attribute(("src", file.as_str()));

                    w.write_event(Event::Text(BytesText::new("\t\t"))).unwrap_or_else(handle_writer_error);
                    w.write_event(Event::Empty(source_start)).unwrap_or_else(handle_writer_error);
                    w.write_event(Event::Text(BytesText::new("\n"))).unwrap_or_else(handle_writer_error);
                }
            },

            _ => unreachable!()
        }

        w.write_event(Event::End(package_end)).unwrap_or_else(handle_writer_error);

        let serialized_xml = w.into_inner().into_inner();
        std::fs::write("yuki.xml", serialized_xml).expect("XML_WRITE_FAILURE")
    }
}
