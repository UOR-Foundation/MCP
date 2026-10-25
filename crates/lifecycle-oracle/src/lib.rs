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
// named, plus `infallible_destructuring_match` (a pair is a one-constructor type that the
// export destructures with a match; the resource catalogs are lists of pairs), and the two
// the include itself causes: items nothing here calls, and no documentation on generated
// items.
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
    clippy::manual_ok_err,
    clippy::collapsible_match,
    clippy::infallible_destructuring_match
)]
#[rustfmt::skip]
#[path = "../generated/mcp_lifecycle/src/lib.rs"]
mod lifecycle;

// The second root, over a catalog of many tools, for the stress tests (`TOOLS.md`).
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
    clippy::manual_ok_err,
    clippy::collapsible_match,
    clippy::infallible_destructuring_match
)]
#[rustfmt::skip]
#[path = "../generated/mcp_crowd/src/lib.rs"]
mod crowd;

// The third root, the step serve over a provider that requests host effects, for the
// provider effect tests (`PROVIDER-EFFECTS.md`). The fixture itself requests none.
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
    clippy::manual_ok_err,
    clippy::collapsible_match,
    clippy::infallible_destructuring_match
)]
#[rustfmt::skip]
#[path = "../generated/mcp_step/src/lib.rs"]
mod step;

/// The text of the generated package this crate compiled, so a test can hash the
/// file the oracle ran against the digest the export recorded.
pub const GENERATED_SOURCE: &str = include_str!("../generated/mcp_lifecycle/src/lib.rs");

/// The text of the second generated package, the root `UorMcp.Crowd.crowdRun`.
pub const CROWD_SOURCE: &str = include_str!("../generated/mcp_crowd/src/lib.rs");

/// The text of the third generated package, the root `UorMcp.StepFixture.stepFixtureRun`.
pub const STEP_SOURCE: &str = include_str!("../generated/mcp_step/src/lib.rs");

/// The stack the root runs on: 2 MiB, which is the size Rust gives a spawned thread
/// when nothing says otherwise. The export states no `recursion` effect and no
/// function of the package calls itself except as a loop, so the stack the root needs
/// does not grow with the input; `tests/stress.rs` runs the largest frames the tests
/// build on exactly this stack, and `LIFECYCLE.md` states the bound.
pub const STACK_BYTES: usize = 2 * 1024 * 1024;

/// One header field as the wire carries it: the name and the value as bytes.
pub type Field = (Vec<u8>, Vec<u8>);

/// What the host tells the core about the instant and the task table of one request
/// (`TASKS.md`, "The host's side"). The core keeps no state, so the clock, an
/// identifier for a task this request may create, and the table of tasks the host
/// holds arrive with the request, and the table the core returns replaces the host's.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TaskHost {
    /// A monotonic millisecond counter. Only differences between two readings matter.
    pub now_ms: u64,
    /// The same instant as ISO 8601 text, written into `createdAt` and `lastUpdatedAt`.
    pub now: String,
    /// The identifier a task created by this request receives. The core cannot make one.
    pub fresh_id: String,
    /// The table of tasks as the previous response returned it; empty for none.
    pub table: Vec<u8>,
}

/// An HTTP request at the core boundary: the method, the target, the header fields
/// in the order received, and the body.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Request {
    /// The request method, for example `POST`.
    pub method: Vec<u8>,
    /// The request target. The core never inspects it.
    pub target: Vec<u8>,
    /// The header fields in the order received.
    pub fields: Vec<Field>,
    /// The body bytes.
    pub body: Vec<u8>,
    /// The number of `tools/call` requests the host has already counted for this client
    /// in the current rate-limit window, not counting this one. The host owns the window
    /// and the client identity; the core only compares the count with the limit its
    /// provider declares (`LIFECYCLE.md`, "The boundary encoding").
    pub tool_calls: u32,
    /// The key the host holds for `requestState` (SEP-2322): the secret reference
    /// `request-state-key` of issue 10. The core keeps no secret of its own, so the host
    /// reads the key from its secret store and passes it with every request, the way it
    /// passes `tool_calls`; the core computes and checks an HMAC with it
    /// (`INPUT-REQUIRED.md`). An empty key, or one shorter than 32 bytes, is no key.
    pub state_key: Vec<u8>,
    /// The clock, the identifier for a new task and the task table of this request.
    pub host: TaskHost,
}

