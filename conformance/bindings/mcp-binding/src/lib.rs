//! The host of the MCP conformance build.
//!
//! The core is a pure function of its request frame, so the one thing it cannot
//! compute is state: how many `tools/call` requests this client has already made in
//! the current rate-limit window. The adapter asks this host once per request, with the
//! flag that says whether the request is a `tools/call`, and puts the answer in the
//! frame. The core compares it with its limit (`TOOLS.md`, "The rate limit").
//!
//! The adapter gives a host no client identity beyond the flag, and the conformance
//! run is one client on one loopback interface, so there is one window for the whole
//! process. The window is fixed: it opens at the first counted request and closes
//! sixty seconds later.

use std::time::{Duration, Instant};

use mcp_conformance_host_api::protocol::{tool_calls, Host, HostError};

/// How long a window of counted requests lasts.
pub const WINDOW: Duration = Duration::from_secs(60);

/// Counts the `tools/call` requests of the current window.
#[derive(Default)]
pub struct Native {
    window: Option<(Instant, u32)>,
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
}
