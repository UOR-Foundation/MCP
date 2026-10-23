//! The requests the pinned conformance scenarios send, run in process against the
//! generated lifecycle root.
//!
//! Each vector in `vectors/suite.json` is one request a scenario sends, built as the
//! scenario builds it, with the checks the scenario makes on the answer. They are
//! falsifying evidence: a vector the root fails is a defect, and a vector it passes
//! shows only that this request meets this check, never that the suite's own run would
//! pass. The oracle's verdict is the suite run against the conformance build (issue 13).

mod common;

use common::run_register;

#[test]
fn missing_meta_or_a_missing_required_member_is_invalid_params_with_400_lc_01() {
    assert_eq!(run_register("LC-01"), 3);
}

#[test]
fn server_discover_and_tools_list_answer_with_the_declared_surface_lc_02() {
    assert_eq!(run_register("LC-02"), 4);
}

#[test]
fn a_request_without_client_info_is_served_lc_03() {
    assert_eq!(run_register("LC-03"), 1);
}

#[test]
fn an_unsupported_version_is_refused_with_the_supported_list_and_the_echo_lc_04() {
    assert_eq!(run_register("LC-04"), 1);
}

#[test]
fn every_result_carries_result_type_and_the_server_identity_lc_05() {
    assert_eq!(run_register("LC-05"), 12);
}

#[test]
fn removed_unknown_and_undeclared_methods_are_not_found_with_404_lc_06() {
    assert_eq!(run_register("LC-06"), 6);
}

#[test]
fn an_undeclared_client_capability_is_refused_with_400_and_the_requirement_lc_07() {
    assert_eq!(run_register("LC-07"), 1);
}

#[test]
fn every_refusal_echoes_the_request_id_lc_08() {
    assert_eq!(run_register("LC-08"), 20);
}

#[test]
fn a_protocol_version_header_that_disagrees_with_the_body_is_a_header_mismatch_hh_01() {
    assert_eq!(run_register("HH-01"), 1);
}

#[test]
fn a_mismatched_mcp_method_is_a_header_mismatch_hh_02() {
    assert_eq!(run_register("HH-02"), 1);
}

#[test]
fn a_missing_mcp_method_is_a_header_mismatch_hh_03() {
    assert_eq!(run_register("HH-03"), 1);
}

#[test]
fn a_mismatched_mcp_name_is_a_header_mismatch_hh_04() {
    assert_eq!(run_register("HH-04"), 1);
}

#[test]
fn optional_whitespace_around_a_header_value_is_trimmed_hh_05() {
    assert_eq!(run_register("HH-05"), 1);
}

#[test]
fn a_tools_call_without_mcp_name_is_a_header_mismatch_hh_06() {
    assert_eq!(run_register("HH-06"), 1);
}

#[test]
fn header_names_are_case_insensitive_hh_07() {
    assert_eq!(run_register("HH-07"), 2);
}

#[test]
fn header_values_are_case_sensitive_hh_08() {
    assert_eq!(run_register("HH-08"), 1);
}

#[test]
fn a_valid_base64_parameter_header_is_decoded_and_compared_hh_09() {
    assert_eq!(run_register("HH-09"), 1);
}

#[test]
fn a_malformed_base64_parameter_header_is_rejected_hh_10() {
    assert_eq!(run_register("HH-10"), 2);
}

#[test]
fn a_value_that_only_resembles_the_sentinel_is_literal_hh_11() {
    assert_eq!(run_register("HH-11"), 2);
}

#[test]
fn an_omitted_parameter_header_whose_body_value_is_present_is_rejected_hh_12() {
    assert_eq!(run_register("HH-12"), 1);
}
