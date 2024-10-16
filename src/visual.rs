use crate::app::App;

impl App {
    pub fn visual_v(&mut self) {
        self.set_normal_mode();
        self.visual_end = self.cursor_column;
    }
}
