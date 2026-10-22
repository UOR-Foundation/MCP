//! Runs the package generated from the LexLean JSON codec against the pinned
//! JSONTestSuite corpus (issue #18).
//!
//! Dev and CI only. Nothing here is a dependency of a shipped crate: the codec
//! is LexLean source in `src/`, and this crate is where its generated Rust
//! package is executed by `cargo test`. The package is LexLean's own export of
//! the root `UorMcp.JsonCodec.codecRun`; this file only converts between bytes
//! and the package's `u64` lists. It parses nothing itself.

// The package carries its own lint table (`warnings = deny`, `clippy::all = deny`
// with ten named exceptions) and is clean under it as a crate. Included here as a
// module it inherits this workspace's table instead, so the same ten exceptions are
// named, plus the two the include itself causes: items nothing here calls, and no
// documentation on generated items.
#[allow(
    dead_code,
    unused,
    missing_docs,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err,
    clippy::result_unit_err,
    clippy::single_match,
    clippy::manual_unwrap_or,
    clippy::manual_unwrap_or_default,
    clippy::manual_map,
    clippy::manual_ok_err
)]
#[rustfmt::skip]
#[path = "../generated/mcp_json_codec/src/lib.rs"]
mod codec;

/// The text of the generated package this crate compiled, so a test can hash the
/// file the oracle ran against the digest the export recorded.
pub const GENERATED_SOURCE: &str = include_str!("../generated/mcp_json_codec/src/lib.rs");

/// What the codec refuses with. The order is the code the LexLean root reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    /// A byte that cannot start or continue a UTF-8 sequence.
    InvalidUtf8,
    /// A UTF-8 sequence cut short by the end of the input.
    TruncatedUtf8,
    /// A byte the JSON grammar does not admit at this point.
    UnexpectedByte,
    /// The input ended inside a value.
    UnexpectedEnd,
    /// A raw control character inside a string.
    ControlInString,
    /// A backslash escape that is not one of RFC 8259 section 7.
    BadEscape,
    /// A surrogate escape without its partner.
    LoneSurrogate,
    /// A number outside the RFC 8259 grammar.
    BadNumber,
    /// A number with too many digits or too large an exponent.
    NumberOutOfRange,
    /// Arrays and objects nested deeper than the declared bound.
    TooDeep,
    /// Bytes after a complete value.
    TrailingContent,
    /// The encoder was given a number outside its bounds.
    EncodeNumberOutOfRange,
    /// The encoder was given a value nested deeper than the bound.
    EncodeTooDeep,
    /// The encoder's work budget ran out.
    EncodeBudget,
    /// The reader's string decoder met bytes that are not UTF-8.
    InternalText,
}

impl Refusal {
    /// Every kind, in code order.
    pub const ALL: [Refusal; 15] = [
        Refusal::InvalidUtf8,
        Refusal::TruncatedUtf8,
        Refusal::UnexpectedByte,
        Refusal::UnexpectedEnd,
        Refusal::ControlInString,
        Refusal::BadEscape,
        Refusal::LoneSurrogate,
        Refusal::BadNumber,
        Refusal::NumberOutOfRange,
        Refusal::TooDeep,
        Refusal::TrailingContent,
        Refusal::EncodeNumberOutOfRange,
        Refusal::EncodeTooDeep,
        Refusal::EncodeBudget,
        Refusal::InternalText,
    ];

    /// The name the LexLean constructor carries.
    pub fn name(self) -> &'static str {
        match self {
            Refusal::InvalidUtf8 => "invalidUtf8",
            Refusal::TruncatedUtf8 => "truncatedUtf8",
            Refusal::UnexpectedByte => "unexpectedByte",
            Refusal::UnexpectedEnd => "unexpectedEnd",
            Refusal::ControlInString => "controlInString",
            Refusal::BadEscape => "badEscape",
            Refusal::LoneSurrogate => "loneSurrogate",
            Refusal::BadNumber => "badNumber",
            Refusal::NumberOutOfRange => "numberOutOfRange",
            Refusal::TooDeep => "tooDeep",
            Refusal::TrailingContent => "trailingContent",
            Refusal::EncodeNumberOutOfRange => "encodeNumberOutOfRange",
            Refusal::EncodeTooDeep => "encodeTooDeep",
            Refusal::EncodeBudget => "encodeBudget",
            Refusal::InternalText => "internalText",
        }
    }

    fn from_code(code: u64) -> Refusal {
        usize::try_from(code)
            .ok()
            .and_then(|at| Refusal::ALL.get(at).copied())
            .unwrap_or_else(|| panic!("the codec reported the unknown refusal code {code}"))
    }
}

