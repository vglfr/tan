use crate::app::{App, Mode};

impl App {
    pub fn is_name_mode(&self) -> bool {
        self.mode == Mode::Name
    }

    pub fn set_name_mode(&mut self) {
        self.modal_column = self.modal_start_column + self.labels[self.modal_row].name.len() + 19;
        self.mode = Mode::Name;
    }

    fn get_active_label_name(&mut self) -> &mut String {
        &mut self.labels[self.modal_row].name
    }

    pub fn name_backspace(&mut self) {
        let name = self.get_active_label_name();

        if !name.is_empty() {
            name.pop();

            self.modal_column -= 1;
            self.change |= 0b_0001_1001;
        }
    }

    pub fn name_char(&mut self, c: char) {
        let name = self.get_active_label_name();

        if name.len() < 20 {
            name.push(c);

            self.modal_column += 1;
            self.change |= 0b_0001_1001;
        }
    }

    pub fn name_esc(&mut self) {
        self.set_modal_mode();
        self.change |= 0b_0001_1011;
    }
}
