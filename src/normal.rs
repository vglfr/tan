use crate::{app::{App, Mode}, common};

impl App {
    pub fn is_normal(&self) -> bool {
        self.mode == Mode::Normal
    }

    pub fn set_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn normal_h(&mut self) {
        common::handle_h(self);
    }

    pub fn normal_l(&mut self) {
        common::handle_l(self);
    }

    pub fn normal_v(&mut self) {
        self.set_visual_mode();

        self.visual_row = self.cursor_row + self.offset_row;
        self.visual_start = self.cursor_column;
        self.visual_end = self.cursor_column;

        self.change = 0b0001;
    }

    pub fn normal_u(&mut self) {
        self.untag();
        self.change = 0b0011;
    }
}
