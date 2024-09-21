use std::io::{Stdout, Write};

use crossterm::{
    cursor,
    execute,
    queue,
    style::{self, Color},
    terminal,
};

use crate::{color, helper::{App, Label, Mode}, view};

struct Chunk {
    text: String,
    color: Color,
}

pub fn handle_m(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_view_mode();
    view::render_view(&app, stdout)
}

pub fn handle_j(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.modal_row = (app.modal_row + 1) % app.labels.len() as i8;
    render_modal(app, stdout)
}

pub fn handle_k(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.modal_row = (app.modal_row - 1).rem_euclid(app.labels.len() as i8);
    render_modal(app, stdout)
}

pub fn handle_c(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.mode = Mode::Color;
    color::render_color(app, stdout)
}

pub fn render_modal(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let lines = chunk_lines(app);
    let window = terminal::window_size().unwrap();

    let start_col = (window.columns as usize - lines[0][0].text.len()).div_ceil(2) as u16;
    let start_row = (window.rows as usize - lines.len()).div_ceil(2) as u16;

    execute!(stdout, cursor::SavePosition)?;
    queue!(stdout, cursor::MoveTo(start_col, start_row))?;

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
        queue!(stdout, cursor::MoveToColumn(start_col))?;
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
