//! The caching hints: which results carry `ttlMs` and `cacheScope`, which do not, and
//! that the core writes the provider's own hint for each result.

mod common;

use common::{
    assert_declared, call, complete_request, list_request, post_owned, prompt_request,
    read_request, rpc, run_register, send, standard, META,
};
use serde_json::{json, Value};

fn assert_hint(body: &Value, ttl: u64, scope: &str) {
    assert_eq!(body["result"]["ttlMs"], ttl, "{body}");
    assert_eq!(body["result"]["cacheScope"], scope, "{body}");
}

fn discover() -> Value {
    let body = rpc("server/discover", &format!(r#"{{"_meta":{META}}}"#), "1");
    send(&post_owned(&standard("server/discover", None), &body)).1
}

// ---- CH-01: every cacheable result ----

#[test]
fn the_caching_scenarios_requests_are_answered_with_hints_ch_01() {
    assert_eq!(run_register("CH-01"), 5);
}

#[test]
fn each_of_the_six_cacheable_results_carries_both_members_ch_01() {
    let results = [
        discover(),
        send(&list_request("tools/list", None)).1,
        send(&list_request("resources/list", None)).1,
        send(&list_request("resources/templates/list", None)).1,
        send(&list_request("prompts/list", None)).1,
        send(&read_request("test://static-text")).1,
    ];
    for body in results {
        assert!(body["result"]["ttlMs"].is_u64(), "{body}");
        let scope = body["result"]["cacheScope"].as_str().expect("a scope");
        assert!(["public", "private"].contains(&scope), "{body}");
    }
}

#[test]
fn the_hints_the_fixture_declares_are_written_as_given_ch_01() {
    assert_hint(
        &send(&list_request("resources/list", None)).1,
        300_000,
        "public",
    );
    assert_hint(
        &send(&list_request("resources/templates/list", None)).1,
        300_000,
        "public",
    );
    assert_hint(
        &send(&list_request("prompts/list", None)).1,
        300_000,
        "public",
    );
    assert_hint(&send(&list_request("tools/list", None)).1, 0, "private");
    assert_hint(&discover(), 0, "private");
}

// ---- CH-02: the provider's own hint for each result ----

#[test]
fn two_reads_may_carry_different_hints_ch_02() {
    assert_eq!(run_register("CH-02"), 1);
    assert_hint(
        &send(&read_request("test://static-text")).1,
        300_000,
        "private",
    );
    assert_hint(
        &send(&read_request("test://static-binary")).1,
        3_600_000,
        "public",
    );
    assert_hint(
        &send(&read_request("test://template/1/data")).1,
        60_000,
        "private",
    );
}

// ---- CH-03: results that are not cacheable ----

#[test]
fn results_that_are_not_cacheable_carry_no_hint_ch_03() {
    let results = [
        send(&post_owned(
            &standard("tools/call", Some("test_simple_text")),
            &call("test_simple_text", "{}", "7"),
        ))
        .1,
        send(&prompt_request("test_simple_prompt", None)).1,
        send(&complete_request(&json!({
            "ref": {"type": "ref/prompt", "name": "test_prompt_with_arguments"},
            "argument": {"name": "arg1", "value": ""}
        })))
        .1,
    ];
    for body in results {
        assert!(body["result"].is_object(), "{body}");
        assert!(body["result"].get("ttlMs").is_none(), "{body}");
        assert!(body["result"].get("cacheScope").is_none(), "{body}");
    }
}

#[test]
fn an_interim_or_task_result_is_never_cacheable_ch_03() {
    assert_declared(&["Methods"], &["an_interim_result_is_never_cacheable"]);
}

// ---- CH-04: the theorems ----

#[test]
fn the_caching_layer_states_its_theorems_ch_04() {
    assert_declared(
        &["Methods"],
        &["a_result_is_cacheable_if_and_only_if_it_has_a_cache_hint"],
    );
    assert_declared(
        &["ResultJson"],
        &[
            "a_cache_hint_is_written_as_ttl_and_scope",
            "every_cacheable_result_is_written_with_its_cache_members",
        ],
    );
}
