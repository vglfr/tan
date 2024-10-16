use crate::{app::App, common};

impl App {
    pub fn visual_v(&mut self) {
        self.set_normal_mode();
        self.visual_end = self.cursor_column;
        self.change = 0b0001;
    }

    pub fn visual_h(&mut self) {
        common::handle_h(self);
        move_visual(self);
        self.change = 0b0011; // todo render_line
    }

    pub fn visual_l(&mut self) {
        common::handle_l(self);
        move_visual(self);
        self.change = 0b0011; // todo render_line
    }
}

fn move_visual(app: &mut App) {
    if app.cursor_row + app.offset_row == app.visual_row {
        app.visual_end = app.cursor_column;
    }
}
