use yarlint::validation::bytes::validate_bytes;

// --- good content ---

#[test]
fn valid_ascii_content_returns_true() {
    let input = b"rule foo { condition: true }";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(true));
}

#[test]
fn empty_input_returns_true() {
    let result = validate_bytes(b"");

    assert_eq!(result, Ok(true));
}

#[test]
fn allowed_control_chars_return_true() {
    // \n \r \t are explicitly permitted
    let input = b"line one\nline two\r\n\ttabbed";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(true));
}

// --- null bytes ---

#[test]
fn null_byte_returns_false() {
    let input = b"valid\x00content";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn null_byte_at_start_returns_false() {
    let input = b"\x00rule foo { condition: true }";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn null_byte_at_end_returns_false() {
    let input = b"rule foo { condition: true }\x00";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

// --- control characters ---

#[test]
fn start_of_heading_returns_false() {
    let input = b"valid\x01content";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn bell_character_returns_false() {
    let input = b"valid\x07content";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn backspace_returns_false() {
    let input = b"valid\x08content";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn vertical_tab_returns_false() {
    // \x0B is NOT in the allowed set unlike \t \n \r
    let input = b"valid\x0Bcontent";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn form_feed_returns_false() {
    let input = b"valid\x0Ccontent";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn escape_character_returns_false() {
    let input = b"valid\x1Bcontent";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn unit_separator_returns_false() {
    let input = b"valid\x1Fcontent";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

// --- non-UTF-8 ---

#[test]
fn invalid_utf8_returns_false() {
    // lone continuation byte — valid ASCII range but invalid UTF-8 sequence
    let input = b"valid\xFF\xFEcontent";

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

// --- bidi control characters ---

#[test]
fn left_to_right_embedding_returns_false() {
    // U+202A — used in trojan source attacks
    let input = "valid\u{202A}content".as_bytes();

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn right_to_left_embedding_returns_false() {
    // U+202B
    let input = "valid\u{202B}content".as_bytes();

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn pop_directional_formatting_returns_false() {
    // U+202C
    let input = "valid\u{202C}content".as_bytes();

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn left_to_right_override_returns_false() {
    // U+202D
    let input = "valid\u{202D}content".as_bytes();

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn right_to_left_override_returns_false() {
    // U+202E — classic trojan source character
    let input = "valid\u{202E}content".as_bytes();

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn left_to_right_isolate_returns_false() {
    // U+2066
    let input = "valid\u{2066}content".as_bytes();

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn right_to_left_isolate_returns_false() {
    // U+2067
    let input = "valid\u{2067}content".as_bytes();

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn first_strong_isolate_returns_false() {
    // U+2068
    let input = "valid\u{2068}content".as_bytes();

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}

#[test]
fn pop_directional_isolate_returns_false() {
    // U+2069
    let input = "valid\u{2069}content".as_bytes();

    let result = validate_bytes(input);

    assert_eq!(result, Ok(false));
}
