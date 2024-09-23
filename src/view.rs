use std::io::{Stdout, Write};

use crossterm::{
    cursor, execute, queue, style::{self, Color}, terminal::{self, ClearType}
};
use serde::{Deserialize, Serialize};

use crate::helper::{App, Line};

#[derive(Debug, Deserialize, Serialize)]
struct Chunk {
    start: u16,
    end: u16,
    color: Color,
}

#[allow(non_snake_case)]
pub fn handle_H(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_row = 0;
    execute!(stdout, cursor::MoveToRow(0))
}

#[allow(non_snake_case)]
pub fn handle_L(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.cursor_row = app.window_height - 1;
    execute!(stdout, cursor::MoveToRow(app.window_height))
}

pub fn handle_u(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.untag();
    render_view(&app, stdout)
}

pub fn handle_v(app: &mut App) -> std::io::Result<()> {
    app.set_visual_mode();
    app.visual_row = app.cursor_row;
    app.visual_start = app.cursor_column;
    app.visual_end = app.cursor_column;
    Ok(())
}

pub fn render_view(app: &App, stdout: &mut Stdout) -> std::io::Result<()> {
    queue!(stdout, cursor::SavePosition)?;
    queue!(stdout, terminal::Clear(ClearType::All))?;

    let start = app.offset_row as usize;
    let end = std::cmp::min(app.window_height + app.offset_row, app.nlines) as usize;

    for line in &app.lines[start..end] {
        for chunk in chunk_line(line, app) {
            let text = &line.text[chunk.start.into()..chunk.end.into()];

            queue!(
                stdout,
                cursor::MoveTo(chunk.start, line.row - app.offset_row),
                style::SetBackgroundColor(chunk.color),
                style::Print(text),
            )?;
        }
    }

    queue!(stdout, cursor::RestorePosition)?;
    stdout.flush()
}

fn chunk_line(line: &Line, app: &App) -> Vec<Chunk> {
    let mut points = vec![0, std::cmp::min(line.width, app.window_width)];

    let starts = line.tags.iter().map(|x| x.start);
    let ends = line.tags.iter().map(|x| x.end);

    points.extend(starts);
    points.extend(ends);

    if app.is_visual() && app.visual_row == line.row {
        let (s,e) = app.get_visual_bounds();
        points.push(s);
        points.push(e);
    }

    points.sort();
    points.dedup();

    let chunks: Vec<Chunk> = points[1..].iter().zip(points.clone()).map(|(e,s)| {
        let color =
            if app.is_visual() && app.visual_row == line.row && s == std::cmp::min(app.visual_start, app.visual_end) {
                Color::Yellow
            } else if line.tags.iter().any(|x| s == x.start) {
                Color::Red
            } else {
                Color::Reset
            };
        Chunk { start: s, end: *e, color }
    }).collect();

    chunks
}
