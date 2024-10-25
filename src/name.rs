use crate::app::{App, Mode};

impl App {
    pub fn is_name_mode(&self) -> bool {
        self.mode == Mode::Name
    }

    pub fn set_name_mode(&mut self) {
        self.modal_column = self.modal_start_column + self.labels[self.modal_row].name.len() + 12;
        self.mode = Mode::Name;
    }

    pub fn name_backspace(&mut self) {
        let name = &mut self.labels[self.modal_row].name;

        if !name.is_empty() {
            name.pop();

            self.modal_column -= 1;
            self.change |= 0b_0001_0001;
        }
    }

    pub fn name_key(&mut self, c: char) {
        let name = &mut self.labels[self.modal_row].name;

        if name.len() < 24 {
            name.push(c);

            self.modal_column += 1;
            self.change |= 0b_0001_0001;
        }
    }
}
