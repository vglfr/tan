pub mod app;
pub mod command;
pub mod common;
pub mod helper;
pub mod io;
pub mod modal;
pub mod name;
pub mod normal;
pub mod render;
pub mod visual;

use anyhow::Result;
use clap::Parser;
use crossterm::event::{read, Event, KeyCode, KeyModifiers};

use app::{FType, Mode};

#[derive(Debug, Parser)]
#[command(version)]
struct Argv {
    #[clap(default_value = "data/test3..json")]
    name: String,
    #[clap(short, long, value_enum)]
    format: Option<FType>,
}

fn main() -> Result<()> {
    let argv = Argv::parse();
    let mut app = io::load_file(&argv)?;

    let mut stdout = std::io::stdout();
    render::render_initial(&mut app, &mut stdout)?;

    loop {
        let keycode = extract_keycode()?;

        match app.mode {
            Mode::Command =>
                match keycode {
                    c@'!'..='~' => app.command_char(c),
                    '\x08' => app.command_backspace(),

                    '\x0a' => app.command_return(&mut stdout)?,
                    '\x1b' => app.command_esc(),

                    _ => (),
                },
            Mode::Modal =>
                match keycode {
                    'm' => app.modal_m(),
                    'i' => app.modal_i(),

                    'h' => app.modal_h(),
                    'j' => app.modal_j(),
                    'k' => app.modal_k(),
                    'l' => app.modal_l(),

                    'v' => app.modal_v(),
                    'V' => app.modal_V(),

                    'a' => app.modal_a(),
                    'd' => app.modal_d(),

                    '\x0a' => app.modal_return(),
                    _ => (),
                },
            Mode::Name =>
                match keycode {
                    c@'!'..='~' => app.name_char(c),
                    '\x08' => app.name_backspace(),

                    '\x0a' => common::handle_esc(&mut app),
                    '\x1b' => common::handle_esc(&mut app),

                    _ => (),
                },
            Mode::Normal =>
                match keycode {
                    ':' => app.common_colon(),
                    'm' => app.normal_m(),
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

                    's' => app.normal_s(),
                    'e' => app.normal_e(),

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
                    'm' => app.visual_m(),
                    'v' => app.visual_v(),

                    'h' => app.visual_h(),
                    'j' => app.visual_j(),
                    'k' => app.visual_k(),
                    'l' => app.visual_l(),

                    's' => app.visual_s(),
                    'e' => app.visual_e(),

                    'w' => app.visual_w(),
                    'b' => app.visual_b(),

                    't' => common::handle_t(&mut app),
                    _ => (),
                },
        }

        render::render_event(&mut app, &mut stdout)?;
    }
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
