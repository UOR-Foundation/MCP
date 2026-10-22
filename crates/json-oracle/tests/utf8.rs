//! JC-01 and JC-02: the Unicode byte validator and the parser's refusal of any
//! byte string that is not well-formed UTF-8.

use json_oracle::{parse, utf8, Parse, Refusal, Verdict};

/// RFC 3629 Table 3-7, written out independently of the LexLean source: the
/// verdict for `bytes` and, when it is not valid, the offset.
fn reference(bytes: &[u8]) -> Verdict {
    let mut at = 0;
    while at < bytes.len() {
        let lead = bytes[at];
        let (second, tail): (std::ops::RangeInclusive<u8>, usize) = match lead {
            0x00..=0x7F => {
                at += 1;
                continue;
            }
            0xC2..=0xDF => (0x80..=0xBF, 1),
            0xE0 => (0xA0..=0xBF, 2),
            0xE1..=0xEC | 0xEE..=0xEF => (0x80..=0xBF, 2),
            0xED => (0x80..=0x9F, 2),
            0xF0 => (0x90..=0xBF, 3),
            0xF1..=0xF3 => (0x80..=0xBF, 3),
            0xF4 => (0x80..=0x8F, 3),
            _ => return Verdict::Invalid(at as u64),
        };
        for step in 1..=tail {
            let Some(&byte) = bytes.get(at + step) else {
                return Verdict::Truncated(bytes.len() as u64);
            };
            let allowed = if step == 1 {
                second.contains(&byte)
            } else {
                (0x80..=0xBF).contains(&byte)
            };
            if !allowed {
                return Verdict::Invalid((at + step) as u64);
            }
        }
        at += tail + 1;
    }
    Verdict::Valid
}

/// The standard library's verdict, reduced to the three classes.
fn standard(bytes: &[u8]) -> &'static str {
    match std::str::from_utf8(bytes) {
        Ok(_) => "valid",
        Err(e) if e.error_len().is_none() => "truncated",
        Err(_) => "invalid",
    }
}

fn class(verdict: &Verdict) -> &'static str {
    match verdict {
        Verdict::Valid => "valid",
        Verdict::Invalid(_) => "invalid",
        Verdict::Truncated(_) => "truncated",
    }
}

/// Byte strings that probe every Table 3-7 boundary.
fn probes() -> Vec<Vec<u8>> {
    let edge: [u8; 12] = [
        0x00, 0x7F, 0x80, 0x8F, 0x90, 0x9F, 0xA0, 0xBF, 0xC0, 0xC1, 0xF5, 0xFF,
    ];
    let mut out: Vec<Vec<u8>> = vec![Vec::new()];
    for a in 0..=255u8 {
        out.push(vec![a]);
        for b in 0..=255u8 {
            out.push(vec![a, b]);
        }
    }
    for a in 0..=255u8 {
        for b in 0..=255u8 {
            for c in edge {
                out.push(vec![a, b, c]);
            }
        }
    }
    for a in 0xEFu8..=0xF5 {
        for b in edge {
            for c in edge {
                for d in edge {
                    out.push(vec![a, b, c, d]);
                    out.push(vec![a, b, c, d, b'x']);
                }
            }
        }
    }
    // A valid prefix, then each kind of fault, then a valid suffix.
    for fault in [
        &[0xC0u8, 0x80][..],
        &[0xED, 0xA0, 0x80],
        &[0xF4, 0x90, 0x80, 0x80],
        &[0xE2, 0x82],
    ] {
        out.push([b"ok \xC3\xA9 ".as_slice(), fault, b" tail"].concat());
    }
    out
}

#[test]
fn the_validator_agrees_with_table_3_7_and_the_standard_library_jc_01() {
    let mut count = 0usize;
    for bytes in probes() {
        let got = utf8(&bytes);
        assert_eq!(class(&got), standard(&bytes), "class of {bytes:02x?}");
        assert_eq!(got, reference(&bytes), "verdict of {bytes:02x?}");
        count += 1;
    }
    assert!(count > 100_000, "only {count} byte strings were judged");
}