/// An HTTP response at the core boundary: the status, the header fields, the progress
/// notifications and the body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    /// The status code the core chose.
    pub status: u32,
    /// The header fields the core wrote.
    pub fields: Vec<Field>,
    /// Reserved: always empty since issue 11. Progress notifications travel inside the
    /// body, which is then an event stream (`Content-Type: text/event-stream`) that the
    /// core encoded; `TRANSPORT.md` states when. The frame keeps the field so that frames
    /// written before the encoding moved into the core stay readable.
    pub notifications: Vec<Vec<u8>>,
    /// The body bytes.
    pub body: Vec<u8>,
    /// The task table the host stores from now on, when the core changed it; `None`
    /// means the host keeps the table it holds.
    pub tasks: Option<Vec<u8>>,
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
/// value, a length-prefixed body, the four-byte count of tool calls the host has
/// already counted for this client and, when the host holds one, the length-prefixed
/// `requestState` key, every length and count four bytes big-endian. A request whose
/// [`TaskHost`] is not the default also carries the host section of the tasks
/// extension after the allow-list (`LIFECYCLE.md`); the default writes nothing for it.
pub fn encode_request(request: &Request) -> Vec<u8> {
    encode_request_allowing(request, &[])
}

/// [`encode_request`] with the host's allow-list of `Origin` and `Host` names: a
/// four-byte count and each name length-prefixed. An empty list writes nothing unless
/// the host section follows, which it then precedes with a count of 0
/// (`TRANSPORT.md`, "The allow-list"). The list follows the `requestState` key, which a
/// frame without a key writes as a zero length when anything follows it.
pub fn encode_request_allowing(request: &Request, allowed_hosts: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    put_chunk(&mut out, &request.method);
    put_chunk(&mut out, &request.target);
    put_u32(&mut out, request.fields.len());
    for (name, value) in &request.fields {
        put_chunk(&mut out, name);
        put_chunk(&mut out, value);
    }
    put_chunk(&mut out, &request.body);
    put_u32(&mut out, request.tool_calls as usize);
    let with_host = request.host != TaskHost::default();
    if !request.state_key.is_empty() {
        put_chunk(&mut out, &request.state_key);
    } else if !allowed_hosts.is_empty() || with_host {
        put_chunk(&mut out, &[]);
    }
    if !allowed_hosts.is_empty() || with_host {
        put_u32(&mut out, allowed_hosts.len());
        for name in allowed_hosts {
            put_chunk(&mut out, name);
        }
    }
    if with_host {
        out.extend_from_slice(&request.host.now_ms.to_be_bytes());
        put_chunk(&mut out, request.host.now.as_bytes());
        put_chunk(&mut out, request.host.fresh_id.as_bytes());
        put_chunk(&mut out, &request.host.table);
    }
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

/// Decode the frame the root writes: the status, the field count, each field, the
/// notification count, each notification as a length-prefixed text, a
/// length-prefixed body, and, when the root returns a task table, a flag of 1 and the
/// table as a length-prefixed chunk; a frame that ends after the body returns none, and
/// there are no bytes after the table.
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
    let note_count = cursor.word()?;
    let mut notifications = Vec::new();
    for _ in 0..note_count {
        notifications.push(cursor.chunk()?);
    }
    let body = cursor.chunk()?;
    let tasks = if cursor.at == frame.len() {
        None
    } else {
        match cursor.word()? {
            1 => Some(cursor.chunk()?),
            _ => return Err(FrameError::Truncated),
        }
    };
    if cursor.at != frame.len() {
        return Err(FrameError::Trailing);
    }
    Ok(Response {
        status,
        fields,
        notifications,
        body,
        tasks,
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
/// bytes out. The call runs on a thread with a [`STACK_BYTES`] stack.
///
/// # Panics
///
/// Panics when the realization reports an overflow, which the model does not admit
/// for any frame the vectors send.
pub fn exchange_frame(frame: &[u8]) -> Vec<u8> {
    exchange_frame_on(frame, STACK_BYTES)
}

/// [`exchange_frame`] on a thread with `stack_bytes` of stack, so a measurement can
/// ask how little the root needs.
///
/// # Panics
///
/// Panics when the realization reports an overflow, and aborts the process when the
/// stack is too small for the frame.
pub fn exchange_frame_on(frame: &[u8], stack_bytes: usize) -> Vec<u8> {
    let input = frame.to_vec();
    let worker = std::thread::Builder::new()
        .stack_size(stack_bytes)
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

/// [`exchange_frame_on`] for the second root, `UorMcp.Crowd.crowdRun`: the same pipeline
/// over a catalog of 60 tools named `crowd_0` to `crowd_59`.
///
/// # Panics
///
/// Panics when the realization reports an overflow, and aborts the process when the
/// stack is too small for the frame.
pub fn exchange_crowd_frame_on(frame: &[u8], stack_bytes: usize) -> Vec<u8> {
    let input = frame.to_vec();
    let worker = std::thread::Builder::new()
        .stack_size(stack_bytes)
        .spawn(move || {
            let mut list = crowd::run(crowd::Bytes::lit(&input))
                .expect("the crowd root does not overflow on this input");
            let mut out = Vec::new();
            while let Some((head, tail)) = list.uncons() {
                out.push(head);
                list = tail;
            }
            out
        })
        .expect("a worker thread starts");
    words_to_bytes(worker.join().expect("the crowd root did not panic"))
}

/// A step of the root of a server application that may request host effects: PrismPM's
/// replay driver reads it (`PROVIDER-EFFECTS.md`, "The turn and the step").
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    /// The word 0 and the response frame.
    Done(Vec<u8>),
    /// The word 1, the tag of the effect to perform and the payload of its request.
    Perform {
        /// The effect tag, four bytes big-endian on the wire.
        tag: u32,
        /// The bytes of the request of the effect.
        payload: Vec<u8>,
    },
}

/// Why a step is not a step the driver reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepError {
    /// The step ends inside the word or the tag.
    Truncated,
    /// The first word is neither 0 nor 1.
    Unknown(u32),
}

/// Encode a turn the way PrismPM's replay driver writes it: the request frame as a
/// length-prefixed chunk, the count of completed effects, and for each its tag and its
/// outcome as a length-prefixed chunk, every length, count and tag four bytes big-endian.
pub fn encode_turn(request_frame: &[u8], completed: &[(u32, Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::new();
    put_chunk(&mut out, request_frame);
    put_u32(&mut out, completed.len());
    for (tag, outcome) in completed {
        out.extend_from_slice(&tag.to_be_bytes());
        put_chunk(&mut out, outcome);
    }
    out
}

/// Decode the step the root wrote.
///
/// # Errors
///
/// Returns [`StepError`] when the bytes are not one of the two forms of a step.
pub fn decode_step(bytes: &[u8]) -> Result<Step, StepError> {
    let word = |at: usize| -> Result<u32, StepError> {
        let end = at.checked_add(4).ok_or(StepError::Truncated)?;
        let bytes = bytes.get(at..end).ok_or(StepError::Truncated)?;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    };
    match word(0)? {
        0 => Ok(Step::Done(bytes[4..].to_vec())),
        1 => Ok(Step::Perform {
            tag: word(4)?,
            payload: bytes[8..].to_vec(),
        }),
        other => Err(StepError::Unknown(other)),
    }
}

/// [`exchange_frame_on`] for the third root, `UorMcp.StepFixture.stepFixtureRun`: a turn
/// in, the bytes of a step out.
///
/// # Panics
///
/// Panics when the realization reports an overflow, and aborts the process when the
/// stack is too small for the turn.
pub fn exchange_step_on(turn: &[u8], stack_bytes: usize) -> Vec<u8> {
    let input = turn.to_vec();
    let worker = std::thread::Builder::new()
        .stack_size(stack_bytes)
        .spawn(move || {
            let mut list = step::run(step::Bytes::lit(&input))
                .expect("the step root does not overflow on this input");
            let mut out = Vec::new();
            while let Some((head, tail)) = list.uncons() {
                out.push(head);
                list = tail;
            }
            out
        })
        .expect("a worker thread starts");
    words_to_bytes(worker.join().expect("the step root did not panic"))
}

/// [`exchange_step_on`] on the default [`STACK_BYTES`] stack.
pub fn exchange_step(turn: &[u8]) -> Vec<u8> {
    exchange_step_on(turn, STACK_BYTES)
}

/// What a driver saw while it answered one request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Driven {
    /// The response frame of the last turn.
    pub response: Vec<u8>,
    /// The effects the root asked for, in order: the tag and the payload of each.
    pub performed: Vec<(u32, Vec<u8>)>,
    /// How many times the root was called.
    pub turns: usize,
}

/// Why a driver stopped without a response.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DriveError {
    /// The root wrote bytes that are not a step.
    Step(StepError),
    /// The root asked for more effects than the budget allows.
    Budget,
}

/// Drive the root the way PrismPM's generated driver does: call it with the turn, perform
/// each effect it asks for through `host` (the tag and the payload in, the outcome out),
/// append the completed effect to the turn and call it again, until it is done or
/// `budget` effects were performed.
///
/// # Errors
///
/// Returns [`DriveError`] when the root writes something that is not a step or asks for
/// more than `budget` effects.
pub fn drive(
    request_frame: &[u8],
    budget: usize,
    mut host: impl FnMut(u32, &[u8]) -> Vec<u8>,
) -> Result<Driven, DriveError> {
    let mut completed: Vec<(u32, Vec<u8>)> = Vec::new();
    let mut performed: Vec<(u32, Vec<u8>)> = Vec::new();
    let mut turns = 0;
    loop {
        turns += 1;
        let written = exchange_step(&encode_turn(request_frame, &completed));
        match decode_step(&written).map_err(DriveError::Step)? {
            Step::Done(response) => {
                return Ok(Driven {
                    response,
                    performed,
                    turns,
                });
            }
            Step::Perform { tag, payload } => {
                if completed.len() >= budget {
                    return Err(DriveError::Budget);
                }
                let outcome = host(tag, &payload);
                performed.push((tag, payload));
                completed.push((tag, outcome));
            }
        }
    }
}

/// One HTTP exchange through the generated root.
///
/// # Errors
///
/// Returns [`FrameError`] when the root's answer is not a response frame.
pub fn exchange(request: &Request) -> Result<Response, FrameError> {
    decode_response(&exchange_frame(&encode_request(request)))
}

/// [`exchange`] for a host that configured `allowed_hosts` besides the loopback names.
///
/// # Errors
///
/// Returns [`FrameError`] when the root's answer is not a response frame.
pub fn exchange_allowing(
    request: &Request,
    allowed_hosts: &[Vec<u8>],
) -> Result<Response, FrameError> {
    decode_response(&exchange_frame(&encode_request_allowing(
        request,
        allowed_hosts,
    )))
}

/// One event of an event stream, as the WHATWG HTML Standard (section 9.2, "Server-sent
/// events") has a client interpret it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Event {
    /// The event type; `message` when the stream names none.
    pub kind: String,
    /// The data lines joined with a line feed, without the last one.
    pub data: String,
    /// The `id` field, when the stream carried one.
    pub id: Option<String>,
    /// The `retry` field, when the stream carried one.
    pub retry: Option<String>,
}

/// An event stream read the way the WHATWG HTML Standard says a client reads it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParsedStream {
    /// The events the stream dispatched, in order.
    pub events: Vec<Event>,
    /// True when the stream ended inside an event (a client discards that event).
    pub dangling: bool,
}

