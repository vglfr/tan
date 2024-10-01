use std::io::{Stdout, Write};

use crossterm::{cursor, execute, queue, style::{self, Color}};

use crate::{command, helper::{App, Mode}, modal, normal};

#[allow(non_snake_case)]
pub fn handle_E(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_column = app.lines[app.nlines as usize - 1].width - 1;
    app.offset_column = 0; // todo

    app.cursor_row = std::cmp::min(app.nlines - 2, app.window_height - 2);
    app.offset_row = app.nlines - app.cursor_row - 1;

    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

#[allow(non_snake_case)]
pub fn handle_H(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_row = 0;
    manage_vertical_overflow(app);

    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

#[allow(non_snake_case)]
pub fn handle_L(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_row = std::cmp::min(app.window_height - 2, app.nlines - 2);
    manage_vertical_overflow(app);

    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

#[allow(non_snake_case)]
pub fn handle_M(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_row = std::cmp::min(app.window_height / 2, app.nlines / 2);
    manage_vertical_overflow(app);

    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

#[allow(non_snake_case)]
pub fn handle_S(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_column = 0;
    app.offset_column = 0;

    app.cursor_row = 0;
    app.offset_row = 0;

    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

pub fn handle_pg_down(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.offset_row = std::cmp::min(app.offset_row + app.cursor_row, app.nlines.saturating_sub(app.window_height - 1));
    app.cursor_row = std::cmp::min(app.window_height - 2, app.nlines - 1);

    manage_vertical_overflow(app);

    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

pub fn handle_pg_up(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.offset_row = app.offset_row.saturating_sub(app.window_height - app.cursor_row - 2);
    app.cursor_row = 0;

    manage_vertical_overflow(app);

    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

pub fn handle_colon(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_command_mode();
    command::render_command(app, stdout)
}

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
    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

pub fn handle_j(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_row + app.offset_row < app.nlines - 1 && app.cursor_row < app.window_height - 2 {
        app.cursor_row += 1;
    } else if app.cursor_row + app.offset_row < app.nlines - 1 {
        app.offset_row += 1;
    }

    manage_vertical_overflow(app);
    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

pub fn handle_k(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_row > 0 {
        app.cursor_row -= 1;
    } else if app.offset_row > 0 {
        app.offset_row -= 1;
    }

    manage_vertical_overflow(app);
    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

pub fn handle_l(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_column + app.offset_column < app.get_current_line_width() - 1 && app.cursor_column < app.window_width - 1 {
        app.cursor_column += 1;
    } else if app.cursor_column + app.offset_column < app.get_current_line_width() - 1 {
        app.offset_column += 1;
    } else if app.cursor_row < app.window_height - 2 {
        app.offset_column = 0;
        app.cursor_column = 0;
        app.cursor_row += 1;
    } else if app.cursor_row + app.offset_row < app.nlines - 1 {
        app.offset_column = 0;
        app.cursor_column = 0;
        app.offset_row += 1;
    }

    move_visual(app);
    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

pub fn handle_w(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let current_column = app.cursor_column + app.offset_column;
    let mut line_iter = app.get_current_line().text.chars().skip(current_column as usize + 1).peekable();

    if let Some(next) = line_iter.peek() {
        let offset = if next.is_whitespace() {
            line_iter.take_while(|x| x.is_whitespace()).collect::<Vec<_>>().len() + 1
        } else {
            line_iter.take_while(|x| !x.is_whitespace()).collect::<Vec<_>>().len()
        };

        app.cursor_column += offset as u16;

        move_visual(app);
        normal::render_normal(app, stdout)?;
        render_statusline(app, stdout)
    } else {
        Ok(())
    }
}

pub fn handle_b(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let current_column = app.cursor_column + app.offset_column;
    let line = app.get_current_line();

    let mut line_iter = line.text.chars().rev().skip((line.width - current_column) as usize).peekable();

    if let Some(next) = line_iter.peek() {
        let offset = if next.is_whitespace() {
            line_iter.take_while(|x| x.is_whitespace()).collect::<Vec<_>>().len() + 1
        } else {
            line_iter.take_while(|x| !x.is_whitespace()).collect::<Vec<_>>().len()
        };

        app.cursor_column -= offset as u16;

        move_visual(app);
        normal::render_normal(app, stdout)?;
        render_statusline(app, stdout)
    } else {
        Ok(())
    }
}

pub fn handle_s(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_column = 0;
    app.offset_column = 0;

    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

pub fn handle_e(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    manage_horizontal_overflow(app);
    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
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

    normal::render_normal(app, stdout)?;
    render_statusline(app, stdout)
}

pub fn handle_1b(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_modal_mode();
    execute!(stdout, cursor::Hide)?;

    normal::render_normal(app, stdout)?;
    modal::render_modal(app, stdout)
}

fn manage_horizontal_overflow(app: &mut App) {
    app.cursor_column = std::cmp::min(app.get_current_line_width() - 1, app.window_width - 1);
    app.offset_column = app.get_current_line_width() - app.cursor_column - 1;
}

fn manage_vertical_overflow(app: &mut App) {
    if app.get_current_line_width() - 1 < app.offset_column {
        app.cursor_column = 0;
        app.offset_column = app.get_current_line_width() - 1;
    } else if app.get_current_line_width() - 1 < app.offset_column + app.cursor_column {
        app.cursor_column = app.get_current_line_width() - app.offset_column - 1;
    }
}

fn move_visual(app: &mut App) {
    if app.is_visual() && app.cursor_row + app.offset_row == app.visual_row {
        app.visual_end = app.cursor_column;
    }
}

pub fn render_statusline(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let mode_color = match app.mode {
        Mode::Color => Color::Yellow,
        Mode::Command => Color::Red,
        Mode::Modal => Color::Yellow,
        Mode::Name => Color::Red,
        Mode::Normal => Color::White,
        Mode::Visual => Color::Blue,
    };

    let status = format!(
        "{}% {}:{}",
        (app.cursor_row + app.offset_row) / app.nlines,
        app.cursor_row + app.offset_row,
        app.cursor_column + app.offset_column,
    );
    
    queue!(
        stdout,
        cursor::MoveTo(0, app.window_height - 1),
        style::SetBackgroundColor(mode_color),
        style::Print("     "),
    )?;

    let col = app.cursor_column;

    let label = app.get_current_line().tags.iter()
        .find(|x| x.start <= col && col < x.end)
        .map(|x| x.label);

    if let Some(n) = label {
        queue!(
            stdout,
            cursor::MoveTo(8, app.window_height - 1),
            style::SetBackgroundColor(app.labels[n].color),
            style::Print("      "),
        )?;

        queue!(
            stdout,
            cursor::MoveTo(16, app.window_height - 1),
            style::SetBackgroundColor(Color::Reset),
            style::Print(&app.labels[n].name),
        )?;
    }

    // queue!(
    //     stdout,
    //     cursor::MoveTo(8, app.window_height - 1),
    //     style::SetBackgroundColor(Color::Reset),
    //     style::Print(&app.fname),
    // )?;

    queue!(
        stdout,
        cursor::MoveTo(70, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print(status),
    )?;

    queue!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
    stdout.flush()
}
