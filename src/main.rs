use std::io::{Result,stdout};

use ratatui::{
    backend::CrosstermBackend, crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    }, layout::{Position, Rect}, style::Stylize, widgets::Paragraph, Terminal
};

struct Line {
    n: u16,
    text: String,
    width: u16,
}

struct Tag {
    s: (Line, u16),
    e: (Line, u16),
    label: Label,
}

struct Label {
    name: String,
    color: String,
}

fn main() -> Result<()> {
    let s = load_content();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut area = Rect { x: 0, y: 0, height: 0, width: 0 };
    let mut curpos = Position { x: 0, y: 0 };

    let mut v = false;

    let mut v_start = Position { x: 0, y: 0 };
    let mut v_end = Position { x: 0, y: 0 };

    loop {
        terminal.draw(|frame| {
            area = frame.area();
            frame.render_widget(
                Paragraph::new(s.as_str()).white(),
                area,
            );

            let rect = if v {
                Rect { x: v_start.x, y: v_start.y, height: v_end.y - v_start.y + 1, width: v_end.x - v_start.x + 1 }
            } else {
                Rect { x: 0, y: 0, height: 0, width: 0 }
            };

            frame.render_widget(
                Paragraph::new("").on_green(),
                rect,
            );

            frame.set_cursor_position(curpos);
        })?;

        terminal.show_cursor()?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        // :q :w :x
                        KeyCode::Char('q') => { break; }
                        // KeyCode::Char('s') => { break; }
                        // KeyCode::Char('x') => { break; }

                        // hjkl movements
                        KeyCode::Char('h') => {
                            if curpos.x > 0 {
                                curpos.x -= 1;
                            } else if curpos.y > 0 {
                                curpos.x = area.width - 1;
                                curpos.y -= 1;
                            }
                        }
                        KeyCode::Char('j') => { curpos.y += if curpos.y < area.height { 1 } else { 0 }; }
                        KeyCode::Char('k') => { curpos.y -= if curpos.y > 0 { 1 } else { 0 }; }
                        KeyCode::Char('l') => {
                            if curpos.x < area.width - 1 {
                                curpos.x += 1;
                            } else if curpos.y < area.height {
                                curpos.x = 0;
                                curpos.y += 1;
                            }
                        }

                        // wb{} movement (later WB + g-setb)
                        // KeyCode::Char('w') => { break; }
                        // KeyCode::Char('b') => { break; }
                        // KeyCode::Char('{') => { break; }
                        // KeyCode::Char('}') => { break; }

                        // toggle selection
                        KeyCode::Char('v') => {
                            if v {
                                v = false;
                                v_end = curpos;
                            } else {
                                v = true;
                                v_start = curpos;
                                v_end = curpos;
                            }
                        }

                        // tag-untag
                        // KeyCode::Char('t') => { break; }
                        // KeyCode::Char('u') => { break; }

                        _ => {}
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn load_content() -> String {
    // read to file
    // prevent from opening non-utf8
    let f = std::env::args().nth(1).unwrap_or("flake.nix".to_string());
    std::fs::read_to_string(f).unwrap()
}
