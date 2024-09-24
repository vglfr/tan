use std::io::Stdout;

use crossterm::{cursor, execute};

use crate::{helper::App, modal, view};

pub fn handle_h(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    // move cursor one left
    // move screen one left
    // move to the end of previous line and adjust screen to move right if needed
    // same with scrolling up
    // do nothing if start of file

    if app.cursor_column > 0 {
        app.cursor_column = app.cursor_column - 1;
        render_move(app, stdout)
    } else if app.offset_column > 0 {
        app.offset_column = app.offset_column - 1;
        view::render_view(app, stdout)
    } else if app.cursor_row > 0 {
        app.cursor_row = app.cursor_row - 1;
        handle_e(app, stdout)?;
        // let width = app.lines[(app.cursor_row + app.offset_row) as usize].width;

        // app.cursor_column = std::cmp::min(width - 1, app.window_width - 1);
        // app.offset_column = width - app.cursor_column - 1;

        // view::render_view(app, stdout)?;
        render_move(app, stdout)
    } else {
        Ok(())
    }
}

pub fn handle_j(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_row + app.offset_row < app.nlines - 1 {
        if app.cursor_row < app.window_height - 1 {
            app.cursor_row += 1;
            let width = app.lines[app.cursor_row as usize].width;

            if app.cursor_column > width - 1 {
                if app.offset_column > width - 1 {
                    app.cursor_column = 0;
                    app.offset_column = width - 1;
                    view::render_view(app, stdout)?;
                } else {
                    app.cursor_column = width.saturating_sub(1);
                }
            }
            render_move(app, stdout)
        } else {
            app.offset_row += 1;
            view::render_view(app, stdout)
        }
    } else {
        Ok(())
    }
}

pub fn handle_k(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.cursor_row > 0 {
        app.cursor_row = app.cursor_row.saturating_sub(1);
        let width = app.lines[app.cursor_row as usize].width;

        if app.cursor_column > width - 1 {
            if app.offset_column > width - 1 {
                app.cursor_column = 0;
                app.offset_column = width - 1;
                view::render_view(app, stdout)?;
            } else {
                app.cursor_column = width.saturating_sub(1);
            }
        }

        render_move(app, stdout)
    } else {
        app.offset_row = app.offset_row.saturating_sub(1);
        view::render_view(app, stdout)
    }
}

pub fn handle_l(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    // move cursor one right
    // move screen one right
    // move to the start of next line and adjust screen to move left if needed
    // do nothing if end of file

    if app.cursor_column < app.window_width - 1 {
        app.cursor_column += 1;
        render_move(app, stdout)
    } else if app.cursor_column + app.offset_column < app.lines[(app.cursor_row + app.offset_row) as usize].width - 1 {
    // } else if app.cursor_column + app.offset_column < {
        app.offset_column += 1;
        view::render_view(app, stdout)
    } else if app.cursor_row + app.offset_row < app.nlines - 1 {
        app.offset_column = 0;
        app.cursor_column = 0;
        app.cursor_row += 1;

        view::render_view(app, stdout)?;
        render_move(app, stdout)
    } else {
        Ok(())
    }

    // if app.cursor_column + app.offset_column < app.lines[(app.cursor_row + app.offset_row) as usize].width - 1 {
    //     if app.cursor_column < app.window_width - 1 {
    //         app.cursor_column += 1;
    //         render_move(app, stdout)
    //     } else {
    //         app.offset_column += 1;
    //         view::render_view(app, stdout)
    //     }
    // } else {
    //     Ok(())
    // }

    // if app.cursor_column < app.lines[app.cursor_row as usize].width.saturating_sub(1) {
    //     app.cursor_column += 1;
    // } else if app.cursor_row < app.nlines.saturating_sub(1) {
    //     app.cursor_column = 0;
    //     app.cursor_row += 1;
    // }
}

pub fn handle_s(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_column = 0;
    app.offset_column = 0;

    view::render_view(app, stdout)?;
    execute!(stdout, cursor::MoveToColumn(app.cursor_column))
}

pub fn handle_e(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_column = std::cmp::min(app.lines[(app.cursor_row + app.offset_row) as usize].width - 1, app.window_width - 1);
    app.offset_column = app.lines[(app.cursor_row + app.offset_row) as usize].width - app.cursor_column - 1;

    view::render_view(app, stdout)?;
    execute!(stdout, cursor::MoveToColumn(app.cursor_column))
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
