use std::path::Path;
use yarlint::filesystem::is_yara_file;

// --- accepted extensions ---

#[test]
fn yar_extension_returns_true() {
    assert!(is_yara_file(Path::new("rule.yar")));
}

#[test]
fn yara_extension_returns_true() {
    assert!(is_yara_file(Path::new("rule.yara")));
}

#[test]
fn yar_extension_with_path_prefix_returns_true() {
    assert!(is_yara_file(Path::new("/some/path/rule.yar")));
}

#[test]
fn yara_extension_with_path_prefix_returns_true() {
    assert!(is_yara_file(Path::new("/some/path/rule.yara")));
}

// --- rejected extensions ---

#[test]
fn txt_extension_returns_false() {
    assert!(!is_yara_file(Path::new("rule.txt")));
}

#[test]
fn rs_extension_returns_false() {
    assert!(!is_yara_file(Path::new("rule.rs")));
}

#[test]
fn no_extension_returns_false() {
    assert!(!is_yara_file(Path::new("rule")));
}

#[test]
fn empty_path_returns_false() {
    assert!(!is_yara_file(Path::new("")));
}

#[test]
fn yar_uppercase_extension_returns_false() {
    // extension matching is case-sensitive
    assert!(!is_yara_file(Path::new("rule.YAR")));
}

#[test]
fn yara_uppercase_extension_returns_false() {
    assert!(!is_yara_file(Path::new("rule.YARA")));
}

#[test]
fn partial_yar_extension_returns_false() {
    assert!(!is_yara_file(Path::new("rule.ya")));
}
