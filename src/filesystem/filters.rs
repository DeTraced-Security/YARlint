use std::path::Path;

pub fn is_yara_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("yar") | Some("yara") => true,
        _ => false,
    }
}