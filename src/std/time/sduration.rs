use std::time::Duration;

/// A Duration type to represent a span of time, typically used for system timeouts.
///
/// See documentation of [`std::time::Duration`]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SDuration {
    secs: u64,
    nanos: u32,
}

impl SDuration {
    pub fn from_duration(d: Duration) -> Self {
        Self {
            secs: d.as_secs(),
            nanos: d.subsec_nanos(),
        }
    }
    pub fn as_duration(self) -> Duration {
        Duration::new(self.secs, self.nanos)
    }
    // todo more methods
}

impl From<Duration> for SDuration {
    fn from(d: Duration) -> Self {
        Self::from_duration(d)
    }
}

impl From<SDuration> for Duration {
    fn from(s: SDuration) -> Self {
        s.as_duration()
    }
}

// todo more trait impls
