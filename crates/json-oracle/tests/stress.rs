//! JC-13: reading a JSON string takes time that grows linearly with its length, up
//! to the logarithmic factor of a balanced merge.
//!
//! Each shape is read at three lengths through the exported root, `codecRun`, in a
//! worker thread. The time of a read is checked two ways:
//!
//! * against a bound that grows linearly, `FIXED + PER_BYTE * length`, so a read that
//!   falls behind that line fails at the first length it exceeds it; and
//! * against the growth from one length to the next: lengths grow by 4 and then 2, a
//!   linear read grows by about the same factor, and a quadratic read by its square.
//!
//! A read that is still running when its bound passes is reported as a failure
//! without waiting for it: the worker thread is left to the end of the process. The
//! control is the package before UOR-Foundation/MCP#23, whose reads of 1 MiB and
//! 2 MiB took about 75 s and 300 s.

use std::sync::mpsc;
use std::time::{Duration, Instant};

use json_oracle::{parse, utf8, Parse, Verdict};

/// The lengths of the string content, in bytes of JSON text.
const LENGTHS: [usize; 3] = [256 * 1024, 1024 * 1024, 2 * 1024 * 1024];

/// The fixed part of the bound, for thread start and allocator warm-up.
const FIXED: Duration = Duration::from_millis(500);

/// The per-byte part of the bound. The measured reads take about a tenth of it.
const PER_BYTE: Duration = Duration::from_nanos(20_000);

/// The largest factor by which one read may exceed the one before it, as a multiple
/// of the factor by which the length grew. A linear read has 1, a read with a
/// logarithmic factor a little more, and a quadratic read the length factor itself,
/// 4 and then 2.
const GROWTH_SLACK: f64 = 2.0;

/// The shortest time a growth ratio is taken against: below it the timer's noise is
/// larger than the quantity measured.
const FLOOR: Duration = Duration::from_millis(20);

/// The bound on a read of `length` bytes.
fn bound(length: usize) -> Duration {
    FIXED + PER_BYTE * u32::try_from(length).expect("a length fits in 32 bits")
}

/// Run `read` on a worker thread and wait for it no longer than `bound`.
fn within<T: Send + 'static>(
    bound: Duration,
    read: impl FnOnce() -> T + Send + 'static,
) -> Result<(Duration, T), Duration> {
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let started = Instant::now();
        let value = read();
        let _ = sender.send((started.elapsed(), value));
    });
    receiver.recv_timeout(bound).map_err(|_| bound)
}

/// A text of one string with `length` bytes of content, and the canonical text the
/// codec must write for it.
struct Shape {
    name: &'static str,
    build: fn(usize) -> (Vec<u8>, Vec<u8>),
}

fn plain(length: usize) -> (Vec<u8>, Vec<u8>) {
    let text = [b"\"", "a".repeat(length).as_bytes(), b"\""].concat();
    (text.clone(), text)
}

/// Escapes, a surrogate pair and multi-byte characters in every unit of 27 bytes.
fn mixed(length: usize) -> (Vec<u8>, Vec<u8>) {
    let unit = "é\\u00e9\\ud83d\\ude00😀\\nx";
    let canonical = "éé😀😀\\nx";
    let units = length / unit.len();
    (
        format!("\"{}\"", unit.repeat(units)).into_bytes(),
        format!("\"{}\"", canonical.repeat(units)).into_bytes(),
    )
}

/// The same bytes as a member name, which the reader collects as it does a value.
fn key(length: usize) -> (Vec<u8>, Vec<u8>) {
    let text = format!("{{\"{}\":0}}", "k".repeat(length)).into_bytes();
    (text.clone(), text)
}

fn check(shape: &Shape) {
    let mut times: Vec<Duration> = Vec::new();
    for length in LENGTHS {
        let (input, expected) = (shape.build)(length);
        let limit = bound(input.len());
        let (elapsed, parsed) = within(limit, move || parse(&input)).unwrap_or_else(|limit| {
            panic!(
                "{}: reading {length} bytes was not finished within the bound of {limit:?}",
                shape.name
            )
        });
        match parsed {
            Parse::Value(bytes) => assert!(
                bytes == expected,
                "{}: the canonical text of {length} bytes differs from the expected one",
                shape.name
            ),
            other => panic!("{}: {length} bytes were not accepted: {other:?}", shape.name),
        }
        eprintln!(
            "{}: {length} bytes read in {elapsed:?} (bound {limit:?})",
            shape.name
        );
        times.push(elapsed);
    }
    for (pair, grew) in times.windows(2).zip([4.0_f64, 2.0]) {
        let ratio = pair[1].max(FLOOR).as_secs_f64() / pair[0].max(FLOOR).as_secs_f64();
        assert!(
            ratio <= grew * GROWTH_SLACK,
            "{}: the length grew {grew}-fold but the time {ratio:.1}-fold ({:?} to {:?})",
            shape.name,
            pair[0],
            pair[1]
        );
    }
}

#[test]
fn a_long_string_is_read_in_linear_time_jc_13() {
    check(&Shape {
        name: "plain string",
        build: plain,
    });
}

#[test]
fn a_long_string_of_escapes_and_multibyte_characters_is_read_in_linear_time_jc_13() {
    check(&Shape {
        name: "escapes and multi-byte characters",
        build: mixed,
    });
}

#[test]
fn a_long_member_name_is_read_in_linear_time_jc_13() {
    check(&Shape {
        name: "member name",
        build: key,
    });
}

#[test]
fn the_validator_alone_is_linear_on_the_same_lengths_jc_13() {
    // The control for the harness: the part of a read that never built a byte string
    // takes the time a linear pass takes, so the bound above is not loose by
    // construction.
    for length in LENGTHS {
        let input = plain(length).0;
        let limit = bound(input.len());
        let (elapsed, verdict) = within(limit, move || utf8(&input))
            .unwrap_or_else(|limit| panic!("the validator took more than {limit:?}"));
        assert_eq!(verdict, Verdict::Valid);
        eprintln!("validator alone: {length} bytes in {elapsed:?}");
    }
}
