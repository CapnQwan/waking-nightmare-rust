use std::time::Instant;

use crate::traits::Update;

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

impl Update for Time {
  fn update(&mut self) {
    let now = Instant::now();
    let duration = now.duration_since(self.previous_frame);
    self.delta_time = duration.as_secs_f32();
    self.previous_frame = now;
  }
}
