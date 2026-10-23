//! Resources, resource templates, prompts, completion and caching, run against the
//! generated lifecycle root: the catalogs and their order, what a read or a prompt
//! returns, the refusals, the completion of arguments, the cache hints and the
//! capabilities the fixture declares.
//!
//! The requests the pinned scenarios send are the vectors of `vectors/resources.json`;
//! the tests here add the requests the scenarios do not send. Each is a falsifying
//! probe authored from the specification text the register cites, never a verdict.

mod common;

use common::{
    crowd_checked, header, post_owned, root, rpc, run_register, send, standard, suite_text, META,
};
use lifecycle_oracle::{Request, Response};
use serde_json::{json, Value};

/// The uris of the fixture's resources, in the order `ResourceFixture.staticEntries`
/// declares them.
const URIS: [&str; 4] = [
    "test://static-text",
    "test://static-binary",
    "test://watched-resource",
    "test://stateless-static-text",
];

/// The prompts of the fixture, in the order `ResourceFixture.fixturePrompts` declares them.
const PROMPTS: [&str; 4] = [
    "test_simple_prompt",
    "test_prompt_with_arguments",
    "test_prompt_with_embedded_resource",
    "test_prompt_with_image",
];

const NOT_FOUND_URI: &str = "test://nonexistent-resource-for-conformance-testing";

fn source(name: &str) -> String {
    std::fs::read_to_string(root().join("src").join(format!("{name}.lex.tex")))
        .unwrap_or_else(|e| panic!("src/{name}.lex.tex reads: {e}"))
}

fn vendored_constant(name: &str) -> String {
    let text = suite_text("examples/servers/typescript/everything-server.ts");
    let at = text
        .find(&format!("const {name} ="))
        .unwrap_or_else(|| panic!("{name} is in the reference fixture"));
    let rest = &text[at..];
    let open = rest.find('\'').expect("a quoted value") + 1;
    let close = open + rest[open..].find('\'').expect("a closing quote");
    rest[open..close].to_owned()
}

/// A request of `method` with `params` (the members, without `_meta`), the standard
/// headers and `name` as `Mcp-Name` when the method carries one.
fn request(method: &str, params: &Value, name: Option<&str>, id: &str) -> Request {
    let mut members = params.as_object().expect("params are an object").clone();
    members.insert(
        "_meta".to_owned(),
        serde_json::from_str(META).expect("META is JSON"),
    );
    post_owned(
        &standard(method, name),
        &rpc(method, &Value::Object(members).to_string(), id),
    )
}

fn list(method: &str) -> Request {
    request(method, &json!({}), None, "1")
}

fn list_with_cursor(method: &str) -> Request {
    request(method, &json!({"cursor": "page-2"}), None, "1")
}

fn read(uri: &str) -> Request {
    request("resources/read", &json!({"uri": uri}), Some(uri), "2")
}

fn get(name: &str, arguments: &Value) -> Request {
    request(
        "prompts/get",
        &json!({"name": name, "arguments": arguments}),
        Some(name),
        "3",
    )
}

fn get_without_arguments(name: &str) -> Request {
    request("prompts/get", &json!({"name": name}), Some(name), "3")
}

fn complete(reference: &Value, argument: &str, typed: &str) -> Request {
    request(
        "completion/complete",
        &json!({"ref": reference, "argument": {"name": argument, "value": typed}}),
        None,
        "4",
    )
}

fn prompt_ref(name: &str) -> Value {
    json!({"type": "ref/prompt", "name": name})
}

fn template_ref() -> Value {
    json!({"type": "ref/resource", "uri": "test://template/{id}/data"})
}

/// The answer's `result`, after the checks every success shares.
fn result_of(request: &Request) -> Value {
    let (response, body) = send(request);
    assert_eq!(response.status, 200, "{body}");
    assert_eq!(body["result"]["resultType"], "complete", "{body}");
    assert!(body.get("error").is_none(), "{body}");
    body["result"].clone()
}

/// The refusal's `error`, after the checks every refusal shares: HTTP 400, code -32602,
/// the request id echoed and no result.
fn refusal_of(request: &Request, id: u64) -> Value {
    let (response, body) = send(request);
    assert_eq!(response.status, 400, "{body}");
    assert_eq!(body["error"]["code"], -32602, "{body}");
    assert_eq!(body["id"], id, "{body}");
    assert!(body.get("result").is_none(), "{body}");
    assert_eq!(
        header(&response, "content-type"),
        Some(&b"application/json"[..])
    );
    body["error"].clone()
}

