//! LC-16: the root needs no stack that grows with its input.
//!
//! Every test here runs the generated root on `lifecycle_oracle::STACK_BYTES`, which
//! is 2 MiB, the size Rust gives a spawned thread when nothing says otherwise, on a
//! frame built to be as large as the tests can afford in each dimension the core
//! reads: header fields, repeated standard headers, a body object with a hundred
//! thousand members, one very long string, JSON nested as deep as the codec admits,
//! and a very long Base64 parameter header. A stack overflow aborts the process, so a
//! passing run is the evidence; `LIFECYCLE.md` gives the argument that it must pass.
//!
//! The argument is a property of the package, checked in `provenance.rs`: no function
//! of it calls itself except as a loop, and its recorded effects do not include
//! recursion.

mod common;

use common::{call, post_owned, rpc, send, standard, META};
use lifecycle_oracle::{Request, STACK_BYTES};
use serde_json::{json, Value};

/// Header fields a stress request carries besides the standard ones.
const PAD_HEADERS: usize = 50_000;
/// Members of the `arguments` object of the large tool call.
const ARGUMENT_MEMBERS: usize = 100_000;
/// Bytes of the one long text argument.
const LONG_TEXT: usize = 2 * 1024 * 1024;
/// Bytes decoded from the long Base64 parameter header.
const LONG_PARAM: usize = 768 * 1024;
/// The deepest nesting the JSON codec admits (`JsonRead.maxDepth`).
const MAX_DEPTH: usize = 512;

/// A request builder for each shape of input the tests stress.
mod shapes {
    use super::*;

    pub fn echo_headers() -> Vec<(String, String)> {
        standard("tools/call", Some("echo"))
    }

    pub fn many_header_fields() -> Request {
        let mut headers = echo_headers();
        for i in 0..PAD_HEADERS {
            headers.push((format!("X-Pad-{i}"), "v".to_owned()));
        }
        post_owned(&headers, &call("echo", "{}", "1"))
    }

    pub fn repeated_standard_header() -> Request {
        let mut headers = echo_headers();
        for _ in 0..PAD_HEADERS {
            headers.push(("MCP-Protocol-Version".to_owned(), "2026-07-28".to_owned()));
        }
        post_owned(&headers, &call("echo", "{}", "1"))
    }

