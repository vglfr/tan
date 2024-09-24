pub mod color;
pub mod common;
pub mod helper;
pub mod io;
pub mod modal;
pub mod name;
pub mod view;
pub mod visual;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal,
};

use crate::helper::Mode;

fn main() -> std::io::Result<()> {
    let fname = std::env::args().nth(1).unwrap_or("test.txt".to_string());
    let qname = format!("data/{fname}.tan");

    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;

    let mut app = io::load_file(&fname, &qname)?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    execute!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
    view::render_view(&app, &mut stdout)?;

    loop {
        let keycode = extract_keycode()?;

        match keycode {
            'q' => break,
            'w' => io::save_tan(&app)?,
            'D' => io::dump_debug(&app)?,
            _ => (),
        }

        match app.mode {
            Mode::Color =>
                match keycode {
                    'h' => color::handle_h(&mut app, &mut stdout)?,
                    'l' => color::handle_l(&mut app, &mut stdout)?,

                    '\x0a' => color::handle_0a(&mut app, &mut stdout)?,
                    '\x1b' => common::handle_1b(&mut app, &mut stdout)?,

                    _ => (),
                },
            Mode::Modal =>
                match keycode {
                    'm' => modal::handle_m(&mut app, &mut stdout)?,

                    'a' => modal::handle_a(&mut app, &mut stdout)?,
                    'd' => modal::handle_d(&mut app, &mut stdout)?,

                    'j' => modal::handle_j(&mut app, &mut stdout)?,
                    'k' => modal::handle_k(&mut app, &mut stdout)?,

                    'n' => modal::handle_n(&mut app, &mut stdout)?,
                    'c' => modal::handle_c(&mut app, &mut stdout)?,
                    _ => (),
                },
            Mode::Name =>
                match keycode {
                    c@'!'..='~' => name::handle_key(c, &mut app, &mut stdout)?,

                    '\x1b' => common::handle_1b(&mut app, &mut stdout)?,
                    '\x08' => name::handle_08(&mut app, &mut stdout)?,

                    // 'c-u' => (),
                    _ => (),
                },
            Mode::View =>
                match keycode {
                    'h' => common::handle_h(&mut app, &mut stdout)?,
                    'j' => common::handle_j(&mut app, &mut stdout)?,
                    'k' => common::handle_k(&mut app, &mut stdout)?,
                    'l' => common::handle_l(&mut app, &mut stdout)?,

                    'H' => view::handle_H(&mut app, &mut stdout)?,
                    'M' => view::handle_M(&mut app, &mut stdout)?,
                    'L' => view::handle_L(&mut app, &mut stdout)?,

                    '\x11' => view::handle_pg_down(&mut app, &mut stdout)?,
                    '\x12' => view::handle_pg_up(&mut app, &mut stdout)?,

                    's' => common::handle_s(&mut app, &mut stdout)?,
                    'e' => common::handle_e(&mut app, &mut stdout)?,

                    // wb{} movement (later WB + g-seSE)
                    // 'w' => { break; }
                    // 'b' => { break; }
                    // '{' => { break; }
                    // '}' => { break; }

                    'm' => common::handle_m(&mut app, &mut stdout)?,
                    'v' => view::handle_v(&mut app)?,

                    't' => common::handle_t(&mut app, &mut stdout)?,
                    'u' => view::handle_u(&mut app, &mut stdout)?,
                    _ => (),
                },
            Mode::Visual =>
                match keycode {
                    'h' => common::handle_h(&mut app, &mut stdout)?,
                    'l' => common::handle_l(&mut app, &mut stdout)?,

                    'm' => common::handle_m(&mut app, &mut stdout)?,
                    't' => common::handle_t(&mut app, &mut stdout)?,
                    'v' => visual::handle_v(&mut app)?,
                    _ => (),
                },
        }

    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

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
