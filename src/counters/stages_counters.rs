use counters::Timer;
use std::fmt::{Display, Formatter, Result};

/// Performance counters related to each stage of the time step.
pub struct StagesCounters {
    /// Time spent for updating the kinematic and dynamics of every body.
    pub update_time: Timer,
    /// Total time spent for the collision detection (including both broad-
    /// and narrow- phases).
    pub collision_detection_time: Timer,
    /// Time spent for the computation of collision island and body
    /// activation/deactivation (sleeping).
    pub island_construction_time: Timer,
    /// Total time spent for the constraints resolution and position update.t
    pub solver_time: Timer,
}

impl StagesCounters {
    /// Create a new counter intialized to zero.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        StagesCounters {
            update_time:              Timer::new(),
            collision_detection_time: Timer::new(),
            island_construction_time: Timer::new(),
            solver_time:              Timer::new(),
        }
    }

    /// Create a new counter intialized to zero.
    #[cfg(target_arch = "wasm32")]
    pub fn new(time_in_sec: fn() -> f64) -> Self {
        StagesCounters {
            update_time:              Timer::new(time_in_sec),
            collision_detection_time: Timer::new(time_in_sec),
            island_construction_time: Timer::new(time_in_sec),
            solver_time:              Timer::new(time_in_sec),
        }
    }
}

impl Display for StagesCounters {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "Update time: {}", self.update_time)?;
        writeln!(
            f,
            "Collision detection time: {}",
            self.collision_detection_time
        )?;
        writeln!(
            f,
            "Island construction time: {}",
            self.island_construction_time
        )?;
        writeln!(f, "Solver time: {}", self.solver_time)
    }
}
