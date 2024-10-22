use std::io::Stdout;

use crossterm::{cursor, execute};

use crate::{app::{self, App, Label}, color, helper};

impl App {
    pub fn modal_m(&mut self) {
        self.set_normal_mode();
        self.change |= 0b_0000_0011;
        // execute!(stdout, cursor::Show)?;
        // render::render_offset(self, stdout)
    }
}

pub fn handle_a(app: &mut App) {
    if app.labels.len() < 24 {
        let label = Label {
            name: "new_label".to_owned(),
            color: app::COLORS[app.rng],
            is_active: false,
            is_visible: true,
        };

        app.rng = (app.rng + 1) % app::COLORS.len();
        app.labels.push(label);

        app.change |= 0b_0001_0000;
        // render_modal(app, stdout)
    }
}

pub fn handle_d(app: &mut App) {
    // handle normal
    if app.labels.len() > 1 {
        app.labels.remove(app.modal_row);
        app.modal_row = (app.modal_row - 1).rem_euclid(app.labels.len());

        app.change |= 0b_0001_0000;
        // render_modal(app, stdout)
    }
}

pub fn handle_j(app: &mut App) {
    app.modal_row = (app.modal_row + 1) % app.labels.len();

    app.change |= 0b_0001_0000;
    // render_modal(app, stdout)
}

pub fn handle_k(app: &mut App) {
    if app.modal_row > 0 {
        app.modal_row = app.modal_row - 1;
    } else {
        app.modal_row = app.labels.len() - 1;
    }
    app.change |= 0b_0001_0000;
    // render_modal(app, stdout)
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

pub fn handle_h(app: &mut App) {
    app.labels[app.modal_row].is_visible ^= true;

    app.change |= 0b_0001_0000;
    // render::render_offset(app, stdout)?;
    // render_modal(app, stdout)
}

pub fn handle_esc(app: &mut App) {
    app.labels[app.modal_active].is_active = false;
    app.modal_active = app.modal_row;

    app.labels[app.modal_active].is_active = true;
    // self.change |= 0b_0001_0000;
    // render_modal(app, stdout)
}
