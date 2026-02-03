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
}

impl System for Time {
  fn run(&mut self, ctx: &mut SystemResources) {
    let now = Instant::now();
    let duration = now.duration_since(self.previous_frame);
    println!("Delta time: {}", self.delta_time);
    self.delta_time = duration.as_secs_f32();
    self.previous_frame = now;
  }

  fn access(&self) -> AccessPattern {
    AccessPattern::ReadOnly
  }
}
