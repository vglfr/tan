use std::io::{Stdout, Write};

use crossterm::{cursor, execute, queue, style::{self, Color}};

use crate::{helper::{App, Mode}, modal, view};

const COLORS: [Color; 7] = [
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
];

pub fn handle_h(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.color_column = (app.color_column - 1).rem_euclid(7);
    render_color(app, stdout)
}

pub fn handle_l(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.color_column = (app.color_column + 1) % 7;
    render_color(app, stdout)
}

pub fn handle_c_j(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.labels[app.modal_row as usize].color = COLORS[app.color_column as usize];
    app.mode = Mode::Modal;

    view::render_view(app, stdout)?;
    modal::render_modal(app, stdout)
}

pub fn render_color(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    execute!(stdout, cursor::SavePosition)?;
    queue!(stdout, cursor::MoveTo(app.window_width / 2 - 10, app.modal_start_row - 2))?;

    for (i, color) in COLORS.iter().enumerate() {
        queue!(
            stdout,
            style::SetBackgroundColor(*color),
            style::Print(if i as i8 == app.color_column { " • "} else { "   " }),
        )?;
    }

    execute!(stdout, cursor::RestorePosition)?;
    stdout.flush()?;

    Ok(())
}