fn assert_canonical_base64(text: &str) {
    assert!(!text.is_empty());
    assert_eq!(text.len() % 4, 0, "{text}");
    assert!(
        text.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'+' || b == b'/' || b == b'='),
        "{text}"
    );
}

fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |acc, (i, b)| acc | (u32::from(*b) << (16 - 8 * i)));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(ALPHABET[((n >> (18 - 6 * i)) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

// ---- RP-01: resources/list ----

#[test]
fn resources_list_answers_the_catalog_in_its_declared_order_rp_01() {
    assert_eq!(run_register("RP-01"), 1);
    let result = result_of(&list("resources/list"));
    let resources = result["resources"].as_array().expect("a resources array");
    let uris: Vec<&str> = resources
        .iter()
        .map(|r| r["uri"].as_str().expect("a uri"))
        .collect();
    assert_eq!(uris, URIS);
    let names: Vec<&str> = resources
        .iter()
        .map(|r| r["name"].as_str().expect("a name"))
        .collect();
    assert_eq!(
        names,
        [
            "static-text",
            "static-binary",
            "watched-resource",
            "Stateless Static Text"
        ]
    );
    for resource in resources {
        assert!(resource["description"].is_string(), "{resource}");
        assert!(resource["mimeType"].is_string(), "{resource}");
        assert!(
            !resource["uri"].as_str().expect("uri").contains('{'),
            "a template is not a direct resource: {resource}"
        );
    }
    assert_eq!(resources[0]["title"], "Static Text Resource");
    assert!(resources[3].get("title").is_none());
    assert!(result.get("nextCursor").is_none(), "{result}");
}

#[test]
fn the_listed_resources_are_those_of_the_reference_fixture_rp_01() {
    let fixture = suite_text("examples/servers/typescript/everything-server.ts");
    let result = result_of(&list("resources/list"));
    for resource in result["resources"].as_array().expect("resources") {
        let description = resource["description"].as_str().expect("description");
        let uri = resource["uri"].as_str().expect("uri");
        assert!(
            fixture.contains(&format!("'{description}'")),
            "{description} is not a description of the reference fixture"
        );
        assert!(
            fixture.contains(&format!("'{uri}'")),
            "{uri} is not a uri of the reference fixture"
        );
        assert!(
            fixture.contains(&format!(
                "'{}'",
                resource["mimeType"].as_str().expect("mime")
            )),
            "{resource}"
        );
    }
}

#[test]
fn the_order_does_not_change_between_requests_rp_01() {
    let first = common::exchange_checked(&list("resources/list")).expect("a frame");
    for _ in 0..3 {
        let again = common::exchange_checked(&list("resources/list")).expect("a frame");
        assert_eq!(first.body, again.body);
    }
}

// ---- RP-02, RP-03: reading text and binary resources ----

#[test]
fn a_text_resource_reads_as_text_with_its_uri_and_mime_type_rp_02() {
    assert_eq!(run_register("RP-02"), 1);
    let fixture = suite_text("examples/servers/typescript/everything-server.ts");
    let result = result_of(&read("test://static-text"));
    let contents = result["contents"].as_array().expect("contents");
    assert_eq!(contents.len(), 1);
    let text = contents[0]["text"].as_str().expect("text");
    assert!(fixture.contains(&format!("'{text}'")), "{text}");
    assert!(contents[0].get("blob").is_none());
    // the other text resources
    let watched = result_of(&read("test://watched-resource"));
    assert_eq!(
        watched["contents"][0]["text"],
        vendored_constant("watchedResourceContent")
    );
    assert_eq!(watched["contents"][0]["mimeType"], "text/plain");
    let stateless = result_of(&read("test://stateless-static-text"));
    let text = stateless["contents"][0]["text"].as_str().expect("text");
    assert!(fixture.contains(&format!("'{text}'")), "{text}");
}

#[test]
fn a_binary_resource_reads_as_a_canonical_base64_blob_rp_03() {
    assert_eq!(run_register("RP-03"), 1);
    let result = result_of(&read("test://static-binary"));
    let entry = &result["contents"][0];
    assert_eq!(entry["blob"], vendored_constant("TEST_IMAGE_BASE64"));
    assert_canonical_base64(entry["blob"].as_str().expect("blob"));
    assert!(entry.get("text").is_none());
    assert_eq!(entry["mimeType"], "image/png");
    assert_eq!(entry["uri"], "test://static-binary");
}

// ---- RP-04: resources/templates/list ----

#[test]
fn templates_list_answers_the_templates_of_the_catalog_rp_04() {
    assert_eq!(run_register("RP-04"), 1);
    let fixture = suite_text("examples/servers/typescript/everything-server.ts");
    let result = result_of(&list("resources/templates/list"));
    let templates = result["resourceTemplates"].as_array().expect("templates");
    assert_eq!(templates.len(), 1);
    let template = &templates[0];
    assert_eq!(template["name"], "template");
    assert_eq!(template["title"], "Resource Template");
    assert_eq!(template["uriTemplate"], "test://template/{id}/data");
    assert_eq!(template["mimeType"], "application/json");
    let description = template["description"].as_str().expect("description");
    assert!(
        fixture.contains(&format!("'{description}'")),
        "{description}"
    );
    assert!(fixture.contains("new ResourceTemplate('test://template/{id}/data'"));
    assert!(result.get("nextCursor").is_none(), "{result}");
}

// ---- RP-05: reading through a template ----

#[test]
fn a_template_read_substitutes_the_variable_rp_05() {
    assert_eq!(run_register("RP-05"), 1);
    for id in ["123", "abc-9", "a b", "x.y_z", "é", "a\"b\\c"] {
        let uri = format!("test://template/{id}/data");
        // a uri with characters a header cannot carry travels as the Base64 sentinel
        let name = if id.is_ascii() && !id.contains(['"', '\\']) {
            uri.clone()
        } else {
            format!("=?base64?{}?=", base64(uri.as_bytes()))
        };
        let result = result_of(&request(
            "resources/read",
            &json!({"uri": uri}),
            Some(&name),
            "5",
        ));
        let entry = &result["contents"][0];
        assert_eq!(entry["uri"], uri);
        assert_eq!(entry["mimeType"], "application/json");
        let text: Value = serde_json::from_str(entry["text"].as_str().expect("text"))
            .expect("the text is a JSON text");
        assert_eq!(
            text,
            json!({"id": id, "templateTest": true, "data": format!("Data for ID: {id}")}),
            "{uri}"
        );
    }
}

#[test]
fn a_uri_the_template_does_not_match_is_not_found_rp_05() {
    for uri in [
        "test://template//data",
        "test://template/a/b/data",
        "test://template/123/dat",
        "test://template/123/data/",
        "test://template/123",
        "test://template/data",
        "test://template/",
        "test://templat/123/data",
        "test://other/123/data",
        "TEST://template/123/data",
        "xtest://template/123/data",
    ] {
        let error = refusal_of(&read(uri), 2);
        assert_eq!(error["data"]["uri"], uri, "{uri}");
    }
}

// ---- RP-06: the resource that does not exist ----

#[test]
fn an_unknown_resource_is_invalid_params_with_its_uri_in_the_data_rp_06() {
    assert_eq!(run_register("RP-06"), 1);
    let error = refusal_of(&read(NOT_FOUND_URI), 2);
    assert_eq!(error["message"], "Invalid params: resource not found");
    assert_eq!(error["data"], json!({"uri": NOT_FOUND_URI}));
}

#[test]
fn an_unknown_resource_is_never_answered_with_empty_contents_rp_06() {
    for uri in [
        NOT_FOUND_URI,
        "file:///etc/passwd",
        "test://static-text/",
        "test://STATIC-TEXT",
        "TEST://static-text",
        "x",
    ] {
        let (response, body) = send(&read(uri));
        assert_eq!(response.status, 400, "{uri}: {body}");
        assert!(body.get("result").is_none(), "{uri}: {body}");
        assert_eq!(body["error"]["data"]["uri"], uri);
    }
}

#[test]
fn an_unknown_member_of_a_read_is_ignored_rp_06() {
    // a read carries no cursor; an unknown member is not a refusal
    let (response, body) = send(&request(
        "resources/read",
        &json!({"uri": "test://static-text", "cursor": "x"}),
        Some("test://static-text"),
        "6",
    ));
    assert_eq!(response.status, 200, "{body}");
}

// ---- RP-07: prompts/list ----

#[test]
fn prompts_list_answers_the_catalog_in_its_declared_order_rp_07() {
    assert_eq!(run_register("RP-07"), 1);
    let fixture = suite_text("examples/servers/typescript/everything-server.ts");
    let result = result_of(&list("prompts/list"));
    let prompts = result["prompts"].as_array().expect("prompts");
    let names: Vec<&str> = prompts
        .iter()
        .map(|p| p["name"].as_str().expect("name"))
        .collect();
    assert_eq!(names, PROMPTS);
    for prompt in prompts {
        let description = prompt["description"].as_str().expect("description");
        assert!(
            fixture.contains(&format!("'{description}'")),
            "{description}"
        );
        let title = prompt["title"].as_str().expect("title");
        assert!(fixture.contains(&format!("'{title}'")), "{title}");
    }
    assert!(prompts[0].get("arguments").is_none());
    assert!(prompts[3].get("arguments").is_none());
    assert_eq!(
        prompts[1]["arguments"],
        json!([
            {"name": "arg1", "description": "First test argument", "required": true},
            {"name": "arg2", "description": "Second test argument", "required": true}
        ])
    );
    assert_eq!(
        prompts[2]["arguments"],
        json!([{"name": "resourceUri", "description": "URI of the resource to embed", "required": true}])
    );
    assert!(result.get("nextCursor").is_none(), "{result}");
}

// ---- RP-08 .. RP-11: prompts/get ----

#[test]
fn a_simple_prompt_answers_one_user_text_message_rp_08() {
    assert_eq!(run_register("RP-08"), 1);
    let result = result_of(&get_without_arguments("test_simple_prompt"));
    assert_eq!(
        result["messages"],
        json!([{"role": "user", "content": {"type": "text", "text": "This is a simple prompt for testing."}}])
    );
    assert!(result.get("description").is_none());
    // arguments the prompt does not declare are ignored
    let again = result_of(&get("test_simple_prompt", &json!({"unused": "x"})));
    assert_eq!(again["messages"], result["messages"]);
}

#[test]
fn the_arguments_given_are_substituted_into_the_messages_rp_09() {
    assert_eq!(run_register("RP-09"), 1);
    for (arg1, arg2) in [
        ("hello", "world"),
        ("it's", "a \"quote\""),
        ("é", "日本語"),
        ("", ""),
    ] {
        let result = result_of(&get(
            "test_prompt_with_arguments",
            &json!({"arg1": arg1, "arg2": arg2}),
        ));
        assert_eq!(
            result["messages"][0]["content"]["text"],
            format!("Prompt with arguments: arg1='{arg1}', arg2='{arg2}'")
        );
        assert_eq!(result["messages"].as_array().expect("messages").len(), 1);
    }
    // an argument the prompt does not declare is ignored
    let result = result_of(&get(
        "test_prompt_with_arguments",
        &json!({"arg1": "a", "arg2": "b", "arg3": "c"}),
    ));
    assert_eq!(
        result["messages"][0]["content"]["text"],
        "Prompt with arguments: arg1='a', arg2='b'"
    );
}

#[test]
fn an_embedded_resource_carries_the_uri_that_was_given_rp_10() {
    assert_eq!(run_register("RP-10"), 1);
    for uri in ["test://example-resource", "file:///tmp/a b", "x"] {
        let result = result_of(&get(
            "test_prompt_with_embedded_resource",
            &json!({"resourceUri": uri}),
        ));
        let messages = result["messages"].as_array().expect("messages");
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0]["role"], "user");
        assert_eq!(messages[0]["content"]["type"], "resource");
        assert_eq!(messages[0]["content"]["resource"]["uri"], uri);
        assert_eq!(messages[0]["content"]["resource"]["mimeType"], "text/plain");
        assert_eq!(messages[1]["content"]["type"], "text");
    }
}

#[test]
fn an_image_prompt_answers_the_reference_image_and_an_instruction_rp_11() {
    assert_eq!(run_register("RP-11"), 1);
    let result = result_of(&get_without_arguments("test_prompt_with_image"));
    let messages = result["messages"].as_array().expect("messages");
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0]["content"]["type"], "image");
    assert_eq!(
        messages[0]["content"]["data"],
        vendored_constant("TEST_IMAGE_BASE64")
    );
    assert_canonical_base64(messages[0]["content"]["data"].as_str().expect("data"));
    assert_eq!(messages[0]["content"]["mimeType"], "image/png");
    assert_eq!(messages[1]["role"], "user");
}

