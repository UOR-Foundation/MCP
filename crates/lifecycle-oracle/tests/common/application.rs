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

use lifecycle_oracle::{decode_response, encode_request, exchange_frame, Request};
use serde_json::Value;

use super::{
    hex, request_of, resources_vector_file, tools_vector_file, transport_vector_file, vector_file,
};

/// The vectors the declaration serves, by the ID of the vector file they come from.
/// Each is a request the pinned scenarios send, so the declaration's vectors exercise
/// discovery, the tools, resources and prompts, an event stream and two refusals.
pub const SERVED: [&str; 11] = [
    "ST-05", "TS-01", "TS-02", "TS-05", "TS-08", "SP-01", "RS-01", "RS-02", "RS-09", "ST-01",
    "ST-14",
];

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
    let frame_response = exchange_frame(&frame_request);
    let response = decode_response(&frame_response).expect("the core writes a response frame");
    assert!(
        response.notifications.is_empty(),
        "the response frame of {id} carries notifications"
    );

    let mut http_response =
        format!("HTTP/1.1 {} {}\r\n", response.status, reason(response.status)).into_bytes();
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

/// The module text of `conformance/src/Application.lex.tex`.
pub fn render() -> String {
    const MODEL: &str = "Foundation.Server.V1.Model";
    let vectors: Vec<Vector> = SERVED.iter().map(|id| vector(id)).collect();

    let acceptance: Vec<String> = vectors
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
            ("origins", list(&format!("{MODEL}::ServingOrigin"), &origins)),
            ("accept", "(nil String)".to_owned()),
            (
                "contentTypes",
                list("String", &[text("application/json")]),
            ),
            ("enforcedMethods", list("String", &[text("POST")])),
            ("countedField", text("Mcp-Method")),
            ("countedValue", text("tools/call")),
            (
                "frame",
                list(
                    &format!("{MODEL}::FrameSection"),
                    &[record(
                        "FrameSection",
                        &[
                            ("name", text("tool_calls")),
                            ("kind", constructor("FrameKind", "U32")),
                            ("effect", text("tool.calls")),
                            ("field", text("calls")),
                            ("values", "(nil String)".to_owned()),
                        ],
                    )],
                ),
            ),
            ("vectors", list(&format!("{MODEL}::ServedVector"), &served)),
        ],
    );

    let effect = record(
        "HostEffect",
        &[
            ("name", text("tool.calls")),
            (
                "request",
                list(
                    &format!("{MODEL}::HostField"),
                    &[record(
                        "HostField",
                        &[
                            ("name", text("counted")),
                            ("kind", constructor("HostKind", "Flag")),
                        ],
                    )],
                ),
            ),
            (
                "result",
                list(
                    &format!("{MODEL}::HostField"),
                    &[record(
                        "HostField",
                        &[
                            ("name", text("calls")),
                            ("kind", constructor("HostKind", "U32")),
                        ],
                    )],
                ),
            ),
            ("failures", "(nil String)".to_owned()),
            ("requestMaximum", word(8)),
            ("resultMaximum", word(16)),
        ],
    );
    let host = record(
        "HostProtocol",
        &[
            ("protocol", text("prismpm/host-effects/1")),
            (
                "description",
                text("The conformance build asks its host for one thing: how many tools/call requests the client has made in the current window, not counting the one being served.\n"),
            ),
            ("usageCode", "(64 : UInt8)".to_owned()),
            ("ioCode", "(74 : UInt8)".to_owned()),
            ("faultCode", "(70 : UInt8)".to_owned()),
            ("effectBudget", word(0)),
            ("effects", list(&format!("{MODEL}::HostEffect"), &[effect])),
            ("transcripts", format!("(nil {MODEL}::HostTranscript)")),
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
                        ("root", text("UorMcp.Fixture.fixtureRun")),
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
