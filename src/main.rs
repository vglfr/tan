pub mod color;
pub mod common;
pub mod helper;
pub mod io;
pub mod modal;
pub mod view;
pub mod visual;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal,
};

use crate::helper::Mode;

fn main() -> std::io::Result<()> {
    let fname = std::env::args().nth(1).unwrap_or("flake.nix".to_string());
    let qname = format!("data/{fname}.tan");

    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;

    let mut app = io::load_file(&fname, &qname)?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    execute!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
    view::render_view(&app, &mut stdout)?;

    loop {
        let (keycode,is_ctrl) = extract_keycode()?;

        match keycode {
            'q' => break,
            'w' => io::save_tan(&app)?,
            _ => (),
        }

        match app.mode {
            Mode::Color =>
                match keycode {
                    'h' => color::handle_h(&mut app, &mut stdout)?,
                    'l' => color::handle_l(&mut app, &mut stdout)?,

                    'j' if is_ctrl => color::handle_c_j(&mut app, &mut stdout)?,
                    // 'c-[' => (),
                    _ => (),
                },
            Mode::Modal =>
                match keycode {
                    'm' => modal::handle_m(&mut app, &mut stdout)?,

                    'a' => (),
                    'd' => (),

                    'j' => modal::handle_j(&mut app, &mut stdout)?,
                    'k' => modal::handle_k(&mut app, &mut stdout)?,

                    'n' => (),
                    'c' => modal::handle_c(&mut app, &mut stdout)?,
                    _ => (),
                },
            Mode::Name =>
                match keycode {
                    // 'c-j' => (),
                    // 'c-h' => (),
                    // 'c-u' => (),
                    _ => (), // input
                },
            Mode::View =>
                match keycode {
                    'h' => common::handle_h(&mut app, &mut stdout)?,
                    'j' => common::handle_j(&mut app, &mut stdout)?,
                    'k' => common::handle_k(&mut app, &mut stdout)?,
                    'l' => common::handle_l(&mut app, &mut stdout)?,

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

fn extract_keycode() -> std::io::Result<(char,bool)> {
    match read()? {
        Event::Key(event) => match event.code {
            KeyCode::Char(c) => Ok((c,false)),
            KeyCode::Enter => Ok(('j',true)),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "snap1!")),
        }
        _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "snap2!")),
    }
}
