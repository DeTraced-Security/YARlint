#[test]
fn lint_engine_default_produces_empty_engine() {
    use yarlint::linter::engine::LintEngine;
    let engine = LintEngine::default();
    // default() calls new() - verify it runs without panic
    let _ = engine;
}
