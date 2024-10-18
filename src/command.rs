use std::io::{Stdout, Write};

use crossterm::{cursor, queue, terminal};

use crate::{app::{App, Mode}, io};

impl App {
    pub fn is_command_mode(&self) -> bool {
        self.mode == Mode::Command
    }

    pub fn set_command_mode(&mut self) {
        self.mode = Mode::Command;
    }

    pub fn command_char(&mut self, c: char) {
        self.command.push(c);
        self.change |= 0b0001;
    }

    pub fn command_backspace(&mut self) {
        if !self.command.is_empty() {
            self.command.pop();
            self.change |= 0b0001;
        }
    }

    pub fn command_esc(&mut self) {
        self.command.clear();
        self.set_normal_mode();
        self.change |= 0b0001;
    }
}

pub fn handle_0a(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.change |= 0b0001;
    match app.command.as_str() {
        "q" | "quit" => execute_exit(stdout),
        "w" | "write" => execute_write(app),
        "d" | "debug" => execute_debug(app),
        _ => Ok(()),
    }
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

fn execute_write(app: &mut App) -> std::io::Result<()> {
    app.command.clear();
    app.set_normal_mode();
    io::save_tan(app)
}

fn execute_debug(app: &mut App) -> std::io::Result<()> {
    app.command.clear();
    app.set_normal_mode();
    io::dump_debug(app)
}