// ---- RP-12: unknown prompts and missing arguments ----

#[test]
fn an_unknown_prompt_is_invalid_params_rp_12() {
    for name in ["no_such_prompt", "TEST_SIMPLE_PROMPT", "test_simple_promp"] {
        let error = refusal_of(&get_without_arguments(name), 3);
        assert_eq!(error["message"], "Invalid params: unknown prompt", "{name}");
    }
}

#[test]
fn a_missing_required_argument_is_invalid_params_and_is_named_rp_12() {
    let cases = [
        ("test_prompt_with_arguments", json!({}), "arg1"),
        ("test_prompt_with_arguments", json!({"arg1": "a"}), "arg2"),
        ("test_prompt_with_arguments", json!({"arg2": "b"}), "arg1"),
        ("test_prompt_with_arguments", json!({"other": "x"}), "arg1"),
        (
            "test_prompt_with_embedded_resource",
            json!({}),
            "resourceUri",
        ),
    ];
    for (name, arguments, missing) in cases {
        let error = refusal_of(&get(name, &arguments), 3);
        assert_eq!(error["message"], "Invalid params: invalid arguments");
        assert_eq!(
            error["data"]["reason"],
            format!("missing required argument: {missing}"),
            "{name} {arguments}"
        );
    }
    // no arguments member at all
    let error = refusal_of(&get_without_arguments("test_prompt_with_arguments"), 3);
    assert_eq!(error["data"]["reason"], "missing required argument: arg1");
}

