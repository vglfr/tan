use std::io::Stdout;

use crossterm::{cursor, execute};

use crate::{app::App, modal, render};

#[allow(non_snake_case)]
pub fn handle_E(app: &mut App) {
    let cursor_column = app.lines[app.nlines.saturating_sub(1)].width.saturating_sub(1);
    let cursor_row = std::cmp::min(app.nlines.saturating_sub(2), app.window_height.saturating_sub(2));

    if app.cursor_column != cursor_column {
        app.cursor_column = cursor_column;
        app.change |= 0b0101;
    }

    if app.cursor_row != cursor_row {
        app.cursor_row = cursor_row;
        app.change |= 0b1001;
    }

    let offset_row = app.nlines.saturating_sub(app.cursor_row).saturating_sub(1);

    if app.offset_row != offset_row {
        app.offset_row = offset_row;
        app.change |= 0b0011;
    }
}

#[allow(non_snake_case)]
pub fn handle_H(app: &mut App) {
    if app.cursor_row > 0 {
        app.cursor_row = 0;
        app.change |= 0b1001;
        manage_vertical_drift(app);
    }
}

#[allow(non_snake_case)]
pub fn handle_L(app: &mut App) {
    let cursor_row = std::cmp::min(app.window_height.saturating_sub(2), app.nlines.saturating_sub(2));

    if app.cursor_row != cursor_row {
        app.cursor_row = cursor_row;
        app.change |= 0b1001;
        manage_vertical_drift(app);
    }
}

#[allow(non_snake_case)]
pub fn handle_M(app: &mut App) {
    let cursor_row = std::cmp::min(app.window_height / 2, app.nlines / 2);

    if app.cursor_row != cursor_row {
        app.cursor_row = cursor_row;
        app.change |= 0b1001;
        manage_vertical_drift(app);
    }
}

#[allow(non_snake_case)]
pub fn handle_S(app: &mut App) {
    if app.cursor_column > 0 {
        app.cursor_column = 0;
        app.change |= 0b0101;
    }

    if app.cursor_row > 0 {
        app.cursor_row = 0;
        app.change |= 0b1001;
    }

    if app.offset_row > 0 {
        app.offset_row = 0;
        app.change |= 0b0011;
    }
}

pub fn handle_pg_down(app: &mut App) {
    let offset_row = std::cmp::min(app.offset_row + app.cursor_row, app.nlines.saturating_sub(app.window_height).saturating_sub(1));
    let cursor_row = std::cmp::min(app.window_height.saturating_sub(2), app.nlines.saturating_sub(1));

    if app.offset_row != offset_row {
        app.offset_row = offset_row;
        app.change |= 0b0011;
    }

    if app.cursor_row != cursor_row {
        app.cursor_row = cursor_row;
        app.change |= 0b1001;
    }

    manage_vertical_drift(app);
}

pub fn handle_pg_up(app: &mut App) {
    let offset_row = app.offset_row.saturating_sub(app.window_height.saturating_sub(app.cursor_row).saturating_sub(2));
    let cursor_row = 0;

    if app.offset_row != offset_row {
        app.offset_row = offset_row;
        app.change |= 0b0011;
    }

    if app.cursor_row != cursor_row {
        app.cursor_row = cursor_row;
        app.change |= 0b1001;
    }

    manage_vertical_drift(app);
}

pub fn handle_colon(app: &mut App) {
    app.set_command_mode();
    app.change |= 0b0001;
}

pub fn handle_h(app: &mut App) {
    if app.cursor_column > 0 {
        app.cursor_column -= 1;
        app.change |= 0b0101;
    } else if app.cursor_row > 0 {
        app.cursor_row -= 1;
        app.change |= 0b1001;
        move_to_line_end(app);
    } else if app.offset_row > 0 {
        app.offset_row -= 1;
        app.change |= 0b0011;
        move_to_line_end(app);
    }

    // move_visual(app);
}

pub fn handle_j(app: &mut App) {
    if app.cursor_row + app.offset_row < app.nlines.saturating_sub(1) {
        if app.cursor_row < app.window_height.saturating_sub(2) {
            app.cursor_row += 1;
            app.change |= 0b1001;
        } else {
            app.offset_row += 1;
            app.change |= 0b0011;
        }
    }

    if app.change & 0b1010 > 0 {
        let is_new = app.get_current_line().is_virtual;
        let is_old = app.lines[(app.cursor_row + app.offset_row).saturating_sub(1)].is_virtual;
        manage_virtual_shift(app, is_new, is_old);
    }

    manage_vertical_drift(app);
}

