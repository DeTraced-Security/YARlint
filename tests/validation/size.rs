use std::io::Write;
use yarlint::validation::size::validate_size;

#[test]
fn small_file_returns_true() {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(b"rule foo { condition: true }").unwrap();

    let result = validate_size(&file.path().to_path_buf());

    assert_eq!(result, Ok(true));
}

#[test]
fn empty_file_returns_true() {
    let file = tempfile::NamedTempFile::new().unwrap();

    let result = validate_size(&file.path().to_path_buf());

    assert_eq!(result, Ok(true));
}

#[test]
fn nonexistent_file_returns_err() {
    let path = std::path::PathBuf::from("/nonexistent/path/file.yar");

    let result = validate_size(&path);

    assert!(result.is_err());
}

#[test]
fn oversized_file_returns_false() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("large.yar");

    let file = std::fs::File::create(&path).unwrap();
    const GIGABYTE_SIZE: u64 = 1073741824;
    file.set_len(GIGABYTE_SIZE + 1).unwrap();

    assert!(!validate_size(&path).unwrap());
}
