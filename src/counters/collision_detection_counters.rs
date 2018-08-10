use counters::Timer;
use std::fmt::{Display, Formatter, Result};

/// Performance counters related to collision detection.
pub struct CollisionDetectionCounters {
    /// Number of contact pairs detected.
    pub ncontact_pairs: usize,
    /// Time spent for the broad-phase of the collision detection.
    pub broad_phase_time: Timer,
    /// Time spent for the narrow-phase of the collision detection.
    pub narrow_phase_time: Timer,
}

impl CollisionDetectionCounters {
    /// Creates a new counter initialized to zero.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        CollisionDetectionCounters {
            ncontact_pairs:    0,
            broad_phase_time:  Timer::new(),
            narrow_phase_time: Timer::new(),
        }
    }

    /// Creates a new counter initialized to zero.
    #[cfg(target_arch = "wasm32")]
    pub fn new(time_in_sec: fn() -> f64) -> Self {
        CollisionDetectionCounters {
            ncontact_pairs:    0,
            broad_phase_time:  Timer::new(time_in_sec),
            narrow_phase_time: Timer::new(time_in_sec),
        }
    }
}

impl Display for CollisionDetectionCounters {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Number of contact pairs: {}", self.ncontact_pairs)?;
        writeln!(f, "Broad-phase time: {}", self.broad_phase_time)?;
        writeln!(f, "Narrow-phase time: {}", self.narrow_phase_time)
    }
}
