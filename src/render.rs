use std::io::{Stdout, Write};

use anyhow::Result;
use crossterm::{cursor, queue, style::{self, Color}, terminal::{self, ClearType}};
use serde::{Deserialize, Serialize};

use crate::{app::{self, App, Label, Line}, helper};

#[derive(Debug, PartialEq)]
enum Change {
    Command,
    Cursor,
    Offset,
    Status,
    Modal,
}

#[derive(Clone)]
struct ModalChunk {
    text: String,
    color: Color,
    is_name: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct OffsetChunk {
    start: usize,
    end: usize,
    color: Color,
}

pub fn render_initial(app: &mut App, stdout: &mut Stdout) -> Result<()> {
    terminal::enable_raw_mode()?;

    queue!(stdout, terminal::EnterAlternateScreen)?;
    queue!(stdout, helper::move_to(app.cursor_column, app.cursor_row))?;

    render_offset(app, stdout)?;
    render_status(app, stdout)?;

    stdout.flush().map_err(anyhow::Error::from)
}

pub fn render_terminal(stdout: &mut Stdout) {
    queue!(stdout, terminal::LeaveAlternateScreen).expect("Error leaving alternate screen");
    queue!(stdout, cursor::Show).expect("Error showing cursor");

    terminal::disable_raw_mode().expect("Error disabling raw mode");
    stdout.flush().expect("Error flushing stdout")
}

pub fn render_event(app: &mut App, stdout: &mut Stdout) -> Result<()> {
    let flags = get_change_flags(app);
    app.change = 0;

    for flag in &flags {
        match flag {
            Change::Cursor => render_cursor(app, stdout)?,
            Change::Modal => render_modal(app, stdout)?,
            Change::Offset => render_offset(app, stdout)?,
            Change::Command => render_command(app, stdout)?,
            Change::Status => render_status(app, stdout)?,
        }
    }

    stdout.flush().map_err(anyhow::Error::from)
}

/*
  u8 0000 0000
             ^ app.status
            ^ app.offset_row
           ^ app.cursor_column
          ^ app.cursor_row

        ^ app.modal
       ^ app.command
*/
fn get_change_flags(app: &mut App) -> Vec<Change> {
    let mut flags = Vec::new();

    if app.change & 0b_0000_0010 > 0 { flags.push(Change::Offset); }
    if app.change & 0b_0001_0000 > 0 { flags.push(Change::Modal); }
    if app.change & 0b_0000_0001 > 0 { flags.push(Change::Status); }
    if app.change & 0b_0010_0000 > 0 { flags.push(Change::Command); }
    if app.change & 0b_0000_1100 > 0 { flags.push(Change::Cursor); }

    flags
}

fn render_cursor(app: &App, stdout: &mut Stdout) -> Result<()> {
    if app.is_name_mode() {
        queue!(stdout, helper::move_to(app.modal_column, app.modal_start_row + app.modal_row + 1)).map_err(anyhow::Error::from)
    } else {
        let cursor_column = if app.get_current_line().is_virtual { app.cursor_column + 2 } else { app.cursor_column };
        queue!(stdout, helper::move_to(cursor_column, app.cursor_row)).map_err(anyhow::Error::from)
    }
}

fn render_modal(app: &mut App, stdout: &mut Stdout) -> Result<()> {
    let lines = chunk_modal_lines(app);

    // queue!(stdout, cursor::SavePosition)?;
    queue!(stdout, helper::move_to(app.modal_start_column, app.modal_start_row))?;

    for (i, line) in lines.iter().enumerate() {
        for chunk in line {
            queue!(
                stdout,
                style::SetBackgroundColor(chunk.color),
                // style::SetForegroundColor(if i == app.modal_row + 1 { Color::Yellow } else { Color::White }),
                style::SetForegroundColor(if i == app.modal_row + 1 && chunk.is_name { Color::Yellow } else { Color::White }),
                style::Print(&chunk.text),
            )?;
        }

        queue!(stdout, cursor::MoveDown(1))?;
        queue!(stdout, helper::move_to_column(app.modal_start_column))?;
    }

    Ok(())

    // queue!(stdout, cursor::RestorePosition)
}

fn render_offset(app: &App, stdout: &mut Stdout) -> Result<()> {
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
    queue!(stdout, helper::move_to(cursor_column, app.cursor_row)).map_err(anyhow::Error::from)
}

fn clear_status(app: &mut App, stdout: &mut Stdout) -> Result<()> {
    queue!(
        stdout,
        helper::move_to(0, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print(format!("{:width$}", "", width=app.window_width - 1)),
    ).map_err(anyhow::Error::from)
}

fn render_status(app: &mut App, stdout: &mut Stdout) -> Result<()> {
    clear_status(app, stdout)?;

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
        (app.cursor_row + app.offset_row) * 100 / app.nlines,
        app.cursor_row + app.offset_row,
        app.cursor_column,
    );

    queue!(
        stdout,
        helper::move_to(app.window_width - status.len() - 1, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print(status),
    )?;

    queue!(stdout, helper::move_to(app.cursor_column + if app.get_current_line().is_virtual { 2 } else { 0 }, app.cursor_row)).map_err(anyhow::Error::from)
}

fn render_command(app: &mut App, stdout: &mut Stdout) -> Result<()> {
    clear_status(app, stdout)?;

    queue!(
        stdout,
        helper::move_to(0, app.window_height - 1),
        style::SetBackgroundColor(Color::Reset),
        style::Print(format!(":{}", app.command)),
    ).map_err(anyhow::Error::from)
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

fn chunk_modal_lines(app: &mut App) -> Vec<Vec<ModalChunk>> {
    let mut lines = app.labels.iter().map(chunk_label).collect::<Vec<Vec<ModalChunk>>>();

    let blank = vec![ModalChunk { text: format!(" │{:width$}│ ", "", width=38), color: Color::Reset, is_name: false }];
    let top = vec![ModalChunk { text: format!(" ┌{:─>width$}┐ ", "", width=38), color: Color::Reset, is_name: false }];
    let bottom = vec![ModalChunk { text: format!(" └{:─>width$}┘ ", "", width=38), color: Color::Reset, is_name: false }];

    let color = app.labels[app.modal_row].color;
    let mut colors: Vec<ModalChunk> = app::COLORS.iter()
        .map(|x| ModalChunk { text: if x == &color { "◄►".to_owned() } else { "  ".to_owned() }, color: *x, is_name: false })
        .collect();
    colors.insert(0, ModalChunk { text: " │ ".to_owned(), color: Color::Reset, is_name: false });
    colors.push(ModalChunk { text: " │ ".to_owned(), color: Color::Reset, is_name: false });

    lines.insert(0, top);
    lines.extend(std::iter::repeat(blank).take(25usize.saturating_sub(lines.len())));
    lines.push(colors);
    lines.push(bottom);

    lines
}

fn chunk_label(label: &Label) -> Vec<ModalChunk> {
    let mut chunks = Vec::new();

    chunks.push(ModalChunk { text: " │ ".to_owned(), color: Color::Reset, is_name: false });
    chunks.push(ModalChunk { text: if label.is_active { "A".to_owned() } else { " ".to_owned() }, color: Color::Reset, is_name: false });
    chunks.push(ModalChunk { text: if label.is_visible { " ".to_owned() } else { "H".to_owned() }, color: Color::Reset, is_name: false });
    chunks.push(ModalChunk { text: "  ".to_owned(), color: Color::Reset, is_name: false });
    chunks.push(ModalChunk { text: "        ".to_owned(), color: label.color, is_name: false });
    chunks.push(ModalChunk { text: "    ".to_owned(), color: Color::Reset, is_name: false });
    chunks.push(ModalChunk { text: format!("{:width$}", label.name, width=20), color: Color::Reset, is_name: true });
    chunks.push(ModalChunk { text: " │ ".to_owned(), color: Color::Reset, is_name: false });

    chunks
}