#[test]
fn a_prompt_name_or_arguments_of_the_wrong_type_are_invalid_params_rp_12() {
    let bad = [
        request("prompts/get", &json!({"name": 5}), Some("5"), "3"),
        request(
            "prompts/get",
            &json!({"name": "x", "arguments": "y"}),
            Some("x"),
            "3",
        ),
        request(
            "prompts/get",
            &json!({"name": "test_prompt_with_arguments", "arguments": {"arg1": 1, "arg2": "b"}}),
            Some("test_prompt_with_arguments"),
            "3",
        ),
    ];
    for request in bad {
        refusal_of(&request, 3);
    }
}

// ---- RP-13, RP-14: completion ----

#[test]
fn completion_answers_the_candidates_that_begin_with_the_typed_text_rp_13() {
    assert_eq!(run_register("RP-13"), 1);
    let cases: [(Value, &str, &str, Vec<&str>); 8] = [
        (
            prompt_ref("test_prompt_with_arguments"),
            "arg1",
            "par",
            vec![],
        ),
        (
            prompt_ref("test_prompt_with_embedded_resource"),
            "resourceUri",
            "test://st",
            vec![
                "test://static-text",
                "test://static-binary",
                "test://stateless-static-text",
            ],
        ),
        (
            prompt_ref("test_prompt_with_embedded_resource"),
            "resourceUri",
            "",
            URIS.to_vec(),
        ),
        (
            prompt_ref("test_prompt_with_embedded_resource"),
            "resourceUri",
            "zzz",
            vec![],
        ),
        (template_ref(), "id", "12", vec!["123", "124"]),
        (template_ref(), "id", "4", vec!["456"]),
        (template_ref(), "id", "5", vec![]),
        (template_ref(), "id", "é", vec![]),
    ];
    for (reference, argument, typed, expected) in cases {
        let result = result_of(&complete(&reference, argument, typed));
        let completion = &result["completion"];
        assert_eq!(completion["values"], json!(expected), "{reference} {typed}");
        assert_eq!(completion["total"], expected.len(), "{reference} {typed}");
        assert_eq!(completion["hasMore"], false);
    }
}

