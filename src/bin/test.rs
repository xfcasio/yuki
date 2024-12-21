use std::collections::BTreeMap;
use yuki::*;

fn main() {
    (Package {
        name: "testpkg",
        version: "0.0.1",

        source: Source::Literal(source_map! {
            ".secondrc" => "~/.secondrc",
            "firstrc" => "~/.firce.rc",
            "init.lua" => "~/.config/nvim/init.lua"
        })
    }).generate();
}
