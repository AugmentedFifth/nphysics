use std::fmt::{Display, Error, Formatter};

/// A timer.
#[derive(Copy, Clone, Debug)]
pub struct Timer {
    time: f64,
    start: Option<f64>,
    #[cfg(target_arch = "wasm32")]
    clock_fn: fn() -> f64,
}

impl Timer {
    /// Creates a new timer initialized to zero and not started.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        Timer {
            time:  0.0,
            start: None,
        }
    }

    /// Creates a new timer initialized to zero and not started.
    #[cfg(target_arch = "wasm32")]
    pub fn new(time_in_sec: fn() -> f64) -> Self {
        Timer {
            time:     0.0,
            start:    None,
            clock_fn: time_in_sec,
        }
    }

    /// Start the timer.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn start(&mut self) {
        self.time = 0.0;
        self.start = Some(now());
    }

    /// Start the timer.
    #[cfg(target_arch = "wasm32")]
    pub fn start(&mut self) {
        self.time = 0.0;
        self.start = Some((self.clock_fn)());
    }

    /// Pause the timer.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn pause(&mut self) {
        if let Some(start) = self.start {
            self.time += now() - start;
        }
        self.start = None;
    }

    /// Pause the timer.
    #[cfg(target_arch = "wasm32")]
    pub fn pause(&mut self) {
        if let Some(start) = self.start {
            self.time += (self.clock_fn)() - start;
        }
        self.start = None;
    }

    /// Resume the timer.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn resume(&mut self) {
        self.start = Some(now());
    }

    /// Resume the timer.
    #[cfg(target_arch = "wasm32")]
    pub fn resume(&mut self) {
        self.start = Some((self.clock_fn)());
    }

    /// The measured time between the last `.start()` and `.pause()` calls.
    pub fn time(&self) -> f64 {
        self.time
    }
}

#[cfg(not(any(target_arch = "wasm32", target_arch = "asmjs")))]
fn now() -> f64 {
    use time;
    time::precise_time_s()
}

#[cfg(target_arch = "asmjs")]
#[allow(unused_results)] // Needed because the js macro triggers it.
fn now() -> f64 {
    use stdweb::unstable::TryInto;

    // https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
    let v = js! { return performance.now() / 1000.0; };
    v.try_into().unwrap()
}

impl Display for Timer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}s", self.time)
    }
}
