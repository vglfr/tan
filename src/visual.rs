use crate::{app::{App, Mode, Visual}, common};

impl App {
    pub fn is_visual_mode(&self) -> bool {
        self.mode == Mode::Visual
    }

    pub fn set_visual_mode(&mut self) {
        self.mode = Mode::Visual;
    }

    fn set_visual_end(&mut self) {
        self.visual.iter_mut()
            .find(|x| x.row == self.cursor_row + self.offset_row)
            .map(|x| x.end = self.cursor_column);
    }

    pub fn visual_m(&mut self) {
        self.set_modal_mode();
        self.change = 0b_0001_0000;
        // execute!(stdout, cursor::Hide)?;
        // render::render_modal(app, stdout)
    }

    pub fn visual_v(&mut self) {
        self.set_normal_mode();
        self.set_visual_end();
        self.change = 0b0001;
    }

    pub fn visual_h(&mut self) {
        common::handle_h(self);
        self.set_visual_end();
        self.change = 0b0010;
    }

    pub fn visual_j(&mut self) {
        common::handle_j(self);

        if self.change != 0 {
            self.visual.iter_mut()
                .find(|x| x.row == self.cursor_row + self.offset_row - 1)
                .map(|x| x.end = self.lines[self.cursor_row + self.offset_row - 1].width - 1);

            if let Some(region) = self.visual.iter_mut().find(|x| x.row == self.cursor_row + self.offset_row) {
                region.end = self.cursor_column;
                self.visual.remove(0);
            } else {
                let region = Visual {
                    start: 0,
                    end: self.cursor_column,
                    row: self.cursor_row + self.offset_row,
                };
                self.visual.push(region);
            }

            self.change = 0b0010;
        }
    }

    pub fn visual_k(&mut self) {
        common::handle_k(self);

        if self.change != 0 {
            self.visual.iter_mut()
                .find(|x| x.row == self.cursor_row + self.offset_row + 1)
                .map(|x| x.end = 0);

            if let Some(region) = self.visual.iter_mut().find(|x| x.row == self.cursor_row + self.offset_row) {
                region.end = self.cursor_column;
                self.visual.pop();
            } else {
                let region = Visual {
                    start: self.get_current_line_width() - 1,
                    end: self.cursor_column,
                    row: self.cursor_row + self.offset_row,
                };
                self.visual.insert(0, region);
            }

            self.change = 0b0010;
        }
    }

    pub fn visual_l(&mut self) {
        common::handle_l(self);
        self.set_visual_end();
        self.change = 0b0010;
    }

    pub fn visual_s(&mut self) {
        common::handle_s(self);
        self.set_visual_end();
        self.change = 0b0010;
    }

    pub fn visual_e(&mut self) {
        common::handle_e(self);
        self.set_visual_end();
        self.change = 0b0010;
    }

    pub fn visual_w(&mut self) {
        common::handle_w(self);
        self.set_visual_end();
        self.change = 0b0010;
    }

    pub fn visual_b(&mut self) {
        common::handle_b(self);
        self.set_visual_end();
        self.change = 0b0010;
    }
}
