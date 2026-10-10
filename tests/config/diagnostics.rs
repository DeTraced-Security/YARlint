use yarlint::config::diagnostics::{did_you_mean, line_col};

#[test]
fn line_col_is_one_based() {
    assert_eq!(line_col("abc", 0), (1, 1));
    assert_eq!(line_col("abc", 2), (1, 3));
}

#[test]
fn line_col_counts_newlines() {
    let text = "one\ntwo\nthree";
    assert_eq!(line_col(text, 4), (2, 1));
    assert_eq!(line_col(text, 8), (3, 1));
    assert_eq!(line_col(text, 10), (3, 3));
}

#[test]
fn line_col_counts_characters_not_bytes() {
    let text = "héllo = 1";
    // `é` is two bytes, so byte offset 6 is character 6 on line 1.
    assert_eq!(line_col(text, 6), (1, 6));
}

#[test]
fn line_col_clamps_past_the_end() {
    assert_eq!(line_col("ab", 99), (1, 3));
}

#[test]
fn a_distant_candidate_is_not_suggested() {
    assert_eq!(did_you_mean("severity", ["cops", "globals"]), None);
}

#[test]
fn a_near_candidate_is_suggested() {
    assert_eq!(did_you_mean("globls", ["cops", "globals"]), Some("globals"));
}

#[test]
fn the_closest_candidate_wins() {
    assert_eq!(
        did_you_mean("max_name_lenght", ["min_name_length", "max_name_length"]),
        Some("max_name_length")
    );
}

#[test]
fn short_inputs_demand_a_near_exact_match() {
    // Threshold is 1 for inputs of three characters or fewer.
    assert_eq!(did_you_mean("abc", ["abd"]), Some("abd"));
    assert_eq!(did_you_mean("abc", ["axy"]), None);
}

#[test]
fn an_abbreviation_is_treated_as_a_near_miss() {
    // Plain edit distance puts these three apart, past the threshold.
    assert_eq!(
        did_you_mean("warn", ["info", "warning", "error"]),
        Some("warning")
    );
}

#[test]
fn a_prefix_only_counts_when_the_gap_is_small() {
    // A bare category must not beat the qualified cop the user meant.
    assert_eq!(
        did_you_mean(
            "Naming/RuleNameLenght",
            ["Naming", "Naming/RuleNameLength", "Style"]
        ),
        Some("Naming/RuleNameLength")
    );
}

#[test]
fn an_empty_input_suggests_nothing_useful() {
    assert_eq!(did_you_mean("", ["warning"]), None);
}

#[test]
fn an_empty_candidate_is_handled() {
    assert_eq!(did_you_mean("ab", [""]), None);
}

#[test]
fn no_candidates_means_no_suggestion() {
    assert_eq!(did_you_mean("anything", std::iter::empty()), None);
}
