use std::io::Stdout;

use crossterm::{cursor, execute};

use crate::{helper::App, modal};

pub fn handle_key(c: char, app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.labels[app.modal_row as usize].name.push(c);
    execute!(stdout, cursor::MoveRight(1))?;
    modal::render_modal(app, stdout)
}

pub fn handle_08(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let name = &mut app.labels[app.modal_row as usize].name;
    
    if !name.is_empty() {
        name.pop();
        execute!(stdout, cursor::MoveLeft(1))?;
        modal::render_modal(app, stdout)
    } else {
        Ok(())
    }
}
