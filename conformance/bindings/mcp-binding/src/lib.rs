//! The host of the MCP conformance build.
//!
//! The core is a pure function of its request frame, so what it cannot compute it is
//! given. The adapter asks this host, once per request, for the sections of the frame:
//!
//! * `tool.calls`, with the flag that says whether the request is a `tools/call`: how many
//!   such requests this client has already made in the current rate-limit window, which
//!   the core compares with its limit (`TOOLS.md`, "The rate limit");
//! * `clock.now`: a monotonic millisecond counter and the same instant as ISO 8601 text,
//!   which a task's `createdAt`, `lastUpdatedAt` and time to live are read from;
//! * `task.id`: an identifier no task has had and no client can guess, which the core
//!   uses only when the request creates a task;
//! * `task.table`: the table of tasks the host holds.
//!
//! The core asks for one effect, `task.store`: the table of tasks a request changed, which
//! the host holds until the core is given it with the next request (`TASKS.md`, "The host's
//! side"). The table is the core's to write and the host's to keep and to hand back
//! unchanged.
//!
//! The adapter gives a host no client identity beyond the flag, and the conformance run is
//! one client on one loopback interface, so there is one window and one table for the whole
//! process. The window is fixed: it opens at the first counted request and closes sixty
//! seconds later. The adapter takes the host's lock for one effect at a time, so two
//! requests that overlap read the table before either has stored it and the later store
//! wins; the suite's scenarios send their requests one after another (`TASKS.md`,
//! "Limits").

use std::io::Read;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use mcp_conformance_host_api::protocol::{
    clock_now, task_id, task_store, task_table, tool_calls, Host, HostError,
};

/// How long a window of counted requests lasts.
pub const WINDOW: Duration = Duration::from_secs(60);

/// Days since 1970-01-01 to a civil date (Howard Hinnant's `civil_from_days`).
fn civil(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

/// An instant, in milliseconds since the Unix epoch, as ISO 8601 text in UTC.
pub fn iso(epoch_ms: u64) -> String {
    let secs = epoch_ms / 1000;
    let milli = epoch_ms % 1000;
    let (year, month, day) = civil(i64::try_from(secs / 86_400).unwrap_or(i64::MAX));
    let rem = secs % 86_400;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}.{milli:03}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}

