use std::io::{Stdout, Write};

use crossterm::{
    cursor, queue, style::{self, Color}, terminal::{self, ClearType}
};
use serde::{Deserialize, Serialize};

use crate::helper::{App, Line};

#[derive(Debug, Deserialize, Serialize)]
struct Chunk {
    start: u16,
    end: u16,
    color: Color,
}

pub fn handle_u(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.untag();
    render_normal(app, stdout)
}

pub fn handle_v(app: &mut App) {
    app.set_visual_mode();
    app.visual_row = app.cursor_row + app.offset_row;
    app.visual_start = app.cursor_column;
    app.visual_end = app.cursor_column;
}

pub fn render_normal(app: &App, stdout: &mut Stdout) -> std::io::Result<()> {
    queue!(stdout, terminal::Clear(ClearType::All))?;

    let start = app.offset_row as usize;
    let end = std::cmp::min(app.window_height + app.offset_row - 1, app.nlines) as usize;

    for line in &app.lines[start..end] {
        if line.is_virtual {
            queue!(
                stdout,
                cursor::MoveTo(0, line.virtual_row - app.offset_row),
                style::SetForegroundColor(Color::DarkGrey),
                style::Print("⤷ "),
            )?;
        }

        for chunk in chunk_line(line, app) {
            let text = &line.text[chunk.start.into()..chunk.end.into()];

            queue!(
                stdout,
                cursor::MoveTo(chunk.start + if line.is_virtual { 2 } else { 0 }, line.virtual_row - app.offset_row),
                style::SetForegroundColor(Color::White),
                style::SetBackgroundColor(chunk.color),
                style::Print(text),
            )?;
        }
    }

    queue!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
    stdout.flush()
}

fn chunk_line(line: &Line, app: &App) -> Vec<Chunk> {
    let mut points = vec![0, line.width];

    let starts = line.tags.iter().map(|x| x.start);
    let ends = line.tags.iter().map(|x| x.end);

    points.extend(starts);
    points.extend(ends);

    if app.visual_row == line.virtual_row {
        let (s,e) = app.get_visual_bounds();
        points.push(s);
        points.push(e);
    }

    points.sort();
    points.dedup();

    points = points.into_iter().filter(
        |x| *x < std::cmp::min(app.window_width, line.width)
    ).collect();

    points.insert(0, 0);
    points.push(std::cmp::min(app.window_width, line.width));

    let chunks = points[1..].iter().zip(points.clone()).filter_map(|(e,s)| {
        let tag = line.tags.iter().find_map(|x| if s == x.start { Some(x.label) } else { None });
        let color =
            if tag.is_some() {
                let label = &app.labels[tag.unwrap()];
                if label.is_visible { label.color } else { Color::Reset }
            } else if app.visual_row == line.virtual_row && s == app.get_visual_bounds().0 && app.visual_start != app.visual_end {
                Color::Yellow
            } else {
                Color::Reset
            };
        if *e > s { Some(Chunk { start: s, end: *e, color }) } else { None }
    }).collect();

    chunks
}
