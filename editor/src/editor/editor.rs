use crate::editor::editor_window::editor_window::EditorFrame;

pub struct Editor {
 pub frame: EditorFrame,
}

impl Default for Editor {
  fn default() -> Self {
    Self {
      frame: EditorFrame::default()
    }
  }
}