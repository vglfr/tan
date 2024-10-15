use std::io::{Stdout, Write};

use crossterm::{
    cursor,
    execute,
    queue,
    style::{self, Color},
};
use rand::seq::SliceRandom;

use crate::{app::{self, App, Label}, color, helper, render};

struct Chunk {
    text: String,
    color: Color,
}

pub fn handle_m(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_normal_mode();
    execute!(stdout, cursor::Show)?;
    render::render_offset(app, stdout)
}

pub fn handle_a(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    if app.labels.len() < 24 {
        let label = Label {
            name: "new_label".to_owned(),
            color: *app::COLORS.choose(&mut app.rng).unwrap(),
            is_active: false,
            is_visible: true,
        };

        app.labels.push(label);
        render_modal(app, stdout)
    } else {
        Ok(())
    }
}

pub fn handle_d(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    // handle normal
    if app.labels.len() > 1 {
        app.labels.remove(app.modal_row);
        app.modal_row = (app.modal_row - 1).rem_euclid(app.labels.len());
        render_modal(app, stdout)
    } else {
        Ok(())
    }
}

pub fn handle_j(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.modal_row = (app.modal_row + 1) % app.labels.len();
    render_modal(app, stdout)
}

pub fn handle_k(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.modal_row = (app.modal_row - 1).rem_euclid(app.labels.len());
    render_modal(app, stdout)
}

pub fn handle_i(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_name_mode();

    let col = app.modal_start_column + app.labels[app.modal_row].name.len() + 17;
    let row = app.modal_start_row + app.modal_row + 1;

    execute!(stdout, helper::move_to(col, row))?;
    execute!(stdout, cursor::Show)
}

pub fn handle_c(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.set_color_mode();
    app.color_column = app::COLORS.iter().position(|x| x == &app.labels[app.modal_row].color).unwrap();
    color::render_color(app, stdout)
}

pub fn handle_h(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.labels[app.modal_row].is_visible ^= true;

    render::render_offset(app, stdout)?;
    render_modal(app, stdout)
}

#[allow(non_snake_case)]
pub fn handle_0a(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.labels[app.modal_active].is_active = false;
    app.modal_active = app.modal_row;

    app.labels[app.modal_active].is_active = true;
    render_modal(app, stdout)
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

fn chunk_lines(app: &mut App) -> Vec<Vec<Chunk>> {
    let width = app.labels.iter().fold(0, |acc, x| std::cmp::max(acc, x.name.len()));
    let mut lines = app.labels.iter().map(|x| chunk_label(x, width)).collect::<Vec<Vec<Chunk>>>();

    lines.insert(0, vec![Chunk { text: format!("{:width$}", "", width=width+20), color: Color::Black }]);
    lines.push(vec![Chunk { text: format!("{:width$}", "", width=width+20), color: Color::Black }]);

    app.modal_start_column = (app.window_width - lines[0][0].text.len()) / 2;
    app.modal_start_row = (app.window_height - lines.len()) / 2;

    lines
}

fn chunk_label(label: &Label, width: usize) -> Vec<Chunk> {
    let mut chunks = Vec::new();

    chunks.push(Chunk { text: "  ".to_owned(), color: Color::Black });
    chunks.push(Chunk { text: if label.is_active { "A".to_owned() } else { " ".to_owned() }, color: Color::Black });
    chunks.push(Chunk { text: if label.is_visible { " ".to_owned() } else { "H".to_owned() }, color: Color::Black });
    chunks.push(Chunk { text: "  ".to_owned(), color: Color::Black });
    chunks.push(Chunk { text: "        ".to_owned(), color: label.color });
    chunks.push(Chunk { text: "    ".to_owned(), color: Color::Black });
    chunks.push(Chunk { text: format!("{:width$}", label.name, width=width), color: Color::Black });
    chunks.push(Chunk { text: "  ".to_owned(), color: Color::Black });

    chunks
}
