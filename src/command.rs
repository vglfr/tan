use std::io::{Stdout, Write};

use crossterm::{cursor, queue, style::{self, Color}, terminal};

use crate::{common, helper::App};

pub fn handle_key(c: char, app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.command.push(c);
    render_command(app, stdout)?;
    Ok(())
}

pub fn handle_08(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if !app.command.is_empty() {
        app.command.pop();
        render_command(app, stdout)?;
    }
    Ok(())
}

pub fn handle_0a(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    match app.command.as_str() {
        "q" => execute_exit(stdout),
        "quit" => execute_exit(stdout),
        // "w" => execute_write(),
        // "write" => execute_write(),
        _ => Ok(()),
    }
    // Ok(())
}

pub fn handle_1b(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_view_mode();
    common::render_statusline(app, stdout)
}

pub fn render_command(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    // queue!(
    //     stdout,
    //     cursor::MoveTo(50, app.window_height - 1),
    //     style::SetBackgroundColor(Color::Reset),
    //     style::Print(format!("{}% {}:{}", app.cursor_column, app.cursor_row, app.cursor_column)),
    // )?;

    queue!(
        stdout,
        cursor::MoveTo(0, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print("                                                                        "),
    )?;

    queue!(
        stdout,
        cursor::MoveTo(0, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print(":"),
    )?;

    queue!(
        stdout,
        cursor::MoveTo(1, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print(&app.command),
    )?;

    // queue!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
    stdout.flush()
}

fn execute_exit(stdout: &mut Stdout) -> std::io::Result<()> {
    queue!(stdout, terminal::LeaveAlternateScreen)?;
    queue!(stdout, cursor::Show)?;

    terminal::disable_raw_mode()?;
    stdout.flush()?;

    std::process::exit(0);
    Ok(())
}

fn execute_write() {
    std::process::exit(1)
}
