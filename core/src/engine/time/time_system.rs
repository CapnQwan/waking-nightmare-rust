use std::time::Instant;

use crate::traits::SystemUpdate;

pub struct TimeSystem {
  previous_frame: Instant,
  delta_time: f32,
}

impl TimeSystem {
  pub fn new() -> Self {
    TimeSystem {
      previous_frame: Instant::now(),
      delta_time: 0.0,
    }
  }
}

impl SystemUpdate for TimeSystem {
  fn update(&mut self) {
    let now = Instant::now();
    let duration = now.duration_since(self.previous_frame);
    self.delta_time = duration.as_secs_f32();
    self.previous_frame = now;
  }
}
