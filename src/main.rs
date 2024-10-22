pub mod app;
pub mod color;
pub mod command;
pub mod common;
pub mod helper;
pub mod io;
pub mod modal;
pub mod name;
pub mod normal;
pub mod render;
pub mod visual;

use std::io::Stdout;

use clap::Parser;
use crossterm::{event::{read, Event, KeyCode, KeyModifiers}, queue, terminal};

use app::{App, Change, FType, Mode};

#[derive(Debug, Parser)]
#[command(version)]
struct Argv {
    #[clap(default_value = "data/test3.json")]
    name: String,
    #[clap(short, long, value_enum, default_value_t = FType::Spacy)]
    format: FType,
}

fn main() -> std::io::Result<()> {
    let argv = Argv::parse();
    let mut app = io::load_file(&argv)?;

    let mut stdout = std::io::stdout();
    render_initial(&mut app, &mut stdout)?;

    loop {
        let keycode = extract_keycode()?;

        match app.mode {
            Mode::Color =>
                match keycode {
                    'h' => color::handle_h(&mut app, &mut stdout)?,
                    'l' => color::handle_l(&mut app, &mut stdout)?,

                    '\x0a' => color::handle_0a(&mut app, &mut stdout)?,
                    '\x1b' => common::handle_1b(&mut app, &mut stdout)?,

                    _ => (),
                },
            Mode::Command =>
                match keycode {
                    c@'!'..='~' => app.command_char(c),
                    '\x08' => app.command_backspace(),

                    '\x0a' => command::handle_0a(&mut app, &mut stdout)?,
                    '\x1b' => app.command_esc(),

                    // 'c-u' => (),
                    // 'c-w' => (),

                    _ => (),
                },
            Mode::Modal =>
                match keycode {
                    // ':' => app.common_colon(),
                    'm' => modal::handle_m(&mut app, &mut stdout)?,

                    'a' => modal::handle_a(&mut app, &mut stdout)?,
                    'd' => modal::handle_d(&mut app, &mut stdout)?,

                    'j' => modal::handle_j(&mut app, &mut stdout)?,
                    'k' => modal::handle_k(&mut app, &mut stdout)?,

                    'i' => modal::handle_i(&mut app, &mut stdout)?,
                    'c' => modal::handle_c(&mut app, &mut stdout)?,

                    'h' => modal::handle_h(&mut app, &mut stdout)?,
                    // isolate tag (hide all tags except this one) -- toggle -- H
                    '\x0a' => modal::handle_0a(&mut app, &mut stdout)?,
                    _ => (),
                },
            Mode::Name =>
                match keycode {
                    c@'!'..='~' => name::handle_key(c, &mut app, &mut stdout)?,
                    '\x08' => name::handle_08(&mut app, &mut stdout)?,

                    '\x0a' => common::handle_1b(&mut app, &mut stdout)?,
                    '\x1b' => common::handle_1b(&mut app, &mut stdout)?,

                    // 'c-u' => (),
                    // 'c-w' => (),

                    _ => (),
                },
            Mode::Normal =>
                match keycode {
                    ':' => app.common_colon(),
                    'm' => common::handle_m(&mut app, &mut stdout)?,
                    'v' => app.normal_v(),

                    'h' => app.normal_h(),
                    'j' => app.normal_j(),
                    'k' => app.normal_k(),
                    'l' => app.normal_l(),

                    'H' => common::handle_H(&mut app),
                    'M' => common::handle_M(&mut app),
                    'L' => common::handle_L(&mut app),

                    '\x11' => common::handle_pg_down(&mut app),
                    '\x12' => common::handle_pg_up(&mut app),

                    's' => common::handle_s(&mut app),
                    'e' => common::handle_e(&mut app),

                    'S' => common::handle_S(&mut app),
                    'E' => common::handle_E(&mut app),

                    'w' => app.normal_w(),
                    'b' => app.normal_b(),

                    't' => common::handle_t(&mut app),
                    'u' => app.normal_u(),
                    _ => (),
                },
            Mode::Visual =>
                match keycode {
                    ':' => app.common_colon(),
                    'm' => common::handle_m(&mut app, &mut stdout)?,
                    'v' => app.visual_v(),

                    'h' => app.visual_h(),
                    'j' => app.visual_j(),
                    'k' => app.visual_k(),
                    'l' => app.visual_l(),

                    'w' => app.visual_w(),
                    'b' => app.visual_b(),

                    't' => common::handle_t(&mut app),
                    _ => (),
                },
        }

        render_event(&mut app, &mut stdout)?;
    }
}

fn render_initial(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    terminal::enable_raw_mode()?;

    queue!(stdout, terminal::EnterAlternateScreen)?;
    queue!(stdout, helper::move_to(app.cursor_column, app.cursor_row))?;

    render::render_offset(app, stdout)?;
    render::render_status(app, stdout)
}

fn render_event(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    let flags = app.get_change_flags();
    app.change = 0;

    for flag in flags {
        match flag {
            Change::Cursor => render::render_cursor(app, stdout)?,
            Change::Offset => render::render_offset(app, stdout)?,
            Change::Status => render::render_status(app, stdout)?,
        }
    }

    // on error
    // - write debug log
    // - restore screen
    // - write error message pointing to debug log

    Ok(())
}

fn extract_keycode() -> std::io::Result<char> {
    match read()? {
        Event::Key(event) => match event.code {
            KeyCode::Char(c) => match c {
                'h' if event.modifiers == KeyModifiers::CONTROL => Ok('\x08'),
                'n' if event.modifiers == KeyModifiers::CONTROL => Ok('\x11'),
                'p' if event.modifiers == KeyModifiers::CONTROL => Ok('\x12'),
                c => Ok(c),
            },
            KeyCode::Backspace => Ok('\x08'),
            KeyCode::Enter => Ok('\x0a'),
            KeyCode::Esc => Ok('\x1b'),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "snap1!")),
        }
        _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "snap2!")),
    }
}
