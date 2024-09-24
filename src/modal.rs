use std::io::{Stdout, Write};

use crossterm::{
    cursor,
    execute,
    queue,
    style::{self, Color},
};

use crate::{color, helper::{self, App, Label}, view};

struct Chunk {
    text: String,
    color: Color,
}

pub fn handle_m(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_view_mode();
    execute!(stdout, cursor::Show)?;
    view::render_view(app, stdout)
}

pub fn handle_a(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.labels.len() < 12 {
        app.labels.push(Label { name: "new_label".to_owned(), color: Color::Red });
        render_modal(app, stdout)
    } else {
        Ok(())
    }
}

pub fn handle_d(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.labels.len() > 1 {
        app.labels.remove(app.modal_row as usize);
        app.modal_row = (app.modal_row - 1).rem_euclid(app.labels.len() as i8);
        render_modal(app, stdout)
    } else {
        Ok(())
    }
}

pub fn handle_j(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.modal_row = (app.modal_row + 1) % app.labels.len() as i8;
    render_modal(app, stdout)
}

pub fn handle_k(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.modal_row = (app.modal_row - 1).rem_euclid(app.labels.len() as i8);
    render_modal(app, stdout)
}

pub fn handle_n(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_name_mode();

    let col = app.modal_start_column + app.labels[app.modal_row as usize].name.len() as u16 + 14;
    let row = app.modal_start_row + app.modal_row as u16 + 1;

    execute!(stdout, cursor::MoveTo(col, row))?;
    execute!(stdout, cursor::Show)
}

pub fn handle_c(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_color_mode();
    app.color_column = helper::COLORS.iter().position(|x| x == &app.labels[app.modal_row as usize].color).unwrap() as i8;
    color::render_color(app, stdout)
}

pub fn render_modal(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let lines = chunk_lines(app);

    execute!(stdout, cursor::SavePosition)?;
    queue!(stdout, cursor::MoveTo(app.modal_start_column, app.modal_start_row))?;

    for (i, line) in lines.iter().enumerate() {
        for chunk in line {
            queue!(
                stdout,
                style::SetBackgroundColor(chunk.color),
                style::SetForegroundColor(if i as i8 == app.modal_row + 1 { Color::Yellow } else { Color::White }),
                style::Print(&chunk.text),
            )?;
        }

        queue!(stdout, cursor::MoveDown(1))?;
        queue!(stdout, cursor::MoveToColumn(app.modal_start_column))?;
    }

    execute!(stdout, cursor::RestorePosition)?;
    stdout.flush()?;

    Ok(())
}

fn chunk_lines(app: &mut App) -> Vec<Vec<Chunk>> {
    let width = app.labels.iter().fold(0, |acc, x| std::cmp::max(acc, x.name.len()));
    let mut lines = app.labels.iter().map(|x| chunk_label(x, width)).collect::<Vec<Vec<Chunk>>>();

    lines.insert(0, vec![Chunk { text: format!("{:width$}", "", width=width+16), color: Color::Black }]);
    lines.push(vec![Chunk { text: format!("{:width$}", "", width=width+16), color: Color::Black }]);

    app.modal_start_column = (app.window_width - lines[0][0].text.len() as u16) / 2;
    app.modal_start_row = (app.window_height - lines.len() as u16) / 2;

    lines
}

fn chunk_label(label: &Label, width: usize) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    chunks.push(Chunk { text: "  ".to_owned(), color: Color::Black });
    chunks.push(Chunk { text: "        ".to_owned(), color: label.color });
    chunks.push(Chunk { text: "    ".to_owned(), color: Color::Black });
    chunks.push(Chunk { text: format!("{:width$}", label.name, width=width), color: Color::Black });
    chunks.push(Chunk { text: "  ".to_owned(), color: Color::Black });

    chunks
}
