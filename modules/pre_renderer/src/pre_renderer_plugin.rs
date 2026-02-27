use crate::pre_renderer_context::PreRendererContext;
use crate::PreRenderer;
use core::engine::Core;
use core::engine::Phase;
use core::traits::Plugin;

pub struct PreRendererPlugin {}

impl Plugin for PreRendererPlugin {
  fn register(core: &mut Core) {
    core.add_resource(PreRendererContext::new());
    core.add_system(Phase::RenderPrepare, PreRenderer::new());
  }
}
