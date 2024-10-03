pub mod color;
pub mod command;
pub mod common;
pub mod helper;
pub mod io;
pub mod modal;
pub mod name;
pub mod normal;
pub mod visual;

use std::io::Stdout;

use clap::Parser;
use crossterm::{cursor, event::{read, Event, KeyCode, KeyModifiers}, queue, terminal};

use helper::{App, FType, Mode};

#[derive(Debug, Parser)]
#[command(version)]
struct Argv {
    #[clap(default_value = "data/test.json")]
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
                    c@'!'..='~' => command::handle_key(c, &mut app, &mut stdout)?,
                    '\x08' => command::handle_08(&mut app, &mut stdout)?,

                    '\x0a' => command::handle_0a(&mut app, &mut stdout)?,
                    '\x1b' => command::handle_1b(&mut app, &mut stdout)?,

                    // 'c-u' => (),
                    // 'c-w' => (),

                    _ => (),
                },
            Mode::Modal =>
                match keycode {
                    // ':' => app.set_command_mode(),
                    ':' => common::handle_colon(&mut app, &mut stdout)?,
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
                    ':' => common::handle_colon(&mut app, &mut stdout)?,
                    'm' => common::handle_m(&mut app, &mut stdout)?,
                    'v' => normal::handle_v(&mut app),

                    'h' => common::handle_h(&mut app, &mut stdout)?,
                    'j' => common::handle_j(&mut app, &mut stdout)?,
                    'k' => common::handle_k(&mut app, &mut stdout)?,
                    'l' => common::handle_l(&mut app, &mut stdout)?,

                    'H' => common::handle_H(&mut app, &mut stdout)?,
                    'M' => common::handle_M(&mut app, &mut stdout)?,
                    'L' => common::handle_L(&mut app, &mut stdout)?,

                    '\x11' => common::handle_pg_down(&mut app, &mut stdout)?,
                    '\x12' => common::handle_pg_up(&mut app, &mut stdout)?,

                    's' => common::handle_s(&mut app, &mut stdout)?,
                    'e' => common::handle_e(&mut app, &mut stdout)?,

                    'S' => common::handle_S(&mut app, &mut stdout)?,
                    'E' => common::handle_E(&mut app, &mut stdout)?,

                    'w' => common::handle_w(&mut app, &mut stdout)?,
                    'b' => common::handle_b(&mut app, &mut stdout)?,

                    't' => common::handle_t(&mut app, &mut stdout)?,
                    'u' => normal::handle_u(&mut app, &mut stdout)?,
                    _ => (),
                },
            Mode::Visual =>
                match keycode {
                    ':' => common::handle_colon(&mut app, &mut stdout)?,
                    'm' => common::handle_m(&mut app, &mut stdout)?,
                    'v' => visual::handle_v(&mut app),

                    'h' => common::handle_h(&mut app, &mut stdout)?,
                    'l' => common::handle_l(&mut app, &mut stdout)?,

                    't' => common::handle_t(&mut app, &mut stdout)?,
                    _ => (),
                },
        }

        // render_event(&mut app)
    }
}

fn render_initial(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    terminal::enable_raw_mode()?;

    queue!(stdout, terminal::EnterAlternateScreen)?;
    queue!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;

    normal::render_normal(app, stdout)?;
    common::render_statusline(app, stdout)
}

// u8 00000000
//    ^    app.mode
//     ^   app.offset_row
//      ^  app.cursor_column
//       ^ app.cursor_row
fn render_event(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
//     app.mode | app.is_command() => command(),
//     app.mode => statusline(),
//     app.offset_row => { view(), statusline() },
//     app.cursor_column => { cursor(), statusline() }, 
//     app.cursor_row => { cursor(), statusline() }, 
//     _ => (), 
    // if app.is_command() {
    //     render_command()?;
    // } else {
    //     render_statusline()?;
    // }
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
