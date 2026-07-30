use yarlint::linter::{
    context::LintContext,
    cop::{Category, Cop},
    cops::selector::{ConflictingSelectorError, CopSelector, resolve_enabled_cops},
    finding::Finding,
};

struct MockCop {
    name: &'static str,
    category: Category,
}

impl Cop for MockCop {
    fn name(&self) -> &'static str {
        self.name
    }

    fn category(&self) -> Category {
        self.category
    }

    fn check(&self, _context: &LintContext, _findings: &mut Vec<Finding>) {}

    fn qualified_name(&self) -> String {
        format!("{}/{}", self.category(), self.name())
    }
}

fn mock(name: &'static str, category: Category) -> MockCop {
    MockCop { name, category }
}

fn qualified_names(cops: &[&dyn Cop]) -> Vec<String> {
    let mut names: Vec<String> = cops.iter().map(|c| c.qualified_name()).collect();
    names.sort();
    names
}

#[test]
fn empty_only_and_except_enables_everything() {
    let a = mock("RuleName", Category::Naming);
    let b = mock("WideStringWithoutAscii", Category::Performance);
    let all: Vec<&dyn Cop> = vec![&a, &b];

    let enabled = resolve_enabled_cops(&all, &[], &[]).unwrap();

    assert_eq!(
        qualified_names(&enabled),
        vec!["Naming/RuleName", "Performance/WideStringWithoutAscii"]
    );
}

#[test]
fn only_category_selects_just_that_category() {
    let a = mock("RuleName", Category::Naming);
    let b = mock("WideStringWithoutAscii", Category::Performance);
    let all: Vec<&dyn Cop> = vec![&a, &b];

    let only = vec![CopSelector::Category(Category::Naming)];
    let enabled = resolve_enabled_cops(&all, &only, &[]).unwrap();

    assert_eq!(qualified_names(&enabled), vec!["Naming/RuleName"]);
}

#[test]
fn except_category_removes_just_that_category() {
    let a = mock("RuleName", Category::Naming);
    let b = mock("WideStringWithoutAscii", Category::Performance);
    let all: Vec<&dyn Cop> = vec![&a, &b];

    let except = vec![CopSelector::Category(Category::Performance)];
    let enabled = resolve_enabled_cops(&all, &[], &except).unwrap();

    assert_eq!(qualified_names(&enabled), vec!["Naming/RuleName"]);
}

#[test]
fn only_selecting_nothing_present_yields_empty_set() {
    let a = mock("RuleName", Category::Naming);
    let all: Vec<&dyn Cop> = vec![&a];

    let only = vec![CopSelector::Category(Category::Performance)];
    let enabled = resolve_enabled_cops(&all, &only, &[]).unwrap();

    assert!(enabled.is_empty());
}

#[test]
fn except_specific_cop_trims_out_of_a_category_wide_only() {
    // --only Naming --except Naming/RuleName -> everything in Naming
    // except that one cop.
    let a = mock("RuleName", Category::Naming);
    let b = mock("RuleNameLength", Category::Naming);
    let c = mock("WideStringWithoutAscii", Category::Performance);
    let all: Vec<&dyn Cop> = vec![&a, &b, &c];

    let only = vec![CopSelector::Category(Category::Naming)];
    let except = vec![CopSelector::Cop("Naming/RuleName".to_string())];
    let enabled = resolve_enabled_cops(&all, &only, &except).unwrap();

    assert_eq!(qualified_names(&enabled), vec!["Naming/RuleNameLength"]);
}

#[test]
fn only_specific_cop_survives_a_category_wide_except() {
    // --except Naming --only Naming/RuleName -> the specific cop wins
    // despite its whole category being excluded. This is the precedence
    // rule that inverts the naive "except always trims" behavior.
    let a = mock("RuleName", Category::Naming);
    let b = mock("RuleNameLength", Category::Naming);
    let all: Vec<&dyn Cop> = vec![&a, &b];

    let except = vec![CopSelector::Category(Category::Naming)];
    let only = vec![CopSelector::Cop("Naming/RuleName".to_string())];
    let enabled = resolve_enabled_cops(&all, &only, &except).unwrap();

    assert_eq!(qualified_names(&enabled), vec!["Naming/RuleName"]);
}

#[test]
fn same_specific_cop_in_only_and_except_is_a_conflict_error() {
    let a = mock("RuleName", Category::Naming);
    let all: Vec<&dyn Cop> = vec![&a];

    let only = vec![CopSelector::Cop("Naming/RuleName".to_string())];
    let except = vec![CopSelector::Cop("Naming/RuleName".to_string())];

    match resolve_enabled_cops(&all, &only, &except) {
        Err(ConflictingSelectorError(name)) => assert_eq!(name, "Naming/RuleName"),
        Ok(_) => panic!("expected a conflict error, but resolution succeeded"),
    }
}

#[test]
fn same_category_in_only_and_except_resolves_to_empty_without_erroring() {
    // Unlike the specific-cop case, a category named in both lists isn't
    // treated as a conflict — it just resolves to nothing, since neither
    // selector is more specific than the other. Documenting this as
    // current behavior rather than asserting it's necessarily the ideal
    // UX; worth a warning at the CLI layer if the resolved set ends up
    // empty while `only` was non-empty.
    let a = mock("RuleName", Category::Naming);
    let all: Vec<&dyn Cop> = vec![&a];

    let selector = vec![CopSelector::Category(Category::Naming)];
    let enabled = resolve_enabled_cops(&all, &selector, &selector).unwrap();

    assert!(enabled.is_empty());
}

#[test]
fn cop_selector_parses_a_known_category() {
    assert_eq!(
        "Naming".parse::<CopSelector>(),
        Ok(CopSelector::Category(Category::Naming))
    );
}

#[test]
fn cop_selector_parses_a_fully_qualified_cop_name() {
    assert_eq!(
        "Performance/WideStringWithoutAscii".parse::<CopSelector>(),
        Ok(CopSelector::Cop(
            "Performance/WideStringWithoutAscii".to_string()
        ))
    );
}

#[test]
fn cop_selector_rejects_a_bare_unknown_word() {
    // No slash and not a recognized category name -> parse error.
    assert!("Bogus".parse::<CopSelector>().is_err());
}

#[test]
fn cop_selector_does_not_validate_specific_cop_existence_at_parse_time() {
    // A slash is sufficient to parse as CopSelector::Cop, even for a
    // cop name that doesn't exist in any registry — existence is only
    // implicitly enforced later, by the fact that it'll just never
    // match anything in resolve_enabled_cops. Documenting this as a
    // deliberate deferral, not an oversight.
    assert_eq!(
        "Category/DoesNotExist".parse::<CopSelector>(),
        Ok(CopSelector::Cop("Category/DoesNotExist".to_string()))
    );
}
