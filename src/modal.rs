use std::io::Stdout;

use crossterm::{cursor, execute};

use crate::{app::{self, App, Label}, color, helper};

impl App {
    pub fn modal_m(&mut self) {
        self.set_normal_mode();
        self.change |= 0b_0000_0011;
    }

    pub fn modal_j(&mut self) {
        self.modal_row = (self.modal_row + 1) % self.labels.len();
        self.change |= 0b_0001_0000;
    }

    pub fn modal_k(&mut self) {
        if self.modal_row > 0 {
            self.modal_row = self.modal_row - 1;
        } else {
            self.modal_row = self.labels.len() - 1;
        }
        self.change |= 0b_0001_0000;
    }

    pub fn modal_h(&mut self) {
        self.labels[self.modal_row].is_visible ^= true;
        self.change |= 0b_0001_0011;
    }

    #[allow(non_snake_case)]
    pub fn modal_H(&mut self) {
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

    pub fn modal_return(&mut self) {
        self.labels[self.modal_active].is_active = false;
        self.modal_active = self.modal_row;

        self.labels[self.modal_active].is_active = true;
        self.change |= 0b_0001_0000;
    }
}

pub fn handle_i(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_name_mode();

    let col = app.modal_start_column + app.labels[app.modal_row].name.len() + 17;
    let row = app.modal_start_row + app.modal_row + 1;

    execute!(stdout, helper::move_to(col, row))?;
    execute!(stdout, cursor::Show)
}

pub fn handle_c(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_color_mode();
    app.color_column = app::COLORS.iter().position(|x| x == &app.labels[app.modal_row].color).unwrap();
    color::render_color(app, stdout)
}
