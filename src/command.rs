use std::io::{Stdout, Write};

use crossterm::{cursor, queue, style::{self, Color}, terminal};

use crate::{app::App, helper, io, render};

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
        "q" | "quit" => execute_exit(stdout),
        "w" | "write" => execute_write(app, stdout),
        "d" | "debug" => execute_debug(app, stdout),

        _ => Ok(()),
    }
}

pub fn handle_1b(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.command.clear();
    app.set_normal_mode();
    render::render_status(app, stdout)
}

pub fn render_command(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    queue!(
        stdout,
        helper::move_to(0, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print("                                                                        "),
    )?;

    queue!(
        stdout,
        helper::move_to(0, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print(":"),
    )?;

    queue!(
        stdout,
        helper::move_to(1, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print(&app.command),
    )?;

    stdout.flush()
}

#[allow(unreachable_code)]
fn execute_exit(stdout: &mut Stdout) -> std::io::Result<()> {
    queue!(stdout, terminal::LeaveAlternateScreen)?;
    queue!(stdout, cursor::Show)?;

    terminal::disable_raw_mode()?;
    stdout.flush()?;

    std::process::exit(0);
    Ok(())
}

fn execute_write(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.command.clear();
    app.set_normal_mode();

    io::save_tan(app)?;
    render::render_status(app, stdout)
}

fn execute_debug(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.command.clear();
    app.set_normal_mode();

    io::dump_debug(app)?;
    render::render_status(app, stdout)
}
