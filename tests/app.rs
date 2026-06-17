use yarlint::app::yarlint_pipeline;
use yarlint::cli::Args;

fn make_args(path: &str) -> Args {
    Args {
        path: path.to_string(),
        recursive: false,
        depth: None,
        verbose: false,
    }
}

#[test]
fn pipeline_succeeds_on_valid_fixture() {
    let args = make_args("tests/fixtures/good_rule.yar");

    let result = yarlint_pipeline(&args);

    assert!(result.is_ok());
}

#[test]
fn pipeline_succeeds_on_rule_with_findings() {
    // findings are printed but don't cause pipeline failure
    let args = make_args("tests/fixtures/bad_rule_name.yar");

    let result = yarlint_pipeline(&args);

    assert!(result.is_ok());
}

#[test]
fn pipeline_returns_err_on_nonexistent_path() {
    let args = make_args("/nonexistent/path/file.yar");

    let result = yarlint_pipeline(&args);

    assert!(result.is_err());
}

#[test]
fn pipeline_succeeds_with_recursive_flag_on_fixture_dir() {
    let args = Args {
        path: "tests/fixtures".to_string(),
        recursive: true,
        depth: None,
        verbose: false,
    };

    let result = yarlint_pipeline(&args);

    assert!(result.is_ok());
}

#[test]
fn pipeline_succeeds_with_depth_limit() {
    let args = Args {
        path: "tests/fixtures".to_string(),
        recursive: true,
        depth: Some(1),
        verbose: false,
    };

    let result = yarlint_pipeline(&args);

    assert!(result.is_ok());
}