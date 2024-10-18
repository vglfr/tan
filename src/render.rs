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
            let text = &line.text[chunk.start..chunk.end];

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

    if app.is_command_mode() {
        queue!(
            stdout,
            helper::move_to(0, app.window_height - 1),
            style::SetBackgroundColor(Color::Reset),
            style::Print(":"),
        )?;

        queue!(
            stdout,
            helper::move_to(1, app.window_height - 1),
            style::SetBackgroundColor(Color::Reset),
            style::Print(&app.command),
        )?;
    } else {
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

        queue!(stdout, helper::move_to(app.cursor_column + if app.get_current_line().is_virtual { 2 } else { 0 }, app.cursor_row))?;
    }

    stdout.flush()
}

fn chunk_line(line: &Line, app: &App) -> Vec<Chunk> {
    let mut points = vec![0, line.width];
    let (visual_start, visual_end) = app.get_visual_bounds();

    if !app.is_empty_visual() && app.visual_row == line.virtual_row {
        points.extend([visual_start, visual_end]);
    }

    let tag_points = line.tags.iter().flat_map(|x| [x.start, x.end]);
    points.extend(tag_points);

    points.sort();
    points.dedup();

    let chunks = points[1..].iter().zip(points.clone())
        .filter(|(e,s)| *e > s)
        .map(|(e,s)| {
            let tags = line.tags.iter()
                .filter(|x| x.start <= s && *e <= x.end)
                .map(|x| x.label)
                .collect::<Vec<usize>>();

            let color =
                if !app.is_empty_visual() && app.visual_row == line.virtual_row && visual_start <= s && *e <= visual_end {
                    Color::AnsiValue(172)
                } else if tags.len() > 1 {
                    Color::AnsiValue(160)
                } else if let [tag] = tags[..] {
                    let label = &app.labels[tag];
                    if label.is_visible { label.color } else { Color::Reset }
                } else {
                    Color::Reset
                };

            Chunk { start: s, end: *e, color }
        })
        .collect();

    // if line.virtual_row == 0 {
    //     let mut f = File::create("/tmp/dbg.line.txt").unwrap();

    //     f.write_all("line = ".as_bytes()).unwrap();
    //     f.write_all(serde_json::to_string(&line).unwrap().as_bytes()).unwrap();
    //     f.write_all("\n".as_bytes()).unwrap();

    //     f.write_all("chunks = ".as_bytes()).unwrap();
    //     f.write_all(serde_json::to_string(&chunks).unwrap().as_bytes()).unwrap();
    //     f.write_all("\n".as_bytes()).unwrap();

    //     f.write_all("points = ".as_bytes()).unwrap();
    //     f.write_all(serde_json::to_string(&points).unwrap().as_bytes()).unwrap();
    //     f.write_all("\n".as_bytes()).unwrap();
    // }

    chunks
}
