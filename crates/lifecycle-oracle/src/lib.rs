//! Runs the package generated from the LexLean stateless lifecycle root against
//! vectors taken from the pinned MCP conformance scenarios (issue #7).
//!
//! Dev and CI only. Nothing here is a dependency of a shipped crate: the protocol
//! is LexLean source in `src/`, and this crate is where its generated Rust package
//! is executed by `cargo test`. The package is LexLean's own export of the root
//! `UorMcp.Fixture.fixtureRun`; this file only converts between an HTTP exchange and
//! the first-order frame the root reads and writes (`LIFECYCLE.md`, "The boundary
//! encoding"). It parses no header, no JSON and no protocol rule itself, and it
//! decides no status: the status in a [`Response`] is the word the root wrote.
//!
//! The host adapter of PrismPM issue 216 satisfies the same frame; what this crate
//! does for a test, that adapter does for a socket.

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
#[path = "../generated/mcp_lifecycle/src/lib.rs"]
mod lifecycle;

/// The text of the generated package this crate compiled, so a test can hash the
/// file the oracle ran against the digest the export recorded.
pub const GENERATED_SOURCE: &str = include_str!("../generated/mcp_lifecycle/src/lib.rs");

/// The stack the root runs on. The root recurses to a depth bounded only by its
/// input (the export states the `recursion` effect), so the host gives it a stack
/// its input fits; this is that stack for every input the vectors send.
const STACK_BYTES: usize = 1 << 29;

/// One header field as the wire carries it: the name and the value as bytes.
pub type Field = (Vec<u8>, Vec<u8>);

/// An HTTP request at the core boundary: the method, the target, the header fields
/// in the order received, and the body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    /// The request method, for example `POST`.
    pub method: Vec<u8>,
    /// The request target. The core never inspects it.
    pub target: Vec<u8>,
    /// The header fields in the order received.
    pub fields: Vec<Field>,
    /// The body bytes.
    pub body: Vec<u8>,
}

/// An HTTP response at the core boundary: the status, the header fields and the body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    /// The status code the core chose.
    pub status: u32,
    /// The header fields the core wrote.
    pub fields: Vec<Field>,
    /// The body bytes.
    pub body: Vec<u8>,
}

/// Why a response frame is not a response frame. The root writes well-formed frames,
/// so seeing one of these is a defect of the root or of this decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrameError {
    /// The frame ended inside a field.
    Truncated,
    /// Bytes remain after the body.
    Trailing,
}

fn put_u32(out: &mut Vec<u8>, value: usize) {
    let value = u32::try_from(value).expect("a frame length fits in thirty-two bits");
    out.extend_from_slice(&value.to_be_bytes());
}

fn put_chunk(out: &mut Vec<u8>, bytes: &[u8]) {
    put_u32(out, bytes.len());
    out.extend_from_slice(bytes);
}

/// Encode a request as the frame the root reads: a length-prefixed method, a
/// length-prefixed target, the field count, each field as a length-prefixed name and
/// value, and a length-prefixed body, every length four bytes big-endian.
pub fn encode_request(request: &Request) -> Vec<u8> {
    let mut out = Vec::new();
    put_chunk(&mut out, &request.method);
    put_chunk(&mut out, &request.target);
    put_u32(&mut out, request.fields.len());
    for (name, value) in &request.fields {
        put_chunk(&mut out, name);
        put_chunk(&mut out, value);
    }
    put_chunk(&mut out, &request.body);
    out
}

struct Cursor<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Cursor<'_> {
    fn word(&mut self) -> Result<u32, FrameError> {
        let end = self.at.checked_add(4).ok_or(FrameError::Truncated)?;
        let word = self.bytes.get(self.at..end).ok_or(FrameError::Truncated)?;
        self.at = end;
        Ok(u32::from_be_bytes([word[0], word[1], word[2], word[3]]))
    }

    fn chunk(&mut self) -> Result<Vec<u8>, FrameError> {
        let len = usize::try_from(self.word()?).map_err(|_| FrameError::Truncated)?;
        let end = self.at.checked_add(len).ok_or(FrameError::Truncated)?;
        let chunk = self.bytes.get(self.at..end).ok_or(FrameError::Truncated)?;
        self.at = end;
        Ok(chunk.to_vec())
    }
}

/// Decode the frame the root writes: the status, the field count, each field, and a
/// length-prefixed body, with no bytes after it.
///
/// # Errors
///
/// Returns [`FrameError`] when the frame is cut short or has trailing bytes.
pub fn decode_response(frame: &[u8]) -> Result<Response, FrameError> {
    let mut cursor = Cursor {
        bytes: frame,
        at: 0,
    };
    let status = cursor.word()?;
    let count = cursor.word()?;
    let mut fields = Vec::new();
    for _ in 0..count {
        let name = cursor.chunk()?;
        let value = cursor.chunk()?;
        fields.push((name, value));
    }
    let body = cursor.chunk()?;
    if cursor.at != frame.len() {
        return Err(FrameError::Trailing);
    }
    Ok(Response {
        status,
        fields,
        body,
    })
}

fn words_to_bytes(words: Vec<u64>) -> Vec<u8> {
    words
        .into_iter()
        .map(|word| u8::try_from(word).expect("the root wrote a byte above 255"))
        .collect()
}

/// Run the generated root on an encoded request and return the encoded response.
///
/// The root is the LexLean function `fixtureRun`: the exact bytes in, the exact
/// bytes out. The call runs on a thread with the stack the export's `recursion`
/// effect asks the host for.
///
/// # Panics
///
/// Panics when the realization reports an overflow, which the model does not admit
/// for any frame the vectors send.
pub fn exchange_frame(frame: &[u8]) -> Vec<u8> {
    let input = frame.to_vec();
    let worker = std::thread::Builder::new()
        .stack_size(STACK_BYTES)
        .spawn(move || {
            let mut list = lifecycle::run(lifecycle::Bytes::lit(&input))
                .expect("the lifecycle root does not overflow on this input");
            let mut out = Vec::new();
            while let Some((head, tail)) = list.uncons() {
                out.push(head);
                list = tail;
            }
            out
        })
        .expect("a worker thread starts");
    words_to_bytes(worker.join().expect("the lifecycle root did not panic"))
}

/// One HTTP exchange through the generated root.
///
/// # Errors
///
/// Returns [`FrameError`] when the root's answer is not a response frame.
pub fn exchange(request: &Request) -> Result<Response, FrameError> {
    decode_response(&exchange_frame(&encode_request(request)))
}
