//! JC-06 to JC-10: strings, numbers, nesting, duplicate keys and the encoder.
//!
//! The package returns the canonical encoding of what it parsed, so an expected
//! value is written as the text the encoder must produce for it. Every refusal
//! the parser or encoder can return is provoked here with its offset (R5).

use json_oracle::{encode_probe, lookup_first, parse, parse_ungated, Parse, Probe, Refusal};

/// The canonical text of a parse that must succeed.
fn canonical(input: &[u8]) -> Vec<u8> {
    match parse(input) {
        Parse::Value(bytes) => bytes,
        other => panic!(
            "{:?} was not accepted: {other:?}",
            String::from_utf8_lossy(input)
        ),
    }
}

/// The refusal of a parse that must be refused.
fn refusal(input: &[u8]) -> (Refusal, u64) {
    match parse(input) {
        Parse::Refused(d) => (d.kind, d.offset),
        other => panic!(
            "{:?} was not refused: {other:?}",
            String::from_utf8_lossy(input)
        ),
    }
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).expect("canonical text is UTF-8")
}

#[test]
fn the_three_literals_and_whitespace_are_read_jc_03() {
    assert_eq!(text(&canonical(b"null")), "null");
    assert_eq!(text(&canonical(b" \t\r\n true \n")), "true");
    assert_eq!(text(&canonical(b"false")), "false");
    assert_eq!(text(&canonical(b"[ ]")), "[]");
    assert_eq!(text(&canonical(b"{\n}")), "{}");
    assert_eq!(text(&canonical(b" [ 1 , 2 ,3 ] ")), "[1,2,3]");
    assert_eq!(
        text(&canonical(b"{ \"a\" : [ ] , \"b\" : { } }")),
        "{\"a\":[],\"b\":{}}"
    );
    // A byte order mark is not whitespace and not part of the grammar.
    assert_eq!(refusal(b"\xEF\xBB\xBF{}"), (Refusal::UnexpectedByte, 0));
    // Form feed and vertical tab are not JSON whitespace.
    assert_eq!(refusal(b"\x0C[]"), (Refusal::UnexpectedByte, 0));
}