/// Read an event stream as a client does (WHATWG HTML Standard, section 9.2.6): UTF-8,
/// a leading byte order mark removed, lines ended by LF, CR or CRLF, a blank line
/// dispatching the event, a line starting with a colon a comment, and the fields `event`,
/// `data`, `id` and `retry` interpreted. This reader is the oracle for the core's framing:
/// it shares no code with the encoder in `src/Transport.lex.tex`.
///
/// # Errors
///
/// Returns the reason when the bytes are not UTF-8.
pub fn parse_event_stream(bytes: &[u8]) -> Result<ParsedStream, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("not UTF-8: {e}"))?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let raw = text.as_bytes();
    let mut lines: Vec<&str> = Vec::new();
    let mut start = 0;
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'\n' || raw[i] == b'\r' {
            lines.push(&text[start..i]);
            if raw[i] == b'\r' && raw.get(i + 1) == Some(&b'\n') {
                i += 1;
            }
            start = i + 1;
        }
        i += 1;
    }
    let dangling_line = start < raw.len();
    let mut out = ParsedStream::default();
    let mut event = Event::default();
    let mut data = String::new();
    let mut open = false;
    for line in lines {
        if line.is_empty() {
            if data.is_empty() {
                event = Event::default();
            } else {
                data.pop();
                event.data = std::mem::take(&mut data);
                if event.kind.is_empty() {
                    event.kind = "message".to_owned();
                }
                out.events.push(std::mem::take(&mut event));
            }
            open = false;
            continue;
        }
        open = true;
        if line.starts_with(':') {
            continue;
        }
        let (field, value) = match line.split_once(':') {
            Some((f, v)) => (f, v.strip_prefix(' ').unwrap_or(v)),
            None => (line, ""),
        };
        match field {
            "event" => event.kind = value.to_owned(),
            "data" => {
                data.push_str(value);
                data.push('\n');
            }
            "id" if !value.contains('\0') => event.id = Some(value.to_owned()),
            "retry" if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) => {
                event.retry = Some(value.to_owned());
            }
            _ => {}
        }
    }
    out.dangling = open || dangling_line;
    Ok(out)
}
