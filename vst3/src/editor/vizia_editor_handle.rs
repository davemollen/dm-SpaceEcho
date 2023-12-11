use baseview::WindowHandle;

pub struct ViziaEditorHandle {
  pub window: WindowHandle,
}

unsafe impl Send for ViziaEditorHandle {}
unsafe impl Sync for ViziaEditorHandle {}

impl Drop for ViziaEditorHandle {
  fn drop(&mut self) {
    self.window.close();
  }
}