#[test]
fn the_context_of_a_completion_is_read_and_does_not_change_the_answer_rp_13() {
    let with = request(
        "completion/complete",
        &json!({
            "ref": template_ref(),
            "argument": {"name": "id", "value": "12"},
            "context": {"arguments": {"other": "x"}}
        }),
        None,
        "4",
    );
    let result = result_of(&with);
    assert_eq!(result["completion"]["values"], json!(["123", "124"]));
}

#[test]
fn a_completion_for_what_the_catalog_does_not_list_is_invalid_params_rp_14() {
    let unknown = [
        complete(&prompt_ref("no_such_prompt"), "arg1", "x"),
        complete(&prompt_ref("test_prompt_with_arguments"), "arg9", "x"),
        complete(&prompt_ref("test_simple_prompt"), "arg1", "x"),
        complete(
            &json!({"type": "ref/resource", "uri": "test://other/{id}"}),
            "id",
            "x",
        ),
        complete(&template_ref(), "other", "x"),
    ];
    for request in unknown {
        refusal_of(&request, 4);
    }
}

#[test]
fn a_malformed_completion_is_invalid_params_rp_14() {
    let params = [
        json!({"argument": {"name": "arg1", "value": "x"}}),
        json!({"ref": prompt_ref("test_prompt_with_arguments")}),
        json!({"ref": {"type": "ref/other", "name": "x"}, "argument": {"name": "a", "value": "x"}}),
        json!({"ref": {"name": "x"}, "argument": {"name": "a", "value": "x"}}),
        json!({"ref": {"type": "ref/prompt"}, "argument": {"name": "a", "value": "x"}}),
        json!({"ref": prompt_ref("test_prompt_with_arguments"), "argument": {"name": "arg1"}}),
        json!({"ref": prompt_ref("test_prompt_with_arguments"), "argument": {"name": "arg1", "value": 5}}),
        json!({"ref": prompt_ref("test_prompt_with_arguments"), "argument": {"name": "arg1", "value": "x"}, "context": 5}),
        json!({"ref": prompt_ref("test_prompt_with_arguments"), "argument": {"name": "arg1", "value": "x"}, "context": {"arguments": {"a": 1}}}),
    ];
    for params in params {
        refusal_of(&request("completion/complete", &params, None, "4"), 4);
    }
}

