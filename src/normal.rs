use std::io::Stdout;

use crate::{app::{App, Mode}, render};

impl App {
    pub fn is_normal(&self) -> bool {
        self.mode == Mode::Normal
    }

    pub fn set_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn normal_v(&mut self) {
        self.set_visual_mode();
        self.visual_row = self.cursor_row + self.offset_row;
        self.visual_start = self.cursor_column;
        self.visual_end = self.cursor_column;
    }
}

pub fn handle_u(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.untag();
    render::render_offset(app, stdout)
}