#[test]
fn the_validator_names_each_forbidden_form_jc_01() {
    // Overlong forms: the byte after the lead is where Table 3-7 stops admitting.
    assert_eq!(utf8(&[0xC0, 0x80]), Verdict::Invalid(0));
    assert_eq!(utf8(&[0xC1, 0xBF]), Verdict::Invalid(0));
    assert_eq!(utf8(&[0xE0, 0x80, 0x80]), Verdict::Invalid(1));
    assert_eq!(utf8(&[0xE0, 0x9F, 0xBF]), Verdict::Invalid(1));
    assert_eq!(utf8(&[0xF0, 0x80, 0x80, 0x80]), Verdict::Invalid(1));
    assert_eq!(utf8(&[0xF0, 0x8F, 0xBF, 0xBF]), Verdict::Invalid(1));
    // Surrogates U+D800..U+DFFF.
    assert_eq!(utf8(&[0xED, 0xA0, 0x80]), Verdict::Invalid(1));
    assert_eq!(utf8(&[0xED, 0xBF, 0xBF]), Verdict::Invalid(1));
    assert_eq!(utf8(&[0xED, 0x9F, 0xBF]), Verdict::Valid);
    // Above U+10FFFF.
    assert_eq!(utf8(&[0xF4, 0x90, 0x80, 0x80]), Verdict::Invalid(1));
    assert_eq!(utf8(&[0xF4, 0x8F, 0xBF, 0xBF]), Verdict::Valid);
    assert_eq!(utf8(&[0xF5, 0x80, 0x80, 0x80]), Verdict::Invalid(0));
    assert_eq!(utf8(&[0xFF]), Verdict::Invalid(0));
    // A continuation byte with no lead.
    assert_eq!(utf8(&[b'a', 0x80]), Verdict::Invalid(1));
    // Truncated sequences report the end of the input.
    assert_eq!(utf8(&[0xC2]), Verdict::Truncated(1));
    assert_eq!(utf8(&[b'a', 0xE2, 0x82]), Verdict::Truncated(3));
    assert_eq!(utf8(&[0xF0, 0x9F, 0x98]), Verdict::Truncated(3));
    // Valid text of every length class.
    assert_eq!(utf8("a\u{e9}\u{20ac}\u{1F600}".as_bytes()), Verdict::Valid);
    assert_eq!(utf8(&[]), Verdict::Valid);
}

#[test]
fn the_parser_refuses_every_short_byte_string_that_is_not_utf8_jc_02() {
    // Every byte string of one and two bytes: exhaustive, so a gap in the gate
    // cannot hide in a sampled space.
    let mut refused = 0usize;
    let mut all: Vec<Vec<u8>> = (0..=255u8).map(|a| vec![a]).collect();
    all.extend((0..=255u8).flat_map(|a| (0..=255u8).map(move |b| vec![a, b])));
    for bytes in all {
        let valid = std::str::from_utf8(&bytes).is_ok();
        match parse(&bytes) {
            Parse::Refused(d)
                if matches!(d.kind, Refusal::InvalidUtf8 | Refusal::TruncatedUtf8) =>
            {
                assert!(!valid, "{bytes:02x?} is well-formed yet refused as UTF-8");
                refused += 1;
            }
            _ => assert!(valid, "{bytes:02x?} is not UTF-8 yet not refused as UTF-8"),
        }
    }
    // 65 792 strings of one or two bytes, of which 128 + 128 * 128 + 30 * 64 = 18 432 are
    // well-formed.
    assert_eq!(refused, 47_360, "the number of refusals");
}

#[test]
fn utf8_is_judged_before_any_json_structure_jc_02() {
    // Each text is also bad JSON before the fault, after it, or both; the
    // diagnostic is still the encoding's.
    let cases: [(&[u8], Refusal, u64); 6] = [
        (b"[1,]\xFF", Refusal::InvalidUtf8, 4),
        (b"\xFF[", Refusal::InvalidUtf8, 0),
        (b"\"\xC0\x80\"", Refusal::InvalidUtf8, 1),
        (b"\"\xED\xA0\x80\"", Refusal::InvalidUtf8, 2),
        (b"{\"k\":\"\xF4\x90\x80\x80\"}", Refusal::InvalidUtf8, 7),
        (b"[\"\xE2\x82", Refusal::TruncatedUtf8, 4),
    ];
    for (input, kind, offset) in cases {
        match parse(input) {
            Parse::Refused(d) => {
                assert_eq!(
                    (d.kind, d.offset),
                    (kind, offset),
                    "diagnostic for {input:02x?}"
                );
            }
            other => panic!("{input:02x?} was not refused: {other:?}"),
        }
    }
}

#[test]
fn a_surrogate_in_the_input_bytes_is_never_a_character_jc_02() {
    for pair in [
        [0xED, 0xA0, 0x80],
        [0xED, 0xAD, 0xBF],
        [0xED, 0xB0, 0x80],
        [0xED, 0xBF, 0xBF],
    ] {
        let mut text = b"[\"".to_vec();
        text.extend_from_slice(&pair);
        text.extend_from_slice(b"\"]");
        match parse(&text) {
            Parse::Refused(d) => assert_eq!(d.kind, Refusal::InvalidUtf8, "{pair:02x?}"),
            other => panic!("{pair:02x?} was not refused: {other:?}"),
        }
    }
}
