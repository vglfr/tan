use std::io::{Stdout, Write};

use crossterm::{
    cursor,
    execute,
    queue,
    style::{self, Color},
};

use crate::{helper::{App, Label}, view};

struct Chunk {
    text: String,
    color: Color,
}

pub fn handle_m(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_view_mode();
    view::render_view(&app, stdout)
}

pub fn render_modal(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let lines = chunk_lines(app);
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

fn chunk_lines(app: &mut App) -> Vec<Vec<Chunk>> {
    let labels = vec![
        Label { name: "label1".to_owned(), color: Color::Red },
        Label { name: "label2".to_owned(), color: Color::Yellow },
        Label { name: "label3".to_owned(), color: Color::Blue },
        Label { name: "label4".to_owned(), color: Color::Green },
    ];

    labels.iter().map(chunk_label).collect()
}

fn chunk_label(label: &Label) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    chunks.push(Chunk { text: "  ".to_owned(), color: Color::Black });
    chunks.push(Chunk { text: "      ".to_owned(), color: label.color });
    chunks.push(Chunk { text: "  ".to_owned(), color: Color::Black });
    chunks.push(Chunk { text: label.name.clone(), color: Color::White });
    chunks.push(Chunk { text: "  ".to_owned(), color: Color::Black });

    chunks
}
