use std::io::Stdout;

use crossterm::{cursor, execute};

use crate::{app::App, modal, render};

pub fn handle_key(c: char, app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.labels[app.modal_row].name.push(c);
    let modal_start_column = app.modal_start_column;

    modal::render_modal(app, stdout)?;
    if app.modal_start_column == modal_start_column { execute!(stdout, cursor::MoveRight(1)) } else { Ok(()) }
}

pub fn handle_08(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let name = &mut app.labels[app.modal_row].name;
    
    if !name.is_empty() {
        name.pop();
        let modal_start_column = app.modal_start_column;

        render::render_offset(app, stdout)?;
        modal::render_modal(app, stdout)?;

        if app.modal_start_column == modal_start_column { execute!(stdout, cursor::MoveLeft(1)) } else { Ok(()) }
    } else {
        Ok(())
    }
}