pub fn handle_k(app: &mut App) {
    if app.cursor_row > 0 {
        app.cursor_row -= 1;
        app.change |= 0b1001;
    } else if app.offset_row > 0 {
        app.offset_row -= 1;
        app.change |= 0b0011;
    }

    if app.change & 0b1010 > 0 {
        let is_new = app.get_current_line().is_virtual;
        let is_old = app.lines[app.cursor_row + app.offset_row + 1].is_virtual;
        manage_virtual_shift(app, is_new, is_old);
    }

    manage_vertical_drift(app);
}

pub fn handle_l(app: &mut App) {
    if app.cursor_column < app.get_current_line_width().saturating_sub(1) && app.cursor_column < app.window_width.saturating_sub(1) {
        app.cursor_column += 1;
        app.change |= 0b0101;
    } else if app.cursor_row < app.window_height.saturating_sub(2) {
        app.cursor_column = 0;
        app.cursor_row += 1;
        app.change |= 0b1101;
    } else if app.cursor_row + app.offset_row < app.nlines.saturating_sub(1) {
        app.cursor_column = 0;
        app.offset_row += 1;
        app.change |= 0b1011;
    }
}

pub fn handle_w(app: &mut App) {
    let mut line_iter = app.get_current_line().text.chars().skip(app.cursor_column + 1).peekable();

    if let Some(next) = line_iter.peek() {
        let offset = if next.is_whitespace() {
            line_iter.take_while(|x|  x.is_whitespace()).collect::<Vec<_>>().len() + 1
        } else {
            line_iter.take_while(|x| !x.is_whitespace()).collect::<Vec<_>>().len()
        };

        app.cursor_column = std::cmp::min(app.cursor_column + offset, app.get_current_line_width() - 1);
        app.change |= 0b0101;

        // move_visual(app);
    } else if app.cursor_row < std::cmp::min(app.window_height - 2, app.nlines) {
        app.cursor_row += 1;
        app.cursor_column = 0;
        app.change |= 0b1101;
    } else if app.cursor_row + app.offset_row < app.nlines - 1 {
        app.offset_row += 1;
        app.cursor_column = 0;
        app.change |= 0b0111;
    }
}

pub fn handle_b(app: &mut App) {
    let line = app.get_current_line();
    let mut line_iter = line.text.chars().rev().skip(line.width - app.cursor_column).peekable();

    if let Some(next) = line_iter.peek() {
        let offset = if next.is_whitespace() {
            line_iter.take_while(|x|  x.is_whitespace()).collect::<Vec<_>>().len() + 1
        } else {
            line_iter.take_while(|x| !x.is_whitespace()).collect::<Vec<_>>().len()
        };

        app.cursor_column -= offset;
        app.change |= 0b0101;

        // move_visual(app);
    } else if app.cursor_row > 0 {
        app.cursor_row = app.cursor_row.saturating_sub(1);
        app.cursor_column = app.get_current_line_width().saturating_sub(1);
        app.change |= 0b1101;
    } else if app.offset_row > 0 {
        app.offset_row = app.offset_row.saturating_sub(1);
        app.cursor_column = app.get_current_line_width().saturating_sub(1);
        app.change |= 0b0111;
    }
}

pub fn handle_s(app: &mut App) {
    app.cursor_column = 0;
    app.change |= 0b0100;
}

pub fn handle_e(app: &mut App) {
    app.cursor_column = std::cmp::min(app.get_current_line_width().saturating_sub(1), app.window_width.saturating_sub(1));
    app.change |= 0b0100;
}

pub fn handle_m(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_modal_mode();
    execute!(stdout, cursor::Hide)?;
    modal::render_modal(app, stdout)
}

pub fn handle_t(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.tag();
    app.set_normal_mode();

    app.visual_start = app.cursor_column;
    app.visual_end = app.visual_start;

    render::render_offset(app, stdout)?;
    render::render_status(app, stdout)
}

pub fn handle_1b(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_modal_mode();
    execute!(stdout, cursor::Hide)?;

    render::render_offset(app, stdout)?;
    modal::render_modal(app, stdout)
}

fn manage_vertical_drift(app: &mut App) {
    if app.cursor_column > app.get_current_line_width().saturating_sub(1) {
        app.cursor_column = app.get_current_line_width().saturating_sub(1);
        app.change |= 0b0101;
    }
}

fn manage_virtual_shift(app: &mut App, is_new: bool, is_old: bool) {
    if is_new != is_old {
        if is_new {
            app.cursor_column = app.cursor_column.saturating_sub(2);
        } else {
            app.cursor_column += 2;
        }
        app.change |= 0b0101;
    }
}

fn move_to_line_end(app: &mut App) {
    let cursor_column = std::cmp::min(app.get_current_line_width().saturating_sub(1), app.window_width.saturating_sub(1));

    if app.cursor_column != cursor_column {
        app.cursor_column = cursor_column;
        app.change |= 0b0100;
    }
}
