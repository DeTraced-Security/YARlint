// --- exit codes ---

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
fn exits_zero_on_rule_with_findings() {
    // findings are warnings, not fatal — exit code should still be zero
    let mut cmd = assert_cmd::Command::cargo_bin("yarlint").unwrap();
    cmd.arg("--path").arg("tests/fixtures/bad_rule_name.yar");

    cmd.assert().success();
}

#[test]
fn recursive_flag_exits_zero_on_fixture_dir() {
    let mut cmd = assert_cmd::Command::cargo_bin("yarlint").unwrap();
    cmd.arg("--path").arg("tests/fixtures").arg("--recursive");

    cmd.assert().success();
}

#[test]
fn verbose_flag_exits_zero() {
    let mut cmd = assert_cmd::Command::cargo_bin("yarlint").unwrap();
    cmd.arg("--path")
        .arg("tests/fixtures/good_rule.yar")
        .arg("--verbose");

    cmd.assert().success();
}

// --- return values ---

#[test]
fn returns_version() {
    let version = env!("CARGO_PKG_VERSION");
    let mut cmd = assert_cmd::Command::cargo_bin("yarlint").unwrap();
    cmd.arg("-V");
    cmd.assert().stdout(predicates::str::contains(version));
}
