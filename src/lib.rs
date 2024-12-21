use std::collections::BTreeMap;
mod generate_manifest;
mod generate;

#[macro_export]
macro_rules! source_map {
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut sources: BTreeMap<String, String> = BTreeMap::new();
            $( sources.insert($key.to_owned(), $value.to_owned()); )*
            sources
        }
    }
}

pub struct Package {
    pub name: &'static str,
    pub version: &'static str,
    pub source: Source
}

pub enum Source {
    Yuki,
    Literal(BTreeMap<String, String>),
    FromGit(String)
}
