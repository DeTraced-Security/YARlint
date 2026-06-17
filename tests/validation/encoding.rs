use yarlint::validation::encoding::validate_encoding;

#[test]
fn valid_utf8_returns_true() {
    let input = "rule foo { condition: true }".as_bytes();

    let result = validate_encoding(input);

    assert_eq!(result, Ok(true));
}

#[test]
fn empty_input_returns_true() {
    let result = validate_encoding(b"");

    assert_eq!(result, Ok(true));
}

#[test]
fn valid_utf8_with_multibyte_characters_returns_true() {
    // ASCII is a strict subset of UTF-8 but so are these
    let input = "// comment with émojis 🦀".as_bytes();

    let result = validate_encoding(input);

    assert_eq!(result, Ok(true));
}

#[test]
fn invalid_utf8_sequence_returns_false() {
    // lone continuation byte - invalid UTF-8
    let input = b"valid\xFF\xFEcontent";

    let result = validate_encoding(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn truncated_multibyte_sequence_returns_false() {
    // start of a 2-byte sequence with no continuation byte
    let input = b"valid\xC3content";

    let result = validate_encoding(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn overlong_encoding_returns_false() {
    // overlong encoding of '/' - invalid UTF-8
    let input = b"\xAF\xC0";

    let result = validate_encoding(input);

    assert_eq!(result, Ok(false));
}
