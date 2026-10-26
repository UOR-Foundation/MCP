//! The declaration of the conformance build, rendered from the core's own answers.
//!
//! `conformance/src/Application.lex.tex` holds one definition, the server application
//! of PrismPM's `Foundation.Server.V1.Model`. Its limits and surfaces are written
//! here, and its vectors are not written by anyone: each is a request taken from the
//! vectors of the pinned scenarios (`vectors/*.json`) and the answer of the committed
//! core package to it, framed and serialized the way the generated HTTP adapter frames
//! and serializes (PrismPM `server_host/serve.rs.in`: `request_frame`,
//! `response_bytes`). `the_vectors_of_the_declaration_are_the_core_packages_answers`
//! regenerates the text and compares it with the committed file, and
//! `cargo run -p lifecycle-oracle --example conformance_application -- --write`
//! writes it.

use lifecycle_oracle::{decode_response, drive_conformance, encode_request, Request, TaskHost};
use serde_json::{json, Value};

use super::{
    hex, iso, request_of, resources_vector_file, tasks_request, tools_vector_file,
    transport_vector_file, vector_file,
};

/// The instant of the acceptance vectors, in the host's millisecond counter.
pub const ACCEPTANCE_MS: u64 = 1_790_000_000_000;

/// The tag of the effect that stores the task table: its position in the declaration
/// (`task.store` is the fifth effect), and the tag `ConformanceFixture.storeEffectTag` names.
pub const STORE_TAG: u32 = 4;

/// The vectors the declaration serves, by the ID of the vector file they come from.
/// Each is a request the pinned scenarios send, so the declaration's vectors exercise
/// discovery, the tools, resources and prompts, an event stream and two refusals.
pub const SERVED: [&str; 11] = [
    "ST-05", "TS-01", "TS-02", "TS-05", "TS-08", "SP-01", "RS-01", "RS-02", "RS-09", "ST-01",
    "ST-14",
];

/// A host effect the core asked for while an acceptance vector was answered: the payload
/// of the request and the outcome the transcript supplies.
pub struct Step {
    pub request: Vec<u8>,
    pub outcome: Vec<u8>,
}

/// One acceptance vector of the tasks extension: the request frame, the response frame
/// and the effects the core asked for between them.
pub struct TaskVector {
    pub frame_request: Vec<u8>,
    pub frame_response: Vec<u8>,
    pub steps: Vec<Step>,
}

/// One vector of the declaration: the request and the answer, as HTTP messages and as
/// the frames the core reads and writes.
pub struct Vector {
    pub http_request: Vec<u8>,
    pub http_response: Vec<u8>,
    pub frame_request: Vec<u8>,
    pub frame_response: Vec<u8>,
}

fn reason(status: u32) -> &'static str {
    match status {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        _ => "Status",
    }
}

fn find(id: &str) -> Value {
    [
        vector_file(),
        tools_vector_file(),
        resources_vector_file(),
        transport_vector_file(),
    ]
    .iter()
    .flat_map(|file| file["vectors"].as_array().expect("vectors").clone())
    .find(|vector| vector["id"] == id)
    .unwrap_or_else(|| panic!("no vector {id}"))
}

/// The vector of the pinned scenarios named `id`, as the adapter would carry it.
pub fn vector(id: &str) -> Vector {
    let mut request: Request = request_of(&find(id));
    // The adapter requires one Host field and the request is delimited by Content-Length;
    // both are fields the core receives in the order the client wrote them.
    request
        .fields
        .insert(0, (b"Host".to_vec(), b"127.0.0.1".to_vec()));
    request.fields.push((
        b"Content-Length".to_vec(),
        request.body.len().to_string().into_bytes(),
    ));
    request.tool_calls = 0;
    request.host = acceptance_host(1, &[]);

    let mut http_request = format!(
        "{} {} HTTP/1.1\r\n",
        String::from_utf8_lossy(&request.method),
        String::from_utf8_lossy(&request.target)
    )
    .into_bytes();
    for (name, value) in &request.fields {
        http_request.extend_from_slice(name);
        http_request.extend_from_slice(b": ");
        http_request.extend_from_slice(value);
        http_request.extend_from_slice(b"\r\n");
    }
    http_request.extend_from_slice(b"\r\n");
    http_request.extend_from_slice(&request.body);

    let frame_request = encode_request(&request);
    let driven = drive_conformance(&frame_request, 0, |tag, _| {
        panic!("the vector {id} asked for the effect {tag}")
    })
    .expect("the conformance root answers without an effect");
    let frame_response = driven.response;
    let response = decode_response(&frame_response).expect("the core writes a response frame");
    assert!(
        response.tasks.is_none(),
        "the response frame of {id} carries a task table"
    );
    assert!(
        response.notifications.is_empty(),
        "the response frame of {id} carries notifications"
    );

    let mut http_response = format!(
        "HTTP/1.1 {} {}\r\n",
        response.status,
        reason(response.status)
    )
    .into_bytes();
    for (name, value) in &response.fields {
        http_response.extend_from_slice(name);
        http_response.extend_from_slice(b": ");
        http_response.extend_from_slice(value);
        http_response.extend_from_slice(b"\r\n");
    }
    http_response
        .extend_from_slice(format!("Content-Length: {}\r\n", response.body.len()).as_bytes());
    http_response.extend_from_slice(b"Connection: close\r\n\r\n");
    http_response.extend_from_slice(&response.body);

    Vector {
        http_request,
        http_response,
        frame_request,
        frame_response,
    }
}

