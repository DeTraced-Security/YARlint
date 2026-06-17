use std::fs;
use std::io::Write;
use tempfile::TempDir;
use yarlint::filesystem::collect_yara_files;

fn make_yar_file(dir: &TempDir, name: &str, content: &str) -> std::path::PathBuf {
    let path = dir.path().join(name);
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    path
}

// --- single file ---

#[test]
fn single_yara_file_is_collected() {
    let dir = TempDir::new().unwrap();
    let file = make_yar_file(&dir, "rule.yar", "rule foo { condition: true }");

    let result = collect_yara_files(file.to_str().unwrap(), false, None).unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0], file);
}

#[test]
fn single_non_yara_file_is_ignored() {
    let dir = TempDir::new().unwrap();
    let file = make_yar_file(&dir, "rule.txt", "not a yara file");

    let result = collect_yara_files(file.to_str().unwrap(), false, None).unwrap();

    assert!(result.is_empty());
}

#[test]
fn yara_extension_is_accepted() {
    let dir = TempDir::new().unwrap();
    let file = make_yar_file(&dir, "rule.yara", "rule foo { condition: true }");

    let result = collect_yara_files(file.to_str().unwrap(), false, None).unwrap();

    assert_eq!(result.len(), 1);
}

// --- directory, non-recursive ---

#[test]
fn directory_collects_yara_files_in_root() {
    let dir = TempDir::new().unwrap();
    make_yar_file(&dir, "rule_a.yar", "rule a { condition: true }");
    make_yar_file(&dir, "rule_b.yar", "rule b { condition: true }");

    let result = collect_yara_files(dir.path().to_str().unwrap(), false, None).unwrap();

    assert_eq!(result.len(), 2);
}

#[test]
fn directory_ignores_non_yara_files() {
    let dir = TempDir::new().unwrap();
    make_yar_file(&dir, "rule.yar", "rule foo { condition: true }");
    make_yar_file(&dir, "readme.txt", "not yara");

    let result = collect_yara_files(dir.path().to_str().unwrap(), false, None).unwrap();

    assert_eq!(result.len(), 1);
}

#[test]
fn non_recursive_does_not_descend_into_subdirectories() {
    let dir = TempDir::new().unwrap();
    make_yar_file(&dir, "root.yar", "rule root { condition: true }");
    let subdir = dir.path().join("sub");
    fs::create_dir(&subdir).unwrap();
    let mut f = fs::File::create(subdir.join("nested.yar")).unwrap();
    f.write_all(b"rule nested { condition: true }").unwrap();

    let result = collect_yara_files(dir.path().to_str().unwrap(), false, None).unwrap();

    assert_eq!(result.len(), 1);
    assert!(result[0].file_name().unwrap() == "root.yar");
}

// --- directory, recursive ---

#[test]
fn recursive_descends_into_subdirectories() {
    let dir = TempDir::new().unwrap();
    make_yar_file(&dir, "root.yar", "rule root { condition: true }");
    let subdir = dir.path().join("sub");
    fs::create_dir(&subdir).unwrap();
    let mut f = fs::File::create(subdir.join("nested.yar")).unwrap();
    f.write_all(b"rule nested { condition: true }").unwrap();

    let result = collect_yara_files(dir.path().to_str().unwrap(), true, None).unwrap();

    assert_eq!(result.len(), 2);
}

#[test]
fn recursive_with_depth_one_only_collects_root_files() {
    let dir = TempDir::new().unwrap();
    make_yar_file(&dir, "root.yar", "rule root { condition: true }");
    let subdir = dir.path().join("sub");
    fs::create_dir(&subdir).unwrap();
    let mut f = fs::File::create(subdir.join("nested.yar")).unwrap();
    f.write_all(b"rule nested { condition: true }").unwrap();

    let result = collect_yara_files(dir.path().to_str().unwrap(), true, Some(1)).unwrap();

    assert_eq!(result.len(), 1);
    assert!(result[0].file_name().unwrap() == "root.yar");
}

#[test]
fn recursive_with_depth_two_collects_one_level_deep() {
    let dir = TempDir::new().unwrap();
    make_yar_file(&dir, "root.yar", "rule root { condition: true }");
    let subdir = dir.path().join("sub");
    fs::create_dir(&subdir).unwrap();
    let mut f = fs::File::create(subdir.join("nested.yar")).unwrap();
    f.write_all(b"rule nested { condition: true }").unwrap();
    let deepdir = subdir.join("deep");
    fs::create_dir(&deepdir).unwrap();
    let mut f = fs::File::create(deepdir.join("deep.yar")).unwrap();
    f.write_all(b"rule deep { condition: true }").unwrap();

    let result = collect_yara_files(dir.path().to_str().unwrap(), true, Some(2)).unwrap();

    assert_eq!(result.len(), 2);
}

#[test]
fn empty_directory_returns_empty_vec() {
    let dir = TempDir::new().unwrap();

    let result = collect_yara_files(dir.path().to_str().unwrap(), false, None).unwrap();

    assert!(result.is_empty());
}

// --- error cases ---

#[test]
fn nonexistent_path_returns_err() {
    let result = collect_yara_files("/nonexistent/path/file.yar", false, None);

    assert!(result.is_err());
}

#[test]
fn error_message_mentions_path_does_not_exist() {
    let result = collect_yara_files("/nonexistent/path", false, None);

    assert!(result.unwrap_err().contains("does not exist"));
}
