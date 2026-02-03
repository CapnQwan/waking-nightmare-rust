use crate::time_context::TimeContext;
use core::engine::Core;
use core::engine::Phase;
use core::traits::Plugin;
use crate::Time;

pub struct TimePlugin {}

impl Plugin for TimePlugin {
  fn register(core: &mut Core) {
    core.add_resource(TimeContext::new());
    core.add_system(Phase::FrameStart, Time::new());
  }
}