// ---- RP-15: caching ----

#[test]
fn the_five_cacheable_results_carry_a_ttl_and_a_scope_rp_15() {
    assert_eq!(run_register("RP-15"), 5);
    for method in [
        "tools/list",
        "prompts/list",
        "resources/list",
        "resources/templates/list",
    ] {
        let result = result_of(&list(method));
        assert!(result["ttlMs"].is_u64(), "{method}: {result}");
        let scope = result["cacheScope"].as_str().expect("a scope");
        assert!(scope == "public" || scope == "private", "{method}");
    }
    for uri in URIS {
        let result = result_of(&read(uri));
        assert!(result["ttlMs"].is_u64(), "{uri}");
        assert!(
            ["public", "private"].contains(&result["cacheScope"].as_str().expect("scope")),
            "{uri}"
        );
    }
}

#[test]
fn lists_are_public_and_reads_are_private_for_five_minutes_rp_15() {
    for method in ["prompts/list", "resources/list", "resources/templates/list"] {
        let result = result_of(&list(method));
        assert_eq!(result["ttlMs"], 300_000, "{method}");
        assert_eq!(result["cacheScope"], "public", "{method}");
    }
    for uri in URIS {
        let result = result_of(&read(uri));
        assert_eq!(result["ttlMs"], 300_000, "{uri}");
        assert_eq!(result["cacheScope"], "private", "{uri}");
    }
    let templated = result_of(&read("test://template/123/data"));
    assert_eq!(templated["ttlMs"], 300_000);
    assert_eq!(templated["cacheScope"], "private");
}

#[test]
fn a_refusal_and_an_uncacheable_result_carry_no_cache_hint_rp_15() {
    let (_, body) = send(&read(NOT_FOUND_URI));
    assert!(body["error"].get("ttlMs").is_none());
    assert!(body.get("ttlMs").is_none() && body.get("cacheScope").is_none());
    for request in [
        get_without_arguments("test_simple_prompt"),
        complete(&prompt_ref("test_prompt_with_arguments"), "arg1", "x"),
    ] {
        let result = result_of(&request);
        assert!(result.get("ttlMs").is_none(), "{result}");
        assert!(result.get("cacheScope").is_none(), "{result}");
    }
}

// ---- RP-16: one page ----

#[test]
fn a_cursor_on_a_list_is_invalid_params_rp_16() {
    for method in ["resources/list", "resources/templates/list", "prompts/list"] {
        let error = refusal_of(&list_with_cursor(method), 1);
        assert_eq!(
            error["message"], "Invalid params: invalid cursor",
            "{method}"
        );
    }
}

#[test]
fn a_cursor_of_the_wrong_type_is_invalid_params_rp_16() {
    for method in ["resources/list", "resources/templates/list", "prompts/list"] {
        let request = request(method, &json!({"cursor": 5}), None, "1");
        refusal_of(&request, 1);
    }
}

// ---- RP-17: the theorems ----

