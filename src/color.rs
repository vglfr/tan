use std::io::{Stdout, Write};

use crossterm::{cursor, execute, queue, style};

use crate::{app::{self, App}, helper, modal, render};

pub fn handle_h(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.color_column = (app.color_column - 1).rem_euclid(7);
    render_color(app, stdout)
}

pub fn handle_l(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.color_column = (app.color_column + 1) % 7;
    render_color(app, stdout)
}

pub fn handle_0a(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.labels[app.modal_row].color = app::COLORS[app.color_column];
    app.set_modal_mode();

    render::render_offset(app, stdout)?;
    render::render_modal(app, stdout)
}

pub fn render_color(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    execute!(stdout, cursor::SavePosition)?;
    queue!(stdout, helper::move_to(app.window_width / 2 - 10, app.modal_start_row - 2))?;

    for (i, color) in app::COLORS.iter().enumerate() {
        queue!(
            stdout,
            style::SetBackgroundColor(*color),
            style::Print(if i == app.color_column { " • "} else { "   " }),
        )?;
    }

    execute!(stdout, cursor::RestorePosition)?;
    stdout.flush()
}
