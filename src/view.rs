use std::io::{Stdout, Write};

use crossterm::{
    cursor,
    queue,
    style::{self, Color},
};

use crate::helper::{App, Chunk, Line};

pub fn handle_u(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.untag();
    render_view(&app, stdout)?;
    Ok(())
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

    for line in &app.lines {
        for chunk in chunk_line(line, app) {
            let text = &line.text[chunk.start.into()..chunk.end.into()];

            queue!(
                stdout,
                cursor::MoveTo(chunk.start, line.row),
                style::SetBackgroundColor(chunk.color),
                style::Print(text),
            )?;
        }
    }

    queue!(stdout, cursor::RestorePosition)?;
    stdout.flush()?;

    Ok(())
}

fn chunk_line(line: &Line, app: &App) -> Vec<Chunk> {
    let mut points = vec![0, line.width];

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

    let chunks = points[1..].iter().zip(points.clone()).map(|(e,s)| {
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

    // add whitespace chunk string to overwrite color change after modal hide

    // if line.row == 1 {
    //     let mut f = File::create("/tmp/dbg.json").unwrap();

    //     let s = serde_json::to_string(&points).unwrap();
    //     f.write_all(s.as_bytes()).unwrap();

    //     let s2 = serde_json::to_string(&chunks).unwrap();
    //     f.write_all(s2.as_bytes()).unwrap();

    //     f.write_all(serde_json::to_string(&line).unwrap().as_bytes()).unwrap();
    // }

    chunks
}
