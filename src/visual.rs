use crate::{app::{App, Mode}, common};

impl App {
    pub fn is_visual_mode(&self) -> bool {
        self.mode == Mode::Visual
    }

    pub fn set_visual_mode(&mut self) {
        self.mode = Mode::Visual;
    }

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
        self.change = 0b0011;
    }

    pub fn visual_w(&mut self) {
        common::handle_w(self);
        move_visual(self);
        self.change = 0b0011;
    }

    pub fn visual_b(&mut self) {
        common::handle_b(self);
        move_visual(self);
        self.change = 0b0011;
    }
}

fn move_visual(app: &mut App) {
    if app.cursor_row + app.offset_row == app.visual_row {
        app.visual_end = app.cursor_column;
    }
}
