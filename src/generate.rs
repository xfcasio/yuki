use crate::{ Package, Source };

impl Package {
    pub fn generate(self) {
        // TODO(cisco): validate that all instances of `Content::File` are existing locations

        self.generate_manifest();
        
        match &self.source {
            Source::Literal(sources) => {
                std::fs::create_dir(self.name).unwrap_or_else(|_| eprintln!("? using existing {}/ directory", self.name));
                std::fs::rename("yuki.xml", format!("{}/yuki.xml", self.name)).unwrap();

                for (source_path, _) in sources {
                    std::fs::rename(source_path, format!("{}/{}", self.name, source_path))
                        .unwrap_or_else(|_| panic!("move ('{source_path}' -> '{}/{source_path}') failed", self.name));
                }

                sevenz_rust::compress_to_path(self.name, format!("{}.7z", self.name)).expect("PACKAGE_GENERATION::COMPRESSION_FAIL");
            }

            _ => unreachable!()
        }
    }
}

