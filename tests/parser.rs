use std::path::PathBuf;

use yarlint::parser::parse_files;

#[path = "parser/yara.rs"]
pub mod yara;

#[path = "parser/hex.rs"]
pub mod hex;

#[test]
fn parser_error_on_file_not_existing() {
    let bad_path_bug = PathBuf::from("not_a_file");
    let files = vec![bad_path_bug];
    let result = parse_files(&files);

    assert!(result.is_err())
}
