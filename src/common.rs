use std::io::Stdout;

use crossterm::{cursor, execute};

use crate::{helper::App, modal, view};

pub fn handle_h(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_column > 0 {
        app.cursor_column -= 1;
    } else if app.offset_column > 0 {
        app.offset_column -= 1;
    } else if app.cursor_row > 0 {
        app.cursor_row -= 1;
        manage_horizontal_overflow(app);
    } else if app.offset_row > 0 {
        app.offset_row -= 1;
        manage_horizontal_overflow(app);
    }

    move_visual(app);
    view::render_view(app, stdout)
}

pub fn handle_j(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_row + app.offset_row < app.nlines - 1 && app.cursor_row < app.window_height - 1 {
        app.cursor_row += 1;
    } else if app.cursor_row + app.offset_row < app.nlines - 1 {
        app.offset_row += 1;
    }

    view::manage_vertical_overflow(app);
    view::render_view(app, stdout)
}

pub fn handle_k(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_row > 0 {
        app.cursor_row -= 1;
    } else if app.offset_row > 0 {
        app.offset_row -= 1;
    }

    view::manage_vertical_overflow(app);
    view::render_view(app, stdout)
}

pub fn handle_l(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_column + app.offset_column < app.current_linewidth() - 1 && app.cursor_column < app.window_width - 1 {
        app.cursor_column += 1;
    } else if app.cursor_column + app.offset_column < app.current_linewidth() - 1 {
        app.offset_column += 1;
    } else if app.cursor_row < app.window_height - 1 {
        app.offset_column = 0;
        app.cursor_column = 0;
        app.cursor_row += 1;
    } else if app.cursor_row + app.offset_row < app.nlines - 1 {
        app.offset_column = 0;
        app.cursor_column = 0;
        app.offset_row += 1;
    }

    move_visual(app);
    view::render_view(app, stdout)
}

pub fn handle_s(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_column = 0;
    app.offset_column = 0;

    view::render_view(app, stdout)
}

pub fn handle_e(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    manage_horizontal_overflow(app);
    view::render_view(app, stdout)
}

pub fn handle_m(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_modal_mode();
    execute!(stdout, cursor::Hide)?;
    modal::render_modal(app, stdout)
}

pub fn handle_t(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.tag();
    app.set_view_mode();

    app.visual_start = app.cursor_column;
    app.visual_end = app.visual_start;

    view::render_view(app, stdout)
}

pub fn handle_1b(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_modal_mode();
    execute!(stdout, cursor::Hide)?;

    view::render_view(app, stdout)?;
    modal::render_modal(app, stdout)
}

fn manage_horizontal_overflow(app: &mut App) {
    app.cursor_column = std::cmp::min(app.current_linewidth() - 1, app.window_width - 1);
    app.offset_column = app.current_linewidth() - app.cursor_column - 1;
}

fn move_visual(app: &mut App) {
    if app.is_visual() && app.cursor_row + app.offset_row == app.visual_row {
        app.visual_end = app.cursor_column;
    }
}
