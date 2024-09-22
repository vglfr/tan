use std::io::Stdout;

use crossterm::{cursor, execute};

use crate::{helper::App, modal, view};

pub fn handle_h(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_column > 0 {
        app.cursor_column = app.cursor_column.saturating_sub(1);
    } else if app.cursor_row > 0 {
        app.cursor_column = app.lines[app.cursor_row.saturating_sub(1) as usize].width.saturating_sub(1);
        app.cursor_row = app.cursor_row.saturating_sub(1);
    }
    render_move(app, stdout)
}

pub fn handle_j(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_row < app.height.saturating_sub(1) {
        app.cursor_row += 1;

        if app.cursor_column >= app.lines[app.cursor_row as usize].width {
            app.cursor_column = app.lines[app.cursor_row as usize].width.saturating_sub(1);
        }
    }
    render_move(app, stdout)
}

pub fn handle_k(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_row < app.height {
        app.cursor_row = app.cursor_row.saturating_sub(1);

        if app.cursor_column >= app.lines[app.cursor_row as usize].width {
            app.cursor_column = app.lines[app.cursor_row as usize].width.saturating_sub(1);
        }
    }
    render_move(app, stdout)
}

pub fn handle_l(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_column < app.lines[app.cursor_row as usize].width.saturating_sub(1) {
        app.cursor_column += 1;
    } else if app.cursor_row < app.height.saturating_sub(1) {
        app.cursor_column = 0;
        app.cursor_row += 1;
    }
    render_move(app, stdout)
}

pub fn handle_m(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_modal_mode();
    execute!(stdout, cursor::Hide)?;
    modal::render_modal(app, stdout)
}

pub fn handle_t(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.tag();
    app.set_view_mode();

    app.visual_end = app.cursor_column;
    view::render_view(app, stdout)
}

pub fn handle_1b(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_modal_mode();
    execute!(stdout, cursor::Hide)?;

    view::render_view(app, stdout)?;
    modal::render_modal(app, stdout)
}

fn render_move(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    execute!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
    if app.is_visual() && app.cursor_row == app.visual_row {
        app.visual_end = app.cursor_column;
        view::render_view(app, stdout)
    } else {
        Ok(())
    }
}
