use yarlint::linter::cop::Category;

use crate::load;

#[test]
fn accessors_read_the_configured_value() {
    let (resolved, _) = load("[cops.Naming.RuleNameLength]\nmax_name_length = 50\n");
    let settings = resolved.get(Category::Naming, "RuleNameLength").unwrap();
    let params = settings.params();

    assert_eq!(params.uint("max_name_length"), 50);
    assert_eq!(params.uint("min_name_length"), 4);
}

#[test]
fn enum_params_read_as_strings() {
    let (resolved, _) = load("[cops.Style.RuleNameCase]\ncase = \"SnakeCase\"\n");
    let settings = resolved.get(Category::Style, "RuleNameCase").unwrap();

    assert_eq!(settings.params().str("case"), "SnakeCase");
}

#[test]
fn globals_are_readable_after_an_override() {
    let (resolved, _) = load("[globals]\nrequired_meta_keys = [\"author\", \"date\"]\n");

    assert_eq!(resolved.globals.required_meta_keys, vec!["author", "date"]);
}

#[test]
fn boolean_accessors_read_configured_and_default_values() {
    let (configured, _) = load("[cops.Lint.Fixture]\nstrict = false\n");
    assert!(
        !configured
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params()
            .bool("strict")
    );

    let (defaulted, _) = load("");
    assert!(
        defaulted
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params()
            .bool("strict")
    );
}

#[test]
fn string_accessors_read_configured_and_default_values() {
    let (configured, _) = load("[cops.Lint.Fixture]\nprefix = \"yara_\"\n");
    assert_eq!(
        configured
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params()
            .str("prefix"),
        "yara_"
    );

    let (defaulted, _) = load("");
    assert_eq!(
        defaulted
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params()
            .str("prefix"),
        "rule_"
    );
}

#[test]
fn string_list_accessors_read_configured_and_default_values() {
    let (configured, _) = load("[cops.Lint.Fixture]\nignored_tags = [\"draft\"]\n");
    assert_eq!(
        configured
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params()
            .str_list("ignored_tags"),
        ["draft"]
    );

    let (defaulted, _) = load("");
    assert_eq!(
        defaulted
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params()
            .str_list("ignored_tags"),
        ["test", "wip"]
    );
}

#[test]
fn an_empty_list_reads_back_as_empty() {
    let (resolved, _) = load("[cops.Lint.Fixture]\nignored_tags = []\n");

    assert!(
        resolved
            .get(Category::Lint, "Fixture")
            .unwrap()
            .params()
            .str_list("ignored_tags")
            .is_empty()
    );
}

#[test]
fn accessors_survive_a_rejected_value_by_returning_the_default() {
    let (resolved, diagnostics) = load("[cops.Naming.RuleNameLength]\nmax_name_length = 9000\n");

    assert!(diagnostics.has_errors());
    assert_eq!(
        resolved
            .get(Category::Naming, "RuleNameLength")
            .unwrap()
            .params()
            .uint("max_name_length"),
        80
    );
}

#[test]
fn a_params_handle_is_cheap_to_copy() {
    let (resolved, _) = load("");
    let settings = resolved.get(Category::Lint, "Fixture").unwrap();
    let params = settings.params();
    let copied = params;

    assert_eq!(params.str("prefix"), copied.str("prefix"));
}
