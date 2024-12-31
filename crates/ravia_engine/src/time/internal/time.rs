use std::time::Duration;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

/// [`Timer`] manages the time information of the engine.
#[derive(Debug)]
pub struct Timer {
    first_frame: bool,
    frames: u64,

    start_frame: Instant,
    current_frame: Instant,
    time: Duration,
    delta: Duration,
}

impl Timer {
    /// Creates a new [`Timer`] instance.
    pub fn new() -> Self {
        Self {
            first_frame: true,
            frames: 0,
            start_frame: Instant::now(),
            current_frame: Instant::now(),
            time: Duration::ZERO,
            delta: Duration::ZERO,
        }
    }

    /// Returns the [`Time`] of the current frame.
    pub fn time(&self) -> Time {
        Time {
            frames: self.frames,
            time: self.time,
            delta: self.delta,
        }
    }

    /// Starts the time measurement.
    pub fn start(&mut self) {
        self.first_frame = false;
        self.frames = 0;
        self.start_frame = Instant::now();
        self.current_frame = Instant::now();
        self.time = Duration::ZERO;
        self.delta = Duration::ZERO;
    }

    /// Frame tick.
    pub fn frame(&mut self) {
        if self.first_frame {
            self.start();
            return;
        }

        self.frames += 1;
        self.time = self.start_frame.elapsed();
        self.delta = self.current_frame.elapsed();
        self.current_frame = Instant::now();
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

/// [`Time`] provides the time information of the engine.
#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub frames: u64,
    pub time: Duration,
    pub delta: Duration,
}

impl Time {
    pub const ZERO: Self = Self {
        frames: 0,
        time: Duration::ZERO,
        delta: Duration::ZERO,
    };

    /// Returns the time in seconds.
    pub fn seconds(&self) -> f32 {
        self.time.as_secs_f32()
    }

    /// Returns the delta time in seconds.
    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::ZERO
    }
}
