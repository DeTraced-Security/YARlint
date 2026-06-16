#[test]
fn exits_nonzero_on_missing_path() {
    let mut cmd = assert_cmd::Command::cargo_bin("yarlint").unwrap();
    cmd.arg("--path").arg("/nonexistent");
    cmd.assert().failure();
}

#[test]
fn exits_zero_on_valid_rule() {
    let mut cmd = assert_cmd::Command::cargo_bin("yarlint").unwrap();
    cmd.arg("--path").arg("tests/fixtures/good_rule.yar");
    cmd.assert().success();
}

#[test]
fn returns_version() {
    let version = env!("CARGO_PKG_VERSION");
    let mut cmd = assert_cmd::Command::cargo_bin("yarlint").unwrap();
    cmd.arg("-V");
    cmd.assert().stdout(predicates::str::contains(version));
}