/// The host fields of an acceptance vector: the instant, the identifier a task created by
/// the request would have, and the table the previous request left.
fn acceptance_host(issued: u32, table: &[u8]) -> TaskHost {
    TaskHost {
        now_ms: ACCEPTANCE_MS,
        now: iso(ACCEPTANCE_MS),
        fresh_id: format!("task-{issued:04}"),
        table: table.to_vec(),
    }
}

/// The effect of the host that stores the table: it completes, and the transcript
/// records the table the core wrote.
fn store_outcome(tag: u32, _payload: &[u8]) -> Vec<u8> {
    assert_eq!(
        tag, STORE_TAG,
        "the core asked for an effect other than the store"
    );
    0u32.to_be_bytes().to_vec()
}

/// The vectors that exercise the one effect the core requests: a task is created (the core
/// stores the table), read back with the table the host holds (the core stores the table it
/// read, as the byte serve returns it for every request that touched a task), and cancelled
/// (the core stores the table again).
pub fn task_vectors() -> Vec<TaskVector> {
    let mut out = Vec::new();
    let mut table: Vec<u8> = Vec::new();
    let sequence = [
        (
            "tools/call",
            json!({"name": "slow_compute", "arguments": {"seconds": 2, "label": "acceptance"}}),
            "slow_compute",
            true,
        ),
        (
            "tasks/get",
            json!({"taskId": "task-0001"}),
            "task-0001",
            true,
        ),
        (
            "tasks/cancel",
            json!({"taskId": "task-0001"}),
            "task-0001",
            true,
        ),
    ];
    for (issued, (method, params, name, stores)) in sequence.into_iter().enumerate() {
        let mut request = tasks_request(method, &params, name);
        request.host = acceptance_host(u32::try_from(issued).expect("few") + 1, &table);
        let frame_request = encode_request(&request);
        let driven = drive_conformance(&frame_request, 1, store_outcome)
            .expect("the conformance root answers the task vector");
        let response = decode_response(&driven.response).expect("a response frame");
        assert!(
            response.tasks.is_none(),
            "the response frame of {method} carries a table"
        );
        assert_eq!(
            driven.performed.len(),
            usize::from(stores),
            "{method} stores the table exactly when it writes one"
        );
        let steps: Vec<Step> = driven
            .performed
            .iter()
            .map(|(_, payload)| Step {
                request: payload.clone(),
                outcome: 0u32.to_be_bytes().to_vec(),
            })
            .collect();
        if let Some((_, payload)) = driven.performed.first() {
            table = payload[4..].to_vec();
        }
        out.push(TaskVector {
            frame_request,
            frame_response: driven.response,
            steps,
        });
    }
    out
}

fn bytes(value: &[u8]) -> String {
    format!("0x{}", hex(value))
}

