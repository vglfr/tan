use crate::{app::{App, Mode, Visual}, common};

impl App {
    pub fn is_normal_mode(&self) -> bool {
        self.mode == Mode::Normal
    }

    pub fn set_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn normal_h(&mut self) {
        common::handle_h(self);
    }

    pub fn normal_j(&mut self) {
        common::handle_j(self);
    }

    pub fn normal_k(&mut self) {
        common::handle_k(self);
    }

    pub fn normal_l(&mut self) {
        common::handle_l(self);
    }

    pub fn normal_w(&mut self) {
        common::handle_w(self);
    }

    pub fn normal_b(&mut self) {
        common::handle_b(self);
    }

    pub fn normal_v(&mut self) {
        self.visual.clear();
        self.set_visual_mode();

        let region = Visual {
            start: self.cursor_column,
            end: self.cursor_column,
            row: self.cursor_row + self.offset_row,
        };
        self.visual.push(region);

        self.change = 0b0001;
    }

    pub fn normal_u(&mut self) {
        self.untag();
    }
}