/// A typed refusal with the byte offset it names (zero when it names none).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Which refusal.
    pub kind: Refusal,
    /// The offset in the input, or zero.
    pub offset: u64,
}

/// The outcome of parsing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Parse {
    /// Accepted; the bytes are the canonical encoding of the parsed value.
    Value(Vec<u8>),
    /// Refused by the parser.
    Refused(Diagnostic),
    /// Parsed, but the encoder refused to write the value back.
    Unwritable(Diagnostic),
}

/// The verdict of the Unicode byte validator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// Well-formed.
    Valid,
    /// The byte at this offset is not admitted.
    Invalid(u64),
    /// A sequence is cut short; the offset is the end of the input.
    Truncated(u64),
}

/// Which synthetic value the encoder probe builds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Probe {
    /// Arrays nested `size` deep around `null`.
    Depth = 0,
    /// A number whose mantissa has `size` digits.
    Digits = 1,
    /// The number `1e<size>`.
    Exponent = 2,
    /// The number `1e-<size>`.
    NegativeExponent = 3,
    /// A string of the 128 ASCII characters in order.
    Ascii = 4,
    /// A small nested value written with a work budget of `size` steps.
    Budget = 5,
}

fn run(mode: u64, input: &[u8]) -> Vec<u64> {
    let mut list = codec::run(mode, codec::Bytes::lit(input))
        .expect("the codec root does not overflow on this input");
    let mut out = Vec::new();
    while let Some((head, tail)) = list.uncons() {
        out.push(head);
        list = tail;
    }
    out
}

fn bytes_of(words: &[u64]) -> Vec<u8> {
    words
        .iter()
        .map(|w| u8::try_from(*w).expect("the codec reported a byte above 255"))
        .collect()
}

fn diagnostic(words: &[u64]) -> Diagnostic {
    match words {
        [_, kind, offset] => Diagnostic {
            kind: Refusal::from_code(*kind),
            offset: *offset,
        },
        other => panic!("a refusal report has three words, not {other:?}"),
    }
}

fn parsed(words: &[u64]) -> Parse {
    match words.first() {
        Some(0) => Parse::Value(bytes_of(&words[1..])),
        Some(1) => Parse::Refused(diagnostic(words)),
        Some(2) => Parse::Unwritable(diagnostic(words)),
        other => panic!("unknown report tag {other:?}"),
    }
}

/// Parse `input` and report the canonical encoding of the value, or the refusal.
pub fn parse(input: &[u8]) -> Parse {
    parsed(&run(0, input))
}

/// Parse `input` without the UTF-8 gate that precedes the reader.
pub fn parse_ungated(input: &[u8]) -> Parse {
    parsed(&run(4, input))
}

/// The Unicode byte validator's verdict on `input`.
pub fn utf8(input: &[u8]) -> Verdict {
    match run(1, input).as_slice() {
        [0] => Verdict::Valid,
        [1, 0, at] => Verdict::Invalid(*at),
        [1, 1, at] => Verdict::Truncated(*at),
        other => panic!("unknown validator report {other:?}"),
    }
}

/// Parse `input`, a text whose value is an object, and report the canonical
/// encoding of the first member named `a`, or `None` when there is none.
pub fn lookup_first(input: &[u8]) -> Option<Vec<u8>> {
    let words = run(3, input);
    match words.first() {
        Some(0) => Some(bytes_of(&words[1..])),
        Some(3) => None,
        other => panic!("unexpected lookup report {other:?} for {words:?}"),
    }
}

/// Encode the synthetic value `probe` of the given `size`.
pub fn encode_probe(probe: Probe, size: u64) -> Result<Vec<u8>, Diagnostic> {
    let input = [probe as u8, (size >> 8) as u8, (size & 0xFF) as u8];
    let words = run(2, &input);
    match words.first() {
        Some(0) => Ok(bytes_of(&words[1..])),
        Some(2) => Err(diagnostic(&words)),
        other => panic!("unexpected probe report {other:?}"),
    }
}
