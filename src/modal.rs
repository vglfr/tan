use std::io::{Stdout, Write};

use crossterm::{
    cursor,
    execute,
    queue,
    style::{self, Color},
};

pub fn render_modal(stdout: &mut Stdout) -> std::io::Result<()> {
    execute!(stdout, cursor::SavePosition)?;

    queue!(
        stdout,
        cursor::MoveTo(10,10),
        style::SetBackgroundColor(Color::Black),
        style::Print("                   "),
    )?;

    queue!(
        stdout,
        cursor::MoveTo(10,11),
        style::SetBackgroundColor(Color::Black),
        style::Print("       help        "),
    )?;

    queue!(
        stdout,
        cursor::MoveTo(10,12),
        style::SetBackgroundColor(Color::Black),
        style::Print("                   "),
    )?;

    execute!(stdout, cursor::RestorePosition)?;
    stdout.flush()?;

    Ok(())
}
