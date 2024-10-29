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
use tap::TapFallible;

use app::{FType, Mode};

#[derive(Debug, Parser)]
#[command(version)]
struct Argv {
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
        let keycode = extract_keycode().tap_err(|_| render::render_terminal(&mut stdout))?;

        match app.mode {
            Mode::Command => match keycode {
                c @ '!'..='~' => app.command_char(c),
                '\x08' => app.command_backspace(),

                '\x0a' => app
                    .command_return(&mut stdout)
                    .tap_err(|_| render::render_terminal(&mut stdout))?,
                '\x1b' => app.command_esc(),

                _ => (),
            },
            Mode::Modal => match keycode {
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
            Mode::Name => match keycode {
                c @ '!'..='~' => app.name_char(c),
                '\x08' => app.name_backspace(),

                '\x0a' => app.name_esc(),
                '\x1b' => app.name_esc(),

                _ => (),
            },
            Mode::Normal => match keycode {
                ':' => app.common_colon(),
                'm' => app.normal_m(),
                'v' => app.normal_v(),

                'h' => app.normal_h(),
                'j' => app.normal_j(),
                'k' => app.normal_k(),
                'l' => app.normal_l(),

                'H' => app.normal_H(),
                'M' => app.normal_M(),
                'L' => app.normal_L(),

                '\x11' => app.normal_pg_down(),
                '\x12' => app.normal_pg_up(),

                's' => app.normal_s(),
                'e' => app.normal_e(),

                'S' => app.normal_S(),
                'E' => app.normal_E(),

                'w' => app.normal_w(),
                'b' => app.normal_b(),

                't' => app.common_t(),
                'u' => app.normal_u(),

                _ => (),
            },
            Mode::Visual => match keycode {
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

                't' => app.common_t(),
                _ => (),
            },
        }

        render::render_event(&mut app, &mut stdout).tap_err(|_| render::render_terminal(&mut stdout))?;
    }
}

fn extract_keycode() -> Result<char> {
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
            _ => Ok('\x00'),
        },
        Event::Resize(..) => Err(anyhow::Error::msg("Window resize is currently not supported")),
        _ => Ok('\x00'),
    }
}
