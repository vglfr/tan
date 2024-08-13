use std::io::{Result,stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};

fn main() -> Result<()> {
    let s = load_content();

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(
                Paragraph::new(s.as_str()).white(),
                area,
            );

            // frame.set_cursor_position()
        })?;

        terminal.show_cursor()?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => { break; }
                        KeyCode::Char('h') => { break; }
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
    let f = std::env::args().nth(1).unwrap_or("flake.nix".to_string());
    std::fs::read_to_string(f).unwrap()
}