#[test]
fn the_named_theorems_of_the_resources_layer_are_declared_rp_17() {
    let laws = source("ResourceLaws");
    for name in [
        "the_catalog_lists_its_resources_in_the_declared_order",
        "resources_list_answers_with_the_catalog_in_its_declared_order",
        "templates_list_answers_with_the_catalog_in_its_declared_order",
        "prompts_list_answers_with_the_catalog_in_its_declared_order",
        "an_unknown_resource_is_refused_as_not_found",
        "an_unknown_resource_has_the_invalid_params_code",
        "a_resource_without_contents_is_never_answered_with_an_empty_array",
        "an_unknown_prompt_is_refused_as_unknown_prompt",
        "a_prompt_called_without_its_arguments_is_invalid_params",
        "the_resource_list_carries_the_list_cache_hint",
        "a_read_carries_the_read_cache_hint",
        "the_five_cacheable_results_are_cacheable",
        "the_encoded_resource_list_carries_the_cache_hint",
        "the_encoded_read_carries_the_cache_hint",
        "a_completion_is_cut_at_one_hundred_values",
        "a_cut_completion_counts_what_it_cut_and_says_there_is_more",
        "the_fixture_offers_resources_prompts_and_completion",
    ] {
        assert!(
            laws.contains(&format!("name: {name},")),
            "ResourceLaws declares no theorem {name}"
        );
    }
    let refusal = source("Refusal");
    for name in [
        "a_missing_resource_has_the_invalid_params_code",
        "a_missing_resource_names_its_uri",
        "an_unknown_prompt_has_the_invalid_params_code",
        "an_invalid_prompt_argument_has_the_invalid_params_code",
    ] {
        assert!(
            refusal.contains(&format!("name: {name},")),
            "Refusal declares no theorem {name}"
        );
    }
    let status = source("Status");
    for name in [
        "a_missing_resource_is_a_bad_request",
        "an_unknown_prompt_is_a_bad_request",
        "an_invalid_prompt_argument_is_a_bad_request",
    ] {
        assert!(
            status.contains(&format!("name: {name},")),
            "Status declares no theorem {name}"
        );
    }
}

#[test]
fn the_vendored_requirements_list_the_resource_scenarios_as_required_rp_17() {
    let yaml = suite_text("requirements/2026-07-28.yaml");
    let server: Vec<&str> = yaml
        .lines()
        .skip_while(|l| *l != "server:")
        .skip(1)
        .take_while(|l| l.starts_with("  - "))
        .collect();
    for scenario in [
        "completion-complete",
        "resources-list",
        "resources-read-text",
        "resources-read-binary",
        "resources-templates-read",
        "sep-2164-resource-not-found",
        "prompts-list",
        "prompts-get-simple",
        "prompts-get-with-args",
        "prompts-get-embedded-resource",
        "prompts-get-with-image",
        "caching",
    ] {
        assert!(
            server.contains(&format!("  - {scenario}").as_str()),
            "{scenario} is a required server scenario"
        );
    }
}

#[test]
fn every_check_id_the_resource_scenarios_emit_is_covered_or_listed_rp_17() {
    let file = common::resources_vector_file();
    let mut known: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut scenarios: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for v in file["vectors"].as_array().expect("vectors") {
        scenarios.insert(v["scenario"].as_str().expect("scenario").to_owned());
        for c in v["suite_checks"].as_array().expect("suite_checks") {
            known.insert(c.as_str().expect("id").to_owned());
        }
    }
    for n in file["not_run"].as_array().expect("not_run") {
        for c in n["checks"].as_array().expect("checks") {
            known.insert(c.as_str().expect("id").to_owned());
        }
    }
    for u in file["unreached"].as_array().expect("unreached") {
        known.insert(u["id"].as_str().expect("id").to_owned());
        assert!(!u["reason"].as_str().expect("reason").is_empty());
    }
    let mut emitted: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for scenario_file in [
        "src/scenarios/server/resources.ts",
        "src/scenarios/server/prompts.ts",
        "src/scenarios/server/caching.ts",
        "src/scenarios/server/utils.ts",
    ] {
        let text = suite_text(scenario_file);
        for token in text.split('\'') {
            let lowercase_id = !token.is_empty()
                && token
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                && token.contains('-')
                && !token.starts_with('-');
            // the ids of checks: the scenario names, the sep- ids and the caching ids
            if lowercase_id
                && (token.starts_with("resources-")
                    || token.starts_with("prompts-")
                    || token.starts_with("sep-")
                    || token == "completion-complete"
                    || token == "logging-set-level")
            {
                emitted.insert(token.to_owned());
            }
        }
        for line in text.lines() {
            if line.trim() == "name = 'ping';" {
                emitted.insert("ping".to_owned());
            }
        }
    }
    let missing: Vec<&String> = emitted
        .iter()
        .filter(|id| !known.contains(*id) && !scenarios.contains(*id))
        .collect();
    assert!(
        missing.is_empty(),
        "check ids no vector covers: {missing:?}"
    );
    for id in &known {
        assert!(
            emitted.contains(id) || id == "ping" || scenarios.contains(id),
            "{id} is not an id the scenarios emit"
        );
    }
}

