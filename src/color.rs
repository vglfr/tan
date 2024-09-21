use std::io::{Stdout, Write};

use crossterm::{cursor, execute, queue, style::{self, Color}};

use crate::helper::App;

pub fn handle_j(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    // app.modal_row = (app.modal_row + 1) % app.labels.len() as i8;
    // render_modal(app, stdout)
    Ok(())
}

pub fn render_color(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let colors = vec![
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
    ];

    execute!(stdout, cursor::SavePosition)?;
    queue!(stdout, cursor::MoveTo(5,5))?;

    for color in colors {
        queue!(
            stdout,
            style::SetBackgroundColor(color),
            style::Print(" • "),
        )?;
    }

    execute!(stdout, cursor::RestorePosition)?;
    stdout.flush()?;

    Ok(())
}