    pub fn many_arguments() -> Request {
        let mut arguments = String::from("{");
        for i in 0..ARGUMENT_MEMBERS {
            if i > 0 {
                arguments.push(',');
            }
            arguments.push_str(&format!(r#""k{i}":"v{i}""#));
        }
        arguments.push('}');
        post_owned(&echo_headers(), &call("echo", &arguments, "1"))
    }

    pub fn long_text() -> Request {
        let text = "a".repeat(LONG_TEXT);
        let arguments = format!(r#"{{"query":"{text}"}}"#);
        post_owned(&echo_headers(), &call("echo", &arguments, "1"))
    }

    /// An array nested `levels` deep inside `params`, which the core ignores.
    pub fn nested(levels: usize) -> Request {
        let nested = format!("{}{}", "[".repeat(levels), "]".repeat(levels));
        let body = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"echo","arguments":{{}},"_meta":{META},"deep":{nested}}}}}"#
        );
        post_owned(&echo_headers(), &body)
    }

    pub fn long_base64_parameter() -> Request {
        // execute_sql mirrors `region` in Mcp-Param-Region.
        let region = "r".repeat(LONG_PARAM);
        const ALPHABET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut encoded = String::new();
        for chunk in region.as_bytes().chunks(3) {
            let n =
                chunk.iter().fold(0u32, |a, b| (a << 8) | u32::from(*b)) << (8 * (3 - chunk.len()));
            for k in 0..=chunk.len() {
                encoded.push(ALPHABET[((n >> (18 - 6 * k)) & 63) as usize] as char);
            }
            for _ in chunk.len()..3 {
                encoded.push('=');
            }
        }
        let mut headers = standard("tools/call", Some("execute_sql"));
        headers.push((
            "Mcp-Param-Region".to_owned(),
            format!("=?base64?{encoded}?="),
        ));
        let body = rpc(
            "tools/call",
            &format!(
                r#"{{"name":"execute_sql","arguments":{{"region":"{region}"}},"_meta":{META}}}"#
            ),
            "1",
        );
        post_owned(&headers, &body)
    }

    pub fn padded_tools_list() -> Request {
        let mut headers = standard("tools/list", None);
        for i in 0..PAD_HEADERS {
            headers.push((format!("X-Pad-{i}"), "v".to_owned()));
        }
        post_owned(
            &headers,
            &rpc("tools/list", &format!(r#"{{"_meta":{META}}}"#), "1"),
        )
    }
}

fn run(request: &Request) -> (u32, Value) {
    let (response, json) = send(request);
    (response.status, json)
}

fn assert_ok(status: u32, body: &Value, what: &str) {
    assert_eq!(status, 200, "{what}: {body:.300}");
    assert!(body.get("result").is_some(), "{what}");
}

#[test]
fn the_stack_is_the_default_thread_stack_lc_16() {
    assert_eq!(STACK_BYTES, 2 * 1024 * 1024);
}

#[test]
fn fifty_thousand_header_fields_are_read_on_the_default_stack_lc_16() {
    let (status, body) = run(&shapes::many_header_fields());
    assert_ok(status, &body, "many header fields");
}

#[test]
fn fifty_thousand_repeats_of_a_standard_header_are_one_refusal_lc_16() {
    let (status, body) = run(&shapes::repeated_standard_header());
    assert_eq!((status, &body["error"]["code"]), (400, &json!(-32020)));
}

#[test]
fn an_arguments_object_of_a_hundred_thousand_members_is_decoded_lc_16() {
    let (status, body) = run(&shapes::many_arguments());
    assert_ok(status, &body, "many arguments");
}

#[test]
fn a_two_mebibyte_text_argument_is_decoded_lc_16() {
    let (status, body) = run(&shapes::long_text());
    assert_ok(status, &body, "long text");
}

#[test]
fn json_nested_to_the_deepest_depth_the_codec_admits_is_read_and_dropped_lc_16() {
    // the object that holds `params` is level 1, `params` level 2, so an array nested
    // MAX_DEPTH - 2 deep inside `params` reaches level MAX_DEPTH.
    let (status, json) = run(&shapes::nested(MAX_DEPTH - 2));
    assert_ok(status, &json, "deepest admitted nesting");
    // one level deeper is refused by the codec, as a typed 400, not by the stack
    let (status, json) = run(&shapes::nested(MAX_DEPTH - 1));
    assert_eq!(status, 400, "{json:.300}");
    assert_eq!(json["error"]["code"], -32700, "{json:.300}");
}

#[test]
fn a_long_base64_parameter_header_is_decoded_and_compared_lc_16() {
    let (status, json) = run(&shapes::long_base64_parameter());
    assert_ok(status, &json, "long Base64 parameter header");
}

#[test]
fn a_tools_list_with_a_padded_header_set_answers_on_the_default_stack_lc_16() {
    let (status, body) = run(&shapes::padded_tools_list());
    assert_ok(status, &body, "tools/list with many header fields");
}

/// How little stack the stress frames need: run with `STACK_PROBE_BYTES=<n>`, for
/// example
/// `STACK_PROBE_BYTES=65536 cargo test -p lifecycle-oracle --test stress -- --ignored`.
/// A stack that is too small aborts the process, so the probe is run once per size,
/// outside the suite.
#[test]
#[ignore = "a measurement: run once per stack size, an overflow aborts the process"]
fn the_stack_floor_probe() {
    let bytes: usize = std::env::var("STACK_PROBE_BYTES")
        .expect("STACK_PROBE_BYTES names the stack in bytes")
        .parse()
        .expect("a number");
    let frames = [
        ("many_header_fields", shapes::many_header_fields()),
        (
            "repeated_standard_header",
            shapes::repeated_standard_header(),
        ),
        ("many_arguments", shapes::many_arguments()),
        ("long_text", shapes::long_text()),
        ("nested", shapes::nested(MAX_DEPTH - 2)),
        ("long_base64_parameter", shapes::long_base64_parameter()),
        ("padded_tools_list", shapes::padded_tools_list()),
    ];
    for (name, request) in frames {
        let frame = lifecycle_oracle::encode_request(&request);
        let out = lifecycle_oracle::exchange_frame_on(&frame, bytes);
        eprintln!(
            "{name}: {} response bytes on {bytes} bytes of stack",
            out.len()
        );
    }
}
