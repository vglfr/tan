pub mod helper;
pub mod io;
pub mod modal;
pub mod view;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal,
};

fn main() -> std::io::Result<()> {
    let fname = std::env::args().nth(1).unwrap_or("flake.nix".to_string());
    let qname = format!("data/{fname}.tan");

    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;

    let mut app = io::load_file(&fname, &qname)?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    execute!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
    view::render_view(&app, &mut stdout)?;

    loop {
        let event = read()?;
        match event {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => match c {
                    'q' => break,
                    'w' => io::save_tan(&app)?,
                    // KeyCode::Char('x') => { break; }

                    'h' => {
                        if app.cursor_column > 0 {
                            app.cursor_column = app.cursor_column.saturating_sub(1);
                        } else if app.cursor_row > 0 {
                            app.cursor_column = app.lines[app.cursor_row.saturating_sub(1) as usize].width.saturating_sub(1);
                            app.cursor_row = app.cursor_row.saturating_sub(1);
                        }
                    },
                    'j' => {
                        if app.cursor_row < app.height.saturating_sub(1) {
                            app.cursor_row += 1;

                            if app.cursor_column >= app.lines[app.cursor_row as usize].width {
                                app.cursor_column = app.lines[app.cursor_row as usize].width.saturating_sub(1);
                            }
                        }
                    },
                    'k' => {
                        if app.cursor_row < app.height {
                            app.cursor_row = app.cursor_row.saturating_sub(1);

                            if app.cursor_column >= app.lines[app.cursor_row as usize].width {
                                app.cursor_column = app.lines[app.cursor_row as usize].width.saturating_sub(1);
                            }
                        }
                    },
                    'l' => {
                        if app.cursor_column < app.lines[app.cursor_row as usize].width.saturating_sub(1) {
                            app.cursor_column += 1;
                        } else if app.cursor_row < app.height.saturating_sub(1) {
                            app.cursor_column = 0;
                            app.cursor_row += 1;
                        }
                    },

                    // wb{} movement (later WB + g-seSE)
                    // KeyCode::Char('w') => { break; }
                    // KeyCode::Char('b') => { break; }
                    // KeyCode::Char('{') => { break; }
                    // KeyCode::Char('}') => { break; }

                    // toggle selection
                    'v' => {
                        if app.is_visual {
                            app.is_visual = false;
                            app.visual_end = app.cursor_column;
                        } else {
                            app.is_visual = true;
                            app.visual_row = app.cursor_row;
                            app.visual_start = app.cursor_column;
                            app.visual_end = app.cursor_column;
                        }
                    }

                    't' => {
                        app.tag();
                        app.is_visual = false;
                        app.visual_end = app.cursor_column;
                        view::render_view(&app, &mut stdout)?;
                    },
                    'u' => {
                        app.untag();
                        view::render_view(&app, &mut stdout)?;
                    },

                    'm' => {
                        if app.is_modal {
                            app.is_modal = false;
                            view::render_view(&app, &mut stdout)?;
                        } else {
                            app.is_modal = true;
                            modal::render_modal(&mut stdout)?;
                        }
                    },

                    _ => (),
                    },
                _ => (),
            },
            _ => (),
        }

        execute!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
        if app.is_visual && app.cursor_row == app.visual_row {
            app.visual_end = app.cursor_column;
            view::render_view(&app, &mut stdout)?;
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
