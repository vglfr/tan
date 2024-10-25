use crate::app::{self, App, Label, Mode};

impl App {
    pub fn is_modal_mode(&self) -> bool {
        self.mode == Mode::Modal
    }

    pub fn set_modal_mode(&mut self) {
        self.mode = Mode::Modal;
    }

    pub fn modal_m(&mut self) {
        self.set_normal_mode();
        self.change |= 0b_0000_0011;
    }

    pub fn modal_h(&mut self) {
        let old = app::COLORS.iter().position(|x| x == &self.labels[self.modal_row].color).unwrap();
        let new = (old as i8 - 1).rem_euclid(18) as usize;

        self.labels[self.modal_row].color = app::COLORS[new];
        self.change |= 0b_0001_0011;
    }

    pub fn modal_j(&mut self) {
        self.modal_row = (self.modal_row + 1) % self.labels.len();
        self.change |= 0b_0001_1000;
    }

    pub fn modal_k(&mut self) {
        if self.modal_row > 0 {
            self.modal_row = self.modal_row - 1;
        } else {
            self.modal_row = self.labels.len() - 1;
        }
        self.change |= 0b_0001_1000;
    }

    pub fn modal_l(&mut self) {
        let old = app::COLORS.iter().position(|x| x == &self.labels[self.modal_row].color).unwrap();
        let new = (old + 1).rem_euclid(18);

        self.labels[self.modal_row].color = app::COLORS[new];
        self.change |= 0b_0001_0011;
    }

    pub fn modal_v(&mut self) {
        self.labels[self.modal_row].is_visible ^= true;
        self.change |= 0b_0001_0011;
    }

    #[allow(non_snake_case)]
    pub fn modal_V(&mut self) {
        self.labels.iter_mut().for_each(|x| x.is_visible ^= true);
        self.change |= 0b_0001_0011;
    }

    pub fn modal_a(&mut self) {
        if self.labels.len() < 24 {
            let label = Label {
                name: "new_label".to_owned(),
                color: app::COLORS[self.rng],
                is_active: false,
                is_visible: true,
            };

            self.labels.insert(self.modal_row + 1, label);
            self.lines.iter_mut().for_each(
                |x| x.tags.iter_mut().for_each(
                    |y| if y.label > self.modal_row { y.label += 1 }
                )
            );

            self.rng = (self.rng + 1) % app::COLORS.len();
            self.change |= 0b_0001_0011;
        }
    }

    pub fn modal_d(&mut self) {
        if self.labels.len() > 1 {
            self.lines.iter_mut().for_each(|x|
                x.tags = x.tags.clone().into_iter()
                    .filter(|y| y.label != self.modal_row)
                    .map(|mut y| if y.label > self.modal_row { y.label -= 1; y } else { y })
                    .collect()
            );

            self.labels.remove(self.modal_row);
            self.modal_row = self.modal_row.saturating_sub(1).rem_euclid(self.labels.len());

            if !self.labels.iter().any(|x| x.is_active) {
                self.modal_active = self.modal_row;
                self.labels[self.modal_active].is_active = true;
            }

            self.change |= 0b_0001_0011;
        }
    }

    pub fn modal_i(&mut self) {
        self.set_name_mode();
        self.change |= 0b_0000_1000;
    }

    pub fn modal_return(&mut self) {
        self.labels[self.modal_active].is_active = false;
        self.modal_active = self.modal_row;

        self.labels[self.modal_active].is_active = true;
        self.change |= 0b_0001_1000;
    }
}
