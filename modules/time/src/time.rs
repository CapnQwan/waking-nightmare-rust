use core::engine::Phase;
use core::traits::{AccessPattern, System, SystemResources};
use std::time::Instant;

pub struct Time {
  previous_frame: Instant,
  delta_time: f32,
}

impl Time {
  pub fn new() -> Self {
    Time {
      previous_frame: Instant::now(),
      delta_time: 0.0,
    }
  }

  /// Delta time (seconds) measured between the two most recent ticks
  pub fn delta_time(&self) -> f32 {
    self.delta_time
  }

  /// Updates internal timing state (called by the ECS `run()`).
  pub fn tick(&mut self) {
    let now = Instant::now();
    let duration = now.duration_since(self.previous_frame);

    self.delta_time = duration.as_secs_f32();
    self.previous_frame = now;
  }
}

impl System for Time {
  fn run(&mut self, _ctx: &mut SystemResources) {
    self.tick();
  }

  fn access(&self) -> AccessPattern {
    AccessPattern::ReadOnly
  }
}