/// Sixteen random bytes as text: from the operating system's source when it can be read,
/// and otherwise from the standard library's randomly keyed hasher.
fn random_hex() -> String {
    let mut bytes = [0u8; 16];
    let read = std::fs::File::open("/dev/urandom")
        .and_then(|mut source| source.read_exact(&mut bytes))
        .is_ok();
    if !read {
        use std::hash::{BuildHasher, Hasher};
        for chunk in bytes.chunks_mut(8) {
            let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
            hasher.write_u128(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_or(0, |elapsed| elapsed.as_nanos()),
            );
            chunk.copy_from_slice(&hasher.finish().to_be_bytes()[..chunk.len()]);
        }
    }
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Counts the `tools/call` requests of the current window, tells the time, issues task
/// identifiers and holds the table of tasks.
pub struct Native {
    window: Option<(Instant, u32)>,
    started: Instant,
    started_epoch_ms: u64,
    table: Vec<u8>,
}

impl Default for Native {
    fn default() -> Self {
        Native {
            window: None,
            started: Instant::now(),
            started_epoch_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_or(0, |elapsed| {
                    u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
                }),
            table: Vec::new(),
        }
    }
}

impl Native {
    /// The number of counted requests made in the window that is open at `now`, before
    /// this request, and then this request counted when `counted` is true.
    pub fn answer(&mut self, counted: bool, now: Instant) -> u32 {
        let (opened, calls) = match self.window {
            Some((opened, calls)) if now.saturating_duration_since(opened) < WINDOW => {
                (opened, calls)
            }
            _ => (now, 0),
        };
        self.window = Some((
            opened,
            if counted {
                calls.saturating_add(1)
            } else {
                calls
            },
        ));
        calls
    }

    /// The milliseconds since this host started, and the same instant as ISO 8601 text.
    pub fn clock(&self, now: Instant) -> (u64, String) {
        let ms = u64::try_from(now.saturating_duration_since(self.started).as_millis())
            .unwrap_or(u64::MAX);
        (ms, iso(self.started_epoch_ms.saturating_add(ms)))
    }
}

impl Host for Native {
    fn tool_calls(
        &mut self,
        request: tool_calls::Request,
    ) -> Result<tool_calls::Completed, HostError<tool_calls::Failure>> {
        Ok(tool_calls::Completed {
            calls: self.answer(request.counted, Instant::now()),
        })
    }

    fn clock_now(
        &mut self,
        _request: clock_now::Request,
    ) -> Result<clock_now::Completed, HostError<clock_now::Failure>> {
        let (ms, iso) = self.clock(Instant::now());
        Ok(clock_now::Completed { ms, iso })
    }

    fn task_id(
        &mut self,
        _request: task_id::Request,
    ) -> Result<task_id::Completed, HostError<task_id::Failure>> {
        Ok(task_id::Completed { id: random_hex() })
    }

    fn task_table(
        &mut self,
        _request: task_table::Request,
    ) -> Result<task_table::Completed, HostError<task_table::Failure>> {
        Ok(task_table::Completed {
            table: self.table.clone(),
        })
    }

    fn task_store(
        &mut self,
        request: task_store::Request,
    ) -> Result<task_store::Completed, HostError<task_store::Failure>> {
        self.table = request.table;
        Ok(task_store::Completed {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_request_is_answered_with_the_count_before_it() {
        let mut host = Native::default();
        let start = Instant::now();
        assert_eq!(host.answer(true, start), 0);
        assert_eq!(host.answer(true, start), 1);
        assert_eq!(host.answer(true, start), 2);
    }

    #[test]
    fn a_request_that_is_not_counted_does_not_move_the_count() {
        let mut host = Native::default();
        let start = Instant::now();
        assert_eq!(host.answer(true, start), 0);
        assert_eq!(host.answer(false, start), 1);
        assert_eq!(host.answer(false, start), 1);
        assert_eq!(host.answer(true, start), 1);
        assert_eq!(host.answer(false, start), 2);
    }

    #[test]
    fn the_window_closes_after_sixty_seconds_and_the_count_starts_again() {
        let mut host = Native::default();
        let start = Instant::now();
        assert_eq!(host.answer(true, start), 0);
        assert_eq!(
            host.answer(true, start + WINDOW - Duration::from_millis(1)),
            1
        );
        assert_eq!(host.answer(true, start + WINDOW), 0);
        assert_eq!(
            host.answer(true, start + WINDOW + Duration::from_secs(1)),
            1
        );
    }

    #[test]
    fn the_effect_reports_the_count_before_the_request() {
        let mut host = Native::default();
        let first = host.tool_calls(tool_calls::Request { counted: true });
        let second = host.tool_calls(tool_calls::Request { counted: true });
        assert_eq!(first.ok().map(|done| done.calls), Some(0));
        assert_eq!(second.ok().map(|done| done.calls), Some(1));
    }

    #[test]
    fn the_clock_reports_one_instant_in_two_forms() {
        let host = Native::default();
        let (first, text) = host.clock(host.started);
        assert_eq!(first, 0);
        assert_eq!(text, iso(host.started_epoch_ms));
        let (later, later_text) = host.clock(host.started + Duration::from_millis(1_500));
        assert_eq!(later, 1_500);
        assert_eq!(later_text, iso(host.started_epoch_ms + 1_500));
    }

    #[test]
    fn an_instant_is_iso_8601_text_in_utc_with_milliseconds() {
        assert_eq!(iso(0), "1970-01-01T00:00:00.000Z");
        assert_eq!(iso(1_790_000_000_123), "2026-09-21T14:13:20.123Z");
        assert_eq!(iso(951_782_400_000), "2000-02-29T00:00:00.000Z");
    }

    #[test]
    fn a_task_identifier_is_thirty_two_hex_digits_and_is_not_repeated() {
        let mut host = Native::default();
        let first = host.task_id(task_id::Request {}).ok().map(|done| done.id);
        let second = host.task_id(task_id::Request {}).ok().map(|done| done.id);
        let first = first.expect("an identifier");
        let second = second.expect("an identifier");
        assert_eq!(first.len(), 32);
        assert!(first.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_ne!(first, second);
    }

    #[test]
    fn the_table_is_what_the_core_last_stored() {
        let mut host = Native::default();
        let empty = host.task_table(task_table::Request {}).ok();
        assert_eq!(empty.map(|done| done.table), Some(Vec::new()));
        assert!(host
            .task_store(task_store::Request {
                table: b"[{}]".to_vec()
            })
            .is_ok());
        let held = host.task_table(task_table::Request {}).ok();
        assert_eq!(held.map(|done| done.table), Some(b"[{}]".to_vec()));
    }
}