#[test]
fn string_escapes_decode_and_encode_jc_06() {
    assert_eq!(
        text(&canonical(br#""a\"b\\c\/d\b\f\n\r\t""#)),
        "\"a\\\"b\\\\c/d\\b\\f\\n\\r\\t\""
    );
    assert_eq!(
        text(&canonical(br#""\u0041\u00e9\u20AC""#)),
        "\"A\u{e9}\u{20ac}\""
    );
    // The control characters without a short escape are written \u00xx.
    assert_eq!(
        text(&canonical(br#""\u0000\u001f\u007f""#)),
        "\"\\u0000\\u001f\u{7f}\""
    );
    assert_eq!(
        text(&canonical("\"\u{e9}\u{1F600}\"".as_bytes())),
        "\"\u{e9}\u{1F600}\""
    );
    // A key is a string with the same escapes.
    assert_eq!(text(&canonical(br#"{"a\n":0}"#)), "{\"a\\n\":0}");
}

#[test]
fn a_surrogate_pair_is_one_code_point_jc_06() {
    assert_eq!(text(&canonical(br#""\uD83D\uDE00""#)), "\"\u{1F600}\"");
    assert_eq!(text(&canonical(br#""\ud800\udc00""#)), "\"\u{10000}\"");
    assert_eq!(text(&canonical(br#""\uDBFF\uDFFF""#)), "\"\u{10FFFF}\"");
}

#[test]
fn a_lone_surrogate_is_refused_as_lone_surrogate_jc_06() {
    // The documented choice: RFC 8259 section 8.2 leaves a lone surrogate
    // escape unpredictable, and a Json text is a String of scalar values, so
    // there is nothing it could decode to.
    assert_eq!(refusal(br#""\uD800""#).0, Refusal::LoneSurrogate);
    assert_eq!(refusal(br#""\uDC00""#).0, Refusal::LoneSurrogate);
    assert_eq!(refusal(br#""\uD800x""#).0, Refusal::LoneSurrogate);
    assert_eq!(refusal(br#""\uD800\u0041""#).0, Refusal::LoneSurrogate);
    assert_eq!(refusal(br#""\uD800\uD800""#).0, Refusal::LoneSurrogate);
    assert_eq!(refusal(br#""\uDE00\uD83D""#).0, Refusal::LoneSurrogate);
    assert_eq!(refusal(br#""\uD83D\n""#).0, Refusal::LoneSurrogate);
}

#[test]
fn a_raw_control_character_in_a_string_is_refused_jc_06() {
    for byte in 0u8..0x20 {
        let input = [b'"', byte, b'"'];
        assert_eq!(
            refusal(&input),
            (Refusal::ControlInString, 1),
            "control {byte:#04x}"
        );
    }
    assert_eq!(text(&canonical(b"\"\x7f\"")), "\"\u{7f}\"");
}

#[test]
fn a_malformed_escape_is_refused_jc_06() {
    for input in [
        &br#""\x41""#[..],
        br#""\a""#,
        br#""\U0041""#,
        br#""\u004""#,
        br#""\u00g1""#,
        br#""\u""#,
        br#""\ ""#,
    ] {
        let (kind, _) = refusal(input);
        assert!(
            matches!(kind, Refusal::BadEscape | Refusal::UnexpectedEnd),
            "{:?}: {kind:?}",
            String::from_utf8_lossy(input)
        );
    }
    assert_eq!(refusal(br#""\x""#), (Refusal::BadEscape, 2));
    assert_eq!(refusal(br#""\u12G4""#), (Refusal::BadEscape, 5));
}

#[test]
fn numbers_map_to_their_decimal_jc_07() {
    // The canonical text of a Decimal is `<mantissa>` or `<mantissa>e<exponent>`.
    let cases: [(&str, &str); 18] = [
        ("0", "0"),
        ("-0", "0"),
        ("7", "7"),
        ("-12", "-12"),
        ("100", "100"),
        ("1.5", "15e-1"),
        ("-0.25", "-25e-2"),
        ("0.001", "1e-3"),
        ("0.0", "0e-1"),
        ("1E2", "1e2"),
        ("1e+2", "1e2"),
        ("1e-2", "1e-2"),
        ("1.50e1", "150e-1"),
        ("0e0", "0"),
        ("123456789012345678", "123456789012345678"),
        ("0.00000000000000000012345", "12345e-23"),
        ("1e9999", "1e9999"),
        ("1e-9999", "1e-9999"),
    ];
    for (input, expected) in cases {
        assert_eq!(
            text(&canonical(input.as_bytes())),
            expected,
            "number {input}"
        );
    }
}

#[test]
fn malformed_numbers_are_refused_as_bad_number_jc_07() {
    for input in [
        "-",
        "+1",
        ".5",
        "1.",
        "1.e2",
        "01",
        "-01",
        "00",
        "1e",
        "1e+",
        "1e-",
        "0x10",
        "1_0",
        "--1",
        "1.2.3",
        "Infinity",
        "NaN",
        "-Infinity",
        "1e1.5",
    ] {
        let (kind, _) = refusal(input.as_bytes());
        assert!(
            matches!(
                kind,
                Refusal::BadNumber
                    | Refusal::UnexpectedByte
                    | Refusal::UnexpectedEnd
                    | Refusal::TrailingContent
            ),
            "{input}: {kind:?}"
        );
    }
    assert_eq!(refusal(b"01"), (Refusal::BadNumber, 1));
    assert_eq!(refusal(b"[1.]"), (Refusal::BadNumber, 3));
    assert_eq!(refusal(b"[-]"), (Refusal::BadNumber, 2));
    assert_eq!(refusal(b"1e+"), (Refusal::BadNumber, 3));
}

#[test]
fn numbers_beyond_the_declared_bounds_are_refused_jc_07() {
    // 18 significant digits is the bound; leading zeros do not count.
    assert_eq!(
        text(&canonical(b"999999999999999999")),
        "999999999999999999"
    );
    assert_eq!(refusal(b"9999999999999999999").0, Refusal::NumberOutOfRange);
    assert_eq!(
        refusal(b"1.0000000000000000000").0,
        Refusal::NumberOutOfRange
    );
    assert_eq!(text(&canonical(b"0.000000000000000000000000001")), "1e-27");
    // The effective exponent is bounded to -9999..9999.
    assert_eq!(refusal(b"1e10000").0, Refusal::NumberOutOfRange);
    assert_eq!(refusal(b"1e-10000").0, Refusal::NumberOutOfRange);
    assert_eq!(
        refusal(b"1e99999999999999999999").0,
        Refusal::NumberOutOfRange
    );
    assert_eq!(refusal(b"0.1e-9999").0, Refusal::NumberOutOfRange);
    assert_eq!(text(&canonical(b"10e9998")), "10e9998");
}

fn nested_arrays(depth: usize) -> Vec<u8> {
    let mut out = vec![b'['; depth];
    out.extend(std::iter::repeat_n(b']', depth));
    out
}

#[test]
fn nesting_to_512_is_accepted_and_513_is_refused_jc_08() {
    assert_eq!(canonical(&nested_arrays(512)), nested_arrays(512));
    assert_eq!(refusal(&nested_arrays(513)).0, Refusal::TooDeep);
    let mut objects = Vec::new();
    for _ in 0..512 {
        objects.extend_from_slice(b"{\"k\":");
    }
    objects.extend_from_slice(b"0");
    objects.extend(std::iter::repeat_n(b'}', 512));
    assert_eq!(canonical(&objects), objects);
    let mut deeper = b"[".to_vec();
    deeper.extend_from_slice(&objects);
    deeper.push(b']');
    assert_eq!(refusal(&deeper).0, Refusal::TooDeep);
}

#[test]
fn a_hundred_thousand_open_brackets_are_refused_without_exhausting_the_stack_jc_08() {
    assert_eq!(refusal(&vec![b'['; 100_000]).0, Refusal::TooDeep);
}

#[test]
fn duplicate_member_names_are_kept_in_source_order_jc_09() {
    assert_eq!(
        text(&canonical(br#"{"a":1,"a":2,"b":3,"a":4}"#)),
        r#"{"a":1,"a":2,"b":3,"a":4}"#
    );
    assert_eq!(lookup_first(br#"{"a":1,"a":2,"b":3}"#), Some(b"1".to_vec()));
    assert_eq!(
        lookup_first(br#"{"b":0,"a":"x","a":2}"#),
        Some(b"\"x\"".to_vec())
    );
    assert_eq!(lookup_first(br#"{"b":1}"#), None);
    assert_eq!(lookup_first(b"{}"), None);
}

#[test]
fn the_encoder_is_compact_and_escapes_jc_10() {
    assert_eq!(
        text(&canonical(b"[ {\"k\" : \"v\"} , [ null ] ]")),
        "[{\"k\":\"v\"},[null]]"
    );
    // Every ASCII character, in order, as the text the encoder must write.
    let written = text(&encode_probe(Probe::Ascii, 0).expect("the ASCII string encodes"));
    let mut expected = String::from("\"");
    for code in 0u8..=127 {
        match code {
            b'"' => expected.push_str("\\\""),
            b'\\' => expected.push_str("\\\\"),
            8 => expected.push_str("\\b"),
            9 => expected.push_str("\\t"),
            10 => expected.push_str("\\n"),
            12 => expected.push_str("\\f"),
            13 => expected.push_str("\\r"),
            0..=0x1F => expected.push_str(&format!("\\u{code:04x}")),
            _ => expected.push(code as char),
        }
    }
    expected.push('"');
    assert_eq!(written, expected);
}

#[test]
fn the_encoder_refuses_what_is_outside_its_bounds_jc_10() {
    let wide = encode_probe(Probe::Digits, 19).expect_err("19 digits are out of bounds");
    assert_eq!(wide.kind, Refusal::EncodeNumberOutOfRange);
    assert!(encode_probe(Probe::Digits, 18).is_ok());
    let high = encode_probe(Probe::Exponent, 10_000).expect_err("exponent 10000");
    assert_eq!(high.kind, Refusal::EncodeNumberOutOfRange);
    assert!(encode_probe(Probe::Exponent, 9_999).is_ok());
    let low = encode_probe(Probe::NegativeExponent, 10_000).expect_err("exponent -10000");
    assert_eq!(low.kind, Refusal::EncodeNumberOutOfRange);
    assert!(encode_probe(Probe::NegativeExponent, 9_999).is_ok());
    let deep = encode_probe(Probe::Depth, 513).expect_err("depth 513");
    assert_eq!(deep.kind, Refusal::EncodeTooDeep);
    let ok = encode_probe(Probe::Depth, 512).expect("depth 512");
    let mut expected = vec![b'['; 512];
    expected.extend_from_slice(b"null");
    expected.extend(std::iter::repeat_n(b']', 512));
    assert_eq!(ok, expected);
}

#[test]
fn the_encoder_refuses_an_exhausted_budget_jc_10() {
    let refused = encode_probe(Probe::Budget, 3).expect_err("a budget of three steps");
    assert_eq!(refused.kind, Refusal::EncodeBudget);
    assert!(encode_probe(Probe::Budget, 1_000).is_ok());
}

#[test]
fn every_parser_refusal_has_a_negative_case_jc_04() {
    let cases: [(&[u8], Refusal, u64); 8] = [
        (b"\xFF", Refusal::InvalidUtf8, 0),
        (b"\xE2\x82", Refusal::TruncatedUtf8, 2),
        (b"[1 2]", Refusal::UnexpectedByte, 3),
        (b"[1,", Refusal::UnexpectedEnd, 0),
        (b"\"\x01\"", Refusal::ControlInString, 1),
        (b"1 2", Refusal::TrailingContent, 2),
        (b"{} {}", Refusal::TrailingContent, 3),
        (b"", Refusal::UnexpectedEnd, 0),
    ];
    for (input, kind, offset) in cases {
        assert_eq!(refusal(input), (kind, offset), "{input:02x?}");
    }
}

#[test]
fn the_ungated_reader_still_refuses_invalid_text_inside_a_string_jc_02() {
    // The UTF-8 gate runs first, so no input reaches the string decoder with an
    // ill-formed sequence. The reader behind it has its own refusal for the case
    // where the gate is bypassed: this is the second line of defence.
    match parse_ungated(b"\"\xC0\x80\"") {
        Parse::Refused(d) => assert_eq!(d.kind, Refusal::InternalText),
        other => panic!("the ungated reader accepted ill-formed text: {other:?}"),
    }
    assert!(matches!(parse_ungated(b"\"ok\""), Parse::Value(_)));
}

#[test]
fn every_refusal_kind_is_provoked_by_some_input_jc_04() {
    let mut seen: Vec<Refusal> = Vec::new();
    let texts: [&[u8]; 11] = [
        b"\xFF",
        b"\xE2\x82",
        b"[1 2]",
        b"[1,",
        b"\"\x01\"",
        br#""\x""#,
        br#""\uD800""#,
        b"[-]",
        b"1e10000",
        b"[[[[",
        b"1 2",
    ];
    for text in texts {
        match parse(text) {
            Parse::Refused(d) => seen.push(d.kind),
            other => panic!("{text:02x?} was not refused: {other:?}"),
        }
    }
    let mut deep = vec![b'['; 513];
    deep.extend(std::iter::repeat_n(b']', 513));
    if let Parse::Refused(d) = parse(&deep) {
        seen.push(d.kind);
    }
    for (probe, size) in [(Probe::Digits, 19), (Probe::Depth, 513), (Probe::Budget, 3)] {
        seen.push(
            encode_probe(probe, size)
                .expect_err("an out-of-bounds probe")
                .kind,
        );
    }
    if let Parse::Refused(d) = parse_ungated(b"\"\xC0\x80\"") {
        seen.push(d.kind);
    }
    for kind in Refusal::ALL {
        assert!(seen.contains(&kind), "no input provokes {kind:?}");
    }
    assert_eq!(
        Refusal::ALL.len(),
        15,
        "the codec declares fifteen refusal kinds"
    );
}
