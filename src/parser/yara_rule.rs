use std::collections::HashMap;
use std::path::PathBuf;

const KEYWORDS: &[&str] = &[
    "all", "and", "any", "ascii", "at", "base64", "base64wide",
    "condition", "contains", "endswith", "entrypoint", "false",
    "filesize", "for", "fullword", "global", "import", "icontains",
    "iendswith", "iequals", "in", "include", "int16", "int16be",
    "int32", "int32be", "int8", "int8be", "istartswith", "matches",
    "meta", "nocase", "none", "not", "of", "or", "private", "rule",
    "startswith", "strings", "them", "true", "uint16", "uint16be",
    "uint32", "uint32be", "uint8", "uint8be", "wide", "xor",
    "defined"
];

pub struct YaraRule {
    pub name: String,
    pub path: PathBuf,
    pub meta: HashMap<String, String>,
    pub strings: HashMap<String, String>,
}

impl YaraRule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            path: PathBuf::new(),
            meta: HashMap::new(),
            strings: HashMap::new(),
        }
    }
}