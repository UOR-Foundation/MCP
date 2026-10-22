//! JC-13 through the lifecycle root: a request whose body holds one long string is
//! answered in time that grows linearly with the string, up to the logarithmic
//! factor of a balanced merge.
//!
//! The string is the value of one argument, then the name of another, in a `tools/call`
//! of the fixture's `echo`, which answers with a fixed acknowledgement: the time is the
//! frame's, the codec's read of the body and the core's checks. The bound and the
//! growth checks are those of `crates/json-oracle/tests/stress.rs`, and the control is
//! the same: the package before UOR-Foundation/MCP#23, which took about 300 s for a
//! 2 MiB string (`LIFECYCLE.md`).

mod common;

use std::sync::mpsc;
use std::time::{Duration, Instant};

use common::{call, frame_checked, post_owned, standard};
use lifecycle_oracle::encode_request;

/// The lengths of the string, in bytes.
const LENGTHS: [usize; 3] = [256 * 1024, 1024 * 1024, 2 * 1024 * 1024];

/// The fixed part of the bound, for thread start and allocator warm-up.
const FIXED: Duration = Duration::from_millis(500);

/// The per-byte part of the bound.
const PER_BYTE: Duration = Duration::from_nanos(20_000);

/// The largest factor by which one request may exceed the one before it, as a
/// multiple of the factor by which the string grew (4, then 2).
const GROWTH_SLACK: f64 = 2.0;

/// The shortest time a growth ratio is taken against.
const FLOOR: Duration = Duration::from_millis(20);

fn bound(length: usize) -> Duration {
    FIXED + PER_BYTE * u32::try_from(length).expect("a length fits in 32 bits")
}

/// A `tools/call` of `echo` whose arguments object holds one string of `length`
/// bytes, as a value or as a member name.
fn frame_with_string(length: usize, as_name: bool) -> Vec<u8> {
    let text = "a".repeat(length);
    let arguments = if as_name {
        format!(r#"{{"{text}":1}}"#)
    } else {
        format!(r#"{{"query":"{text}"}}"#)
    };
    let request = post_owned(
        &standard("tools/call", Some("echo")),
        &call("echo", &arguments, "1"),
    );
    encode_request(&request)
}

/// Run one frame on a worker thread, waiting no longer than `bound`.
fn answered_within(frame: Vec<u8>, bound: Duration) -> Result<(Duration, u32), Duration> {
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let started = Instant::now();
        let response = frame_checked(&frame).expect("a response frame");
        let _ = sender.send((started.elapsed(), response.status));
    });
    receiver.recv_timeout(bound).map_err(|_| bound)
}

fn check(name: &str, as_name: bool) {
    // The schema the oracle checks every response against is compiled on the first
    // use; do that outside the measurement.
    let (_, warm) = answered_within(frame_with_string(8, as_name), Duration::from_secs(120))
        .expect("a small request is answered");
    assert_eq!(warm, 200);
    let mut times: Vec<Duration> = Vec::new();
    for length in LENGTHS {
        let frame = frame_with_string(length, as_name);
        let limit = bound(frame.len());
        let (elapsed, status) = answered_within(frame, limit).unwrap_or_else(|limit| {
            panic!("{name}: a {length}-byte string was not answered within the bound of {limit:?}")
        });
        assert_eq!(status, 200, "{name}: {length} bytes");
        eprintln!("{name}: {length} bytes answered in {elapsed:?} (bound {limit:?})");
        times.push(elapsed);
    }
    for (pair, grew) in times.windows(2).zip([4.0_f64, 2.0]) {
        let ratio = pair[1].max(FLOOR).as_secs_f64() / pair[0].max(FLOOR).as_secs_f64();
        assert!(
            ratio <= grew * GROWTH_SLACK,
            "{name}: the string grew {grew}-fold but the time {ratio:.1}-fold ({:?} to {:?})",
            pair[0],
            pair[1]
        );
    }
}

#[test]
fn a_long_string_argument_is_read_in_linear_time_jc_13() {
    check("string argument", false);
}

#[test]
fn a_long_argument_name_is_read_in_linear_time_jc_13() {
    check("argument name", true);
}
