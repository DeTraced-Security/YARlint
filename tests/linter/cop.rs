use yarlint::linter::context::LintContext;
use yarlint::linter::cop::{Category, Cop};
use yarlint::linter::finding::Finding;

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
}

#[test]
fn qualified_name_joins_category_and_name_with_a_slash() {
    let cop = MockCop {
        name: "WideStringWithoutAscii",
        category: Category::Performance,
    };

    assert_eq!(cop.qualified_name(), "Performance/WideStringWithoutAscii");
}

#[test]
fn qualified_name_uses_each_category_variants_display_form() {
    // Guards against Category's Display impl and qualified_name's
    // formatting drifting apart independently.
    let cases = [
        (Category::Lint, "Lint"),
        (Category::Logic, "Logic"),
        (Category::Naming, "Naming"),
        (Category::Performance, "Performance"),
        (Category::Style, "Style"),
    ];

    for (category, expected_prefix) in cases {
        let cop = MockCop {
            name: "Whatever",
            category,
        };
        assert_eq!(cop.qualified_name(), format!("{expected_prefix}/Whatever"));
    }
}