// ---- RP-18: the capabilities a provider declares ----

fn discover() -> Request {
    request("server/discover", &json!({}), None, "7")
}

#[test]
fn the_fixture_declares_the_capabilities_of_the_methods_it_serves_rp_18() {
    let result = result_of(&discover());
    let capabilities = &result["capabilities"];
    assert_eq!(capabilities["resources"], json!({}));
    assert_eq!(capabilities["prompts"], json!({}));
    assert_eq!(capabilities["completions"], json!({}));
    assert_eq!(capabilities["tools"], json!({}));
    for (method, name) in [
        ("resources/list", None),
        ("resources/templates/list", None),
        ("prompts/list", None),
        ("resources/read", Some("test://static-text")),
        ("prompts/get", Some("test_simple_prompt")),
        ("completion/complete", None),
    ] {
        let params = match method {
            "resources/read" => json!({"uri": "test://static-text"}),
            "prompts/get" => json!({"name": "test_simple_prompt"}),
            "completion/complete" => json!({
                "ref": prompt_ref("test_prompt_with_arguments"),
                "argument": {"name": "arg1", "value": ""}
            }),
            _ => json!({}),
        };
        let (response, body) = send(&request(method, &params, name, "8"));
        assert_eq!(response.status, 200, "{method}: {body}");
    }
}

#[test]
fn a_provider_that_declares_none_answers_the_methods_as_not_found_rp_18() {
    // the second root offers tools only
    let discovered = crowd_checked(&discover());
    let body: Value = serde_json::from_slice(&discovered.body).expect("JSON");
    let capabilities = body["result"]["capabilities"]
        .as_object()
        .expect("capabilities");
    assert!(capabilities.contains_key("tools"));
    for key in ["resources", "prompts", "completions"] {
        assert!(!capabilities.contains_key(key), "{key}");
    }
    for (method, params, name) in [
        ("resources/list", json!({}), None),
        ("resources/templates/list", json!({}), None),
        ("prompts/list", json!({}), None),
        (
            "resources/read",
            json!({"uri": "test://static-text"}),
            Some("test://static-text"),
        ),
        (
            "prompts/get",
            json!({"name": "test_simple_prompt"}),
            Some("test_simple_prompt"),
        ),
        (
            "completion/complete",
            json!({"ref": prompt_ref("test_prompt_with_arguments"), "argument": {"name": "arg1", "value": ""}}),
            None,
        ),
    ] {
        let response: Response = crowd_checked(&request(method, &params, name, "9"));
        assert_eq!(response.status, 404, "{method}");
        let body: Value = serde_json::from_slice(&response.body).expect("JSON");
        assert_eq!(body["error"]["code"], -32601, "{method}");
    }
}

// ---- RP-19: sanitization of prompts ----

#[test]
fn control_and_bidirectional_characters_of_a_prompt_are_replaced_rp_19() {
    let unsafe_text = "bell\u{7} escape\u{1b}[31m rtl\u{202e} c1\u{85} keep\u{a0}and\ttab";
    let result = result_of(&get(
        "test_prompt_with_arguments",
        &json!({"arg1": unsafe_text, "arg2": "ok"}),
    ));
    let text = result["messages"][0]["content"]["text"]
        .as_str()
        .expect("text");
    assert_eq!(
        text,
        "Prompt with arguments: arg1='bell\u{fffd} escape\u{fffd}[31m rtl\u{fffd} c1\u{fffd} keep\u{a0}and\ttab', arg2='ok'"
    );
    // the same through an embedded resource uri
    let embedded = result_of(&get(
        "test_prompt_with_embedded_resource",
        &json!({"resourceUri": "a\u{7}b"}),
    ));
    assert_eq!(
        embedded["messages"][0]["content"]["resource"]["uri"],
        "a\u{fffd}b"
    );
}

#[test]
fn the_contents_of_a_resource_are_written_as_the_catalog_stores_them_rp_19() {
    let uri = "test://template/a\u{7}b/data";
    let name = format!("=?base64?{}?=", base64(uri.as_bytes()));
    let result = result_of(&request(
        "resources/read",
        &json!({"uri": uri}),
        Some(&name),
        "5",
    ));
    assert_eq!(result["contents"][0]["uri"], uri);
    let text: Value = serde_json::from_str(result["contents"][0]["text"].as_str().expect("text"))
        .expect("JSON text");
    assert_eq!(text["id"], "a\u{7}b");
}
