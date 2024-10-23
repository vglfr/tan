use std::io::{Stdout, Write};

use crossterm::{cursor, execute, queue, style::{self, Color}, terminal::{self, ClearType}};
use serde::{Deserialize, Serialize};

use crate::{app::{App, Label, Line}, helper};

struct ModalChunk {
    text: String,
    color: Color,
}

#[derive(Debug, Deserialize, Serialize)]
struct OffsetChunk {
    start: usize,
    end: usize,
    color: Color,
}

pub fn render_cursor(app: &App, stdout: &mut Stdout) -> std::io::Result<()> {
    let cursor_column = if app.get_current_line().is_virtual { app.cursor_column + 2 } else { app.cursor_column };
    execute!(stdout, helper::move_to(cursor_column, app.cursor_row))
}

pub fn render_modal(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let lines = chunk_lines(app);

    execute!(stdout, cursor::SavePosition)?;
    queue!(stdout, helper::move_to(app.modal_start_column - 1, app.modal_start_row))?;

    for (i, line) in lines.iter().enumerate() {
        for chunk in line {
            queue!(
                stdout,
                style::SetBackgroundColor(chunk.color),
                style::SetForegroundColor(if i == app.modal_row + 1 { Color::Yellow } else { Color::White }),
                style::Print(&chunk.text),
            )?;
        }

        queue!(stdout, cursor::MoveDown(1))?;
        queue!(stdout, helper::move_to_column(app.modal_start_column - 1))?;
    }

    execute!(stdout, cursor::RestorePosition)?;
    stdout.flush()?;

    Ok(())
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
    queue!(
        stdout,
        helper::move_to(0, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print("                                                                                 "),
    )?;

    if app.is_visual_mode() {
        queue!(
            stdout,
            helper::move_to(0, app.window_height - 1),
            style::SetBackgroundColor(Color::AnsiValue(172)),
            style::Print("      "),
        )?;
    } else {
        let labels = app.get_current_line().tags.iter()
            .filter(|x| app.labels[x.label].is_visible && x.start <= app.cursor_column && app.cursor_column < x.end)
            .map(|x| x.label)
            .collect::<Vec<usize>>();

        if let [n] = labels[..] {
            queue!(
                stdout,
                helper::move_to(0, app.window_height - 1),
                style::SetBackgroundColor(app.labels[n].color),
                style::Print("      "),
            )?;

            queue!(
                stdout,
                helper::move_to(8, app.window_height - 1),
                style::SetBackgroundColor(Color::Reset),
                style::Print(&app.labels[n].name),
            )?;
        } else if labels.len() > 1 {
            queue!(
                stdout,
                helper::move_to(0, app.window_height - 1),
                style::SetBackgroundColor(Color::AnsiValue(160)),
                style::Print("      "),
            )?;
        }
    }

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

    stdout.flush()
}

pub fn render_command(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    queue!(
        stdout,
        helper::move_to(0, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print("                                                                                 "),
    )?;

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

    stdout.flush()
}

fn chunk_line(line: &Line, app: &App) -> Vec<OffsetChunk> {
    let mut points = vec![0, line.width];

    let (visual_start,visual_end) = app.get_visual_bounds(line.virtual_row);
    points.extend([visual_start, visual_end]);

    let tag_points = line.tags.iter().flat_map(|x| [x.start, x.end]);
    points.extend(tag_points);

    points.sort();
    points.dedup();

    let chunks = points[1..].iter().zip(points.clone())
        .filter(|(e,s)| *e > s)
        .map(|(e,s)| {
            let tags = line.tags.iter()
                .filter(|x| app.labels[x.label].is_visible && x.start <= s && *e <= x.end)
                .map(|x| x.label)
                .collect::<Vec<usize>>();

            let color =
                if visual_start <= s && *e <= visual_end {
                    Color::AnsiValue(172)
                } else if tags.len() > 1 {
                    Color::AnsiValue(160)
                } else if let [tag] = tags[..] {
                    app.labels[tag].color
                } else {
                    Color::Reset
                };

            OffsetChunk { start: s, end: *e, color }
        })
        .collect();

    chunks
}

fn chunk_lines(app: &mut App) -> Vec<Vec<ModalChunk>> {
    let width = app.labels.iter().fold(0, |acc, x| std::cmp::max(acc, x.name.len()));
    let mut lines = app.labels.iter().map(|x| chunk_label(x, width)).collect::<Vec<Vec<ModalChunk>>>();

    lines.insert(0, vec![ModalChunk { text: format!("{:width$}", "", width=width+20), color: Color::Black }]);
    lines.push(vec![ModalChunk { text: format!("{:width$}", "", width=width+20), color: Color::Black }]);

    app.modal_start_column = (app.window_width - lines[0][0].text.len()) / 2;
    app.modal_start_row = (app.window_height - lines.len()) / 2;

    lines
}

fn chunk_label(label: &Label, width: usize) -> Vec<ModalChunk> {
    let mut chunks = Vec::new();

    chunks.push(ModalChunk { text: "  ".to_owned(), color: Color::Black });
    chunks.push(ModalChunk { text: if label.is_active { "A".to_owned() } else { " ".to_owned() }, color: Color::Black });
    chunks.push(ModalChunk { text: if label.is_visible { " ".to_owned() } else { "H".to_owned() }, color: Color::Black });
    chunks.push(ModalChunk { text: "  ".to_owned(), color: Color::Black });
    chunks.push(ModalChunk { text: "        ".to_owned(), color: label.color });
    chunks.push(ModalChunk { text: "    ".to_owned(), color: Color::Black });
    chunks.push(ModalChunk { text: format!("{:width$}", label.name, width=width), color: Color::Black });
    chunks.push(ModalChunk { text: "  ".to_owned(), color: Color::Black });

    chunks
}
