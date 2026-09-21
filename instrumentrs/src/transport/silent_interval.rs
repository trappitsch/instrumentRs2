//! Manage silent intervals that might occur in an instrument.

use std::time::{Duration, Instant};

/// Manage silent interval in communications.
///
/// MODBUS requires us that we have a silent interval at the beginning and at the end of each
/// transission of at least 3.5 characters. This helper facilitates complying with this rule.
/// As the chiller can run at two baud rates but we don't know which one they chose, we will use the
/// slower one here to enforce the silent interval. The silent interval is still only 8.021 ms for 7
/// characters (3.5 at start and 3.5 at end).
///
/// The silent interval is define in the static `SILENT_INT` in this module.
pub struct SilentInterval {
    duration: Duration,
    last_write: Instant,
}

impl SilentInterval {
    /// Create a new SilentInterval structure with a given, fixed duration.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            last_write: Instant::now(),
        }
    }

    /// Block thread (sleep) until silent interval has expired.
    ///
    /// This checks how much time expired since the last read. If this is smaller than the required
    /// silent interval, it will block the thread for the time required, afterwards update the last
    /// write time and continue.
    pub fn block(&mut self) {
        let dt = Instant::now() - self.last_write;

        if dt < self.duration {
            std::thread::sleep(self.duration - dt);
        }

        self.last_write = Instant::now();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn silent_interval_duration_blocks_thread() {
        let mut si = SilentInterval::new(Duration::from_micros(100));

        let tic = Instant::now();
        si.block();
        let toc = Instant::now();

        assert!(si.last_write > tic);
        assert!(toc - tic >= Duration::from_micros(100));
    }
}
