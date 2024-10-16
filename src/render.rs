use std::io::{Stdout, Write};

use crossterm::{execute, queue, style::{self, Color}, terminal::{self, ClearType}};
use serde::{Deserialize, Serialize};

use crate::{app::{App, Line, Mode}, helper};

#[derive(Debug, Deserialize, Serialize)]
struct Chunk {
    start: usize,
    end: usize,
    color: Color,
}

pub fn render_cursor(app: &App, stdout: &mut Stdout) -> std::io::Result<()> {
    let cursor_column = if app.get_current_line().is_virtual { app.cursor_column + 2 } else { app.cursor_column };
    execute!(stdout, helper::move_to(cursor_column, app.cursor_row))
}

pub fn render_offset(app: &App, stdout: &mut Stdout) -> std::io::Result<()> {
    queue!(stdout, terminal::Clear(ClearType::All))?;

    let start = app.offset_row as usize;
    let end = std::cmp::min(app.window_height + app.offset_row - 1, app.nlines);

    for line in &app.lines[start..end] {
        if line.is_virtual {
            queue!(
                stdout,
                helper::move_to(0, line.virtual_row - app.offset_row),
                style::SetBackgroundColor(Color::Reset),
                style::SetForegroundColor(Color::DarkGrey),
                style::Print("⤷ "),
            )?;
        }

        for chunk in chunk_line(line, app) {
            let text = &line.text[chunk.start.into()..chunk.end.into()];

            queue!(
                stdout,
                helper::move_to(chunk.start + if line.is_virtual { 2 } else { 0 }, line.virtual_row - app.offset_row),
                style::SetForegroundColor(Color::White),
                style::SetBackgroundColor(chunk.color),
                style::Print(text),
            )?;
        }
    }

    let cursor_column = if app.get_current_line().is_virtual { app.cursor_column + 2 } else { app.cursor_column };
    queue!(stdout, helper::move_to(cursor_column, app.cursor_row))?;

    stdout.flush()
}

pub fn render_status(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    // clear
    queue!(
        stdout,
        helper::move_to(0, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print("                                                                                 "),
    )?;

    // mode
    let mode_color = match app.mode {
        // Mode::Color => Color::Yellow,
        // Mode::Name => Color::Red,
        Mode::Visual => Color::Yellow,
        _ => Color::Reset,
    };

    queue!(
        stdout,
        helper::move_to(0, app.window_height - 1),
        style::SetBackgroundColor(mode_color),
        style::Print("     "),
    )?;

    // label
    let col = app.cursor_column;
    let label = app.get_current_line().tags.iter()
        .find(|x| x.start <= col && col < x.end)
        .map(|x| x.label);

    if let Some(n) = label {
        queue!(
            stdout,
            helper::move_to(8, app.window_height - 1),
            style::SetBackgroundColor(app.labels[n].color),
            style::Print("      "),
        )?;

        queue!(
            stdout,
            helper::move_to(16, app.window_height - 1),
            style::SetBackgroundColor(Color::Reset),
            style::Print(&app.labels[n].name),
        )?;
    }

    // status
    let status = format!(
        "{}% {}:{}",
        (app.cursor_row + app.offset_row) / app.nlines,
        app.cursor_row + app.offset_row,
        app.cursor_column,
    );

    queue!(
        stdout,
        helper::move_to(70, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print(status),
    )?;

    queue!(stdout, helper::move_to(app.cursor_column, app.cursor_row))?;
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