fn text(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

fn list(item: &str, entries: &[String]) -> String {
    let mut out = String::new();
    for entry in entries {
        out.push_str(&format!("(cons {entry} "));
    }
    out.push_str(&format!("(nil {item})"));
    out.push_str(&")".repeat(entries.len()));
    out
}

fn record(name: &str, fields: &[(&str, String)]) -> String {
    let body: Vec<String> = fields
        .iter()
        .map(|(field, value)| format!("{field} := {value}"))
        .collect();
    format!(
        "(record Foundation.Server.V1.Model::{name} {{{}}})",
        body.join(", ")
    )
}

fn constructor(owner: &str, name: &str) -> String {
    format!("(ctor Foundation.Server.V1.Model::{owner}.{name})")
}

fn word(value: u32) -> String {
    format!("({value} : UInt32)")
}

/// The largest table the conformance host carries in either direction, in bytes.
pub const TABLE_MAXIMUM: u32 = 524_288;

fn section(name: &str, kind: &str, effect: &str, field: &str) -> String {
    record(
        "FrameSection",
        &[
            ("name", text(name)),
            ("kind", constructor("FrameKind", kind)),
            ("effect", text(effect)),
            ("field", text(field)),
            ("values", "(nil String)".to_owned()),
        ],
    )
}

fn effect(
    name: &str,
    request: &[(&str, &str)],
    result: &[(&str, &str)],
    request_maximum: u32,
    result_maximum: u32,
) -> String {
    const MODEL: &str = "Foundation.Server.V1.Model";
    let fields = |entries: &[(&str, &str)]| -> String {
        let items: Vec<String> = entries
            .iter()
            .map(|(field, kind)| {
                record(
                    "HostField",
                    &[
                        ("name", text(field)),
                        ("kind", constructor("HostKind", kind)),
                    ],
                )
            })
            .collect();
        list(&format!("{MODEL}::HostField"), &items)
    };
    record(
        "HostEffect",
        &[
            ("name", text(name)),
            ("request", fields(request)),
            ("result", fields(result)),
            ("failures", "(nil String)".to_owned()),
            ("requestMaximum", word(request_maximum)),
            ("resultMaximum", word(result_maximum)),
        ],
    )
}

/// The module text of `conformance/src/Application.lex.tex`.
pub fn render() -> String {
    const MODEL: &str = "Foundation.Server.V1.Model";
    let vectors: Vec<Vector> = SERVED.iter().map(|id| vector(id)).collect();

    let tasks = task_vectors();
    let mut acceptance: Vec<String> = vectors
        .iter()
        .map(|v| {
            record(
                "AcceptanceVector",
                &[
                    ("request", bytes(&v.frame_request)),
                    ("response", bytes(&v.frame_response)),
                ],
            )
        })
        .collect();
    acceptance.extend(tasks.iter().map(|v| {
        record(
            "AcceptanceVector",
            &[
                ("request", bytes(&v.frame_request)),
                ("response", bytes(&v.frame_response)),
            ],
        )
    }));
    let transcripts: Vec<String> = tasks
        .iter()
        .enumerate()
        .filter(|(_, v)| !v.steps.is_empty())
        .map(|(index, v)| {
            let steps: Vec<String> = v
                .steps
                .iter()
                .map(|step| {
                    record(
                        "HostStep",
                        &[
                            ("effect", text("task.store")),
                            ("request", bytes(&step.request)),
                            ("outcome", bytes(&step.outcome)),
                        ],
                    )
                })
                .collect();
            record(
                "HostTranscript",
                &[
                    (
                        "vector",
                        word(u32::try_from(vectors.len() + index).expect("few vectors")),
                    ),
                    ("steps", list(&format!("{MODEL}::HostStep"), &steps)),
                ],
            )
        })
        .collect();
    let served: Vec<String> = vectors
        .iter()
        .enumerate()
        .map(|(index, v)| {
            record(
                "ServedVector",
                &[
                    ("vector", word(u32::try_from(index).expect("few vectors"))),
                    ("request", bytes(&v.http_request)),
                    ("response", bytes(&v.http_response)),
                ],
            )
        })
        .collect();

    let origins: Vec<String> = ["http", "https"]
        .iter()
        .flat_map(|scheme| {
            ["localhost", "127.0.0.1", "[::1]"].iter().map(move |host| {
                record(
                    "ServingOrigin",
                    &[
                        ("scheme", text(scheme)),
                        ("host", text(host)),
                        ("port", word(0)),
                    ],
                )
            })
        })
        .collect();

    let serving = record(
        "ServerServing",
        &[
            ("address", text("127.0.0.1")),
            ("port", word(0)),
            ("connectionMaximum", word(64)),
            ("readTimeoutMillis", word(10_000)),
            ("rootStackBytes", word(2_097_152)),
            ("requestLineMaximum", word(8192)),
            ("headerCountMaximum", word(100)),
            ("headerBytesMaximum", word(65_536)),
            ("bodyMaximum", word(1_048_576)),
            (
                "origins",
                list(&format!("{MODEL}::ServingOrigin"), &origins),
            ),
            ("accept", "(nil String)".to_owned()),
            ("contentTypes", list("String", &[text("application/json")])),
            ("enforcedMethods", list("String", &[text("POST")])),
            ("countedField", text("Mcp-Method")),
            ("countedValue", text("tools/call")),
            (
                "frame",
                list(
                    &format!("{MODEL}::FrameSection"),
                    &[
                        section("tool_calls", "U32", "tool.calls", "calls"),
                        section("now_ms", "U64", "clock.now", "ms"),
                        section("now", "Text", "clock.now", "iso"),
                        section("fresh_task_id", "Text", "task.id", "id"),
                        section("task_table", "Bytes", "task.table", "table"),
                    ],
                ),
            ),
            ("vectors", list(&format!("{MODEL}::ServedVector"), &served)),
        ],
    );

    let effects = [
        effect(
            "tool.calls",
            &[("counted", "Flag")],
            &[("calls", "U32")],
            8,
            16,
        ),
        effect("clock.now", &[], &[("ms", "U64"), ("iso", "Text")], 8, 64),
        effect("task.id", &[], &[("id", "Text")], 8, 128),
        effect(
            "task.table",
            &[],
            &[("table", "Bytes")],
            8,
            TABLE_MAXIMUM + 8,
        ),
        effect(
            "task.store",
            &[("table", "Bytes")],
            &[],
            TABLE_MAXIMUM + 8,
            8,
        ),
    ];
    let host = record(
        "HostProtocol",
        &[
            ("protocol", text("prismpm/host-effects/1")),
            (
                "description",
                text("The conformance build asks its host for four things that fill the request frame, how many tools/call requests the client has made in the current window (not counting the one being served), the host's clock, an identifier for a task the request may create and the table of tasks the host holds, and for one thing the core requests, that the host store the table of tasks the request changed.\n"),
            ),
            ("usageCode", "(64 : UInt8)".to_owned()),
            ("ioCode", "(74 : UInt8)".to_owned()),
            ("faultCode", "(70 : UInt8)".to_owned()),
            ("effectBudget", word(1)),
            ("effects", list(&format!("{MODEL}::HostEffect"), &effects)),
            (
                "transcripts",
                list(&format!("{MODEL}::HostTranscript"), &transcripts),
            ),
        ],
    );

    let application = record(
        "ServerApplication",
        &[
            ("profile", text("prismpm/server-application/1")),
            ("name", text("MCP Conformance Server")),
            ("cargoName", text("mcp-conformance")),
            ("cargoVersion", text("0.1.0")),
            (
                "cargoDescription",
                text("The MCP 2026-07-28 protocol core with the conformance suite's fixture provider, served over loopback HTTP."),
            ),
            ("cargoRepository", text("https://github.com/UOR-Foundation/MCP")),
            ("cargoHomepage", text("https://github.com/UOR-Foundation/MCP")),
            (
                "core",
                record(
                    "ServerCore",
                    &[
                        ("root", text("UorMcp.ConformanceFixture.conformanceRun")),
                        ("package", text("mcp_conformance_core")),
                        ("version", text("1.0.0")),
                    ],
                ),
            ),
            (
                "acceptanceVectors",
                list(&format!("{MODEL}::AcceptanceVector"), &acceptance),
            ),
            ("requestMaximum", "(2097152 : UInt32)".to_owned()),
            ("responseMaximum", "(4194304 : UInt32)".to_owned()),
            (
                "bindings",
                list(
                    &format!("{MODEL}::ServerBinding"),
                    &[record(
                        "ServerBinding",
                        &[
                            ("path", text("bindings/mcp-binding")),
                            ("crateName", text("mcp-binding")),
                            ("host", text("mcp_binding::Native")),
                        ],
                    )],
                ),
            ),
            ("serving", list(&format!("{MODEL}::ServerServing"), &[serving])),
            ("host", host),
        ],
    );

    format!(
        "\\begin{{lexlean}}{{Application}}\n\\useglossary{{lexlean.std.bool@1.2.0}}\n\\useglossary{{lexlean.std.nat@1.2.0}}\n\\useglossary{{prism.arch@1.0.0}}\n\\importmodule{{Foundation.Bytes}}\n\\importmodule{{{MODEL}}}\n\\title{{Server application model}}\n\\begin{{semanticmodule}}\ndefinition{{\n  name: application,\n  parameters: [],\n  result: {MODEL}::ServerApplication,\n  body: {application}\n}}\n\\end{{semanticmodule}}\n\\end{{lexlean}}\n"
    )
}
