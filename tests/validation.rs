use std::io::Write;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use yarlint::validation::validate_files;

#[path = "validation/bytes.rs"]
pub mod bytes;

#[path = "validation/encoding.rs"]
pub mod encoding;

#[path = "validation/size.rs"]
pub mod size;

fn temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    std::env::temp_dir().join(format!("yarlint_{}_{}", name, nanos))
}

#[test]
fn validate_files_returns_empty_vec_when_given_no_files() {
    let result = validate_files(&Vec::new()).unwrap();

    assert!(result.is_empty());
}

#[test]
fn validate_files_returns_error_for_missing_file() {
    let path = temp_path("missing");

    let result = validate_files(&vec![path]);

    assert!(result.is_err());
}

#[test]
fn validate_files_returns_valid_file() {
    let path = temp_path("valid");

    {
        let mut file = fs::File::create(&path).unwrap();
        writeln!(file, "rule test {{ condition: true }}").unwrap();
    }

    let result = validate_files(&vec![path.clone()]).unwrap();

    assert_eq!(result, vec![path.clone()]);

    fs::remove_file(path).unwrap();
}

#[test]
fn validate_files_filters_invalid_files() {
    let valid = temp_path("valid");
    let invalid = temp_path("invalid");

    {
        let mut file = fs::File::create(&valid).unwrap();
        writeln!(file, "rule test {{ condition: true }}").unwrap();
    }

    // Invalid UTF-8
    fs::write(&invalid, [0xFF]).unwrap();

    let result = validate_files(&vec![valid.clone(), invalid.clone()]).unwrap();

    assert_eq!(result, vec![valid.clone()]);

    fs::remove_file(valid).unwrap();
    fs::remove_file(invalid).unwrap();
}
