pub struct RendererContext {}

impl RendererContext {
  pub fn new() -> Self { Self {} }
}

impl Default for RendererContext {
  fn default() -> Self { Self::new() }
}
