pub struct TimeContext {
  previous_frame: f32,
  delta_time: f32,
}

impl TimeContext {
  pub fn new() -> Self {
    TimeContext {
      previous_frame: 0.0,
      delta_time: 0.0,
    }
  }
  pub fn update() {}
}
