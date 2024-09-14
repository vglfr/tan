use std::fs::File;
use std::io::{BufRead, BufReader, Stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    queue,
    style::{self, Color},
    terminal,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct App {
    fname: String,
    qname: String,
    cursor_column: u16,
    cursor_row: u16,
    height: u16,
    labels: Vec<Label>,
    lines: Vec<Line>,
    visual: bool,
    visual_row: u16,
    visual_start: u16,
    visual_end: u16,
}

#[derive(Debug, Deserialize, Serialize)]
struct Chunk {
    start: u16,
    end: u16,
    color: Color,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Label {
    name: String,
    color: Color,
}

#[derive(Debug, Deserialize, Serialize)]
struct Line {
    row: u16,
    text: String,
    width: u16,
    tags: Vec<Tag>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Tag {
    start: u16,
    end: u16,
    label: Label,
}

fn main() -> std::io::Result<()> {
    let fname = std::env::args().nth(1).unwrap_or("flake.nix".to_string());
    let qname = format!("data/{fname}.tan");

    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;

    let mut app = load_file(&fname, &qname)?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    execute!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
    render_screen(&app, &mut stdout)?;

    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => match c {
                    'q' => break,
                    'w' => save_tan(&app)?,
                    // KeyCode::Char('x') => { break; }

                    'h' => {
                        if app.cursor_column > 0 {
                            app.cursor_column = app.cursor_column.saturating_sub(1);
                        } else if app.cursor_row > 0 {
                            app.cursor_column = app.lines[app.cursor_row.saturating_sub(1) as usize].width.saturating_sub(1);
                            app.cursor_row = app.cursor_row.saturating_sub(1);
                        }
                    },
                    'j' => {
                        if app.cursor_row < app.height.saturating_sub(1) {
                            app.cursor_row += 1;

                            if app.cursor_column >= app.lines[app.cursor_row as usize].width {
                                app.cursor_column = app.lines[app.cursor_row as usize].width.saturating_sub(1);
                            }
                        }
                    },
                    'k' => {
                        if app.cursor_row < app.height {
                            app.cursor_row = app.cursor_row.saturating_sub(1);

                            if app.cursor_column >= app.lines[app.cursor_row as usize].width {
                                app.cursor_column = app.lines[app.cursor_row as usize].width.saturating_sub(1);
                            }
                        }
                    },
                    'l' => {
                        if app.cursor_column < app.lines[app.cursor_row as usize].width.saturating_sub(1) {
                            app.cursor_column += 1;
                        } else if app.cursor_row < app.height.saturating_sub(1) {
                            app.cursor_column = 0;
                            app.cursor_row += 1;
                        }
                    },

                    // wb{} movement (later WB + g-seSE)
                    // KeyCode::Char('w') => { break; }
                    // KeyCode::Char('b') => { break; }
                    // KeyCode::Char('{') => { break; }
                    // KeyCode::Char('}') => { break; }

                    // toggle selection
                    'v' => {
                        if app.visual {
                            app.visual = false;
                            app.visual_end = app.cursor_column;
                        } else {
                            app.visual = true;
                            app.visual_start = app.cursor_column;
                            app.visual_row = app.cursor_row;
                        }
                    }

                    't' => {
                        tag(&mut app);
                        app.visual = false;
                    },
                    'u' => { untag(&mut app); },

                    _ => (),
                    },
                _ => (),
            },
            _ => (),
        }

        execute!(stdout, cursor::MoveTo(app.cursor_column, app.cursor_row))?;
        if app.visual {
            app.visual_end = app.cursor_column;
            render_screen(&app, &mut stdout)?;
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn tag(app: &mut App) {
    let mut a = [app.visual_start, app.visual_end];
    a.sort();

    let s = a[0];
    let e = std::cmp::min(a[1] + 1, app.lines[app.visual_row as usize].width);

    app.lines[app.visual_row as usize].tags.push(Tag { start: s, end: e, label: Label { name: "tag1".to_owned(), color: Color::Red }});
}

fn untag(app: &mut App) {
    let tags = app.lines[app.cursor_row as usize].tags.clone();
    app.lines[app.cursor_row as usize].tags = tags.into_iter().filter(|x| !(x.start <= app.cursor_column && app.cursor_column <= x.end)).collect::<_>();
}

fn render_screen(app: &App, stdout: &mut Stdout) -> std::io::Result<()> {
    execute!(stdout, cursor::SavePosition)?;

    for line in &app.lines {
        for chunk in chunk_line(line, app) {
            queue!(
                stdout,
                cursor::MoveTo(chunk.start, line.row),
                style::SetBackgroundColor(chunk.color),
                style::Print(&line.text[chunk.start.into()..chunk.end.into()]),
            )?;
        }
    }

    execute!(stdout, cursor::RestorePosition)?;
    stdout.flush()?;

    Ok(())
}

fn chunk_line(line: &Line, app: &App) -> Vec<Chunk> {
    let mut tags = Vec::new();

    if app.visual && line.row == app.visual_row {
        let mut a = [app.visual_start, app.visual_end];
        a.sort();

        let s = a[0];
        let e = std::cmp::min(a[1] + 1, line.width);

        tags.push(Chunk { start: 0, end: s, color: Color::Reset });
        tags.push(Chunk { start: s, end: e, color: Color::Yellow });
        tags.push(Chunk { start: e, end: line.width, color: Color::Reset });

        // let s = serde_json::to_string(&tags).unwrap();
        // let mut f = File::create("/tmp/dbg.json").unwrap();
        // f.write_all(s.as_bytes()).unwrap();
        // f.write_all(serde_json::to_string(&line).unwrap().as_bytes()).unwrap();

    } else {
        tags.push(Chunk { start: 0, end: line.width, color: Color::Reset });
    }

    tags
}

fn load_file(fname: &str, qname: &str) -> std::io::Result<App> {
    if std::path::Path::new(&qname).exists() {
        load_tan(qname)
    } else {
        load_src(fname, qname)
    }
}

fn load_tan(qname: &str) -> std::io::Result<App> {
    let s = std::fs::read_to_string(qname)?;
    serde_json::from_str(&s).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
}

fn load_src(fname: &str, qname: &str) -> std::io::Result<App> {
    let f = File::open(fname).unwrap();
    let mut b = BufReader::new(f);

    let mut lines = Vec::new();
    let mut s = String::new();
    let mut n = 0;

    while b.read_line(&mut s).unwrap_or(0) > 0 {
        let l = Line {
            row: n,
            text: s.strip_suffix("\n").map(|x| x.to_owned()).unwrap_or(s.clone()),
            width: s.len() as u16 - 1,
            tags: Vec::new(),
        };
        lines.push(l);

        n += 1;
        s.truncate(0);
    }

    let app = App {
        fname: fname.to_owned(),
        qname: qname.to_owned(),
        cursor_column: 0,
        cursor_row: 0,
        height: lines.len() as u16,
        labels: Vec::new(),
        lines,
        visual: false,
        visual_row: 0,
        visual_start: 0,
        visual_end: 0,
    };

    Ok(app)

    // prevent from opening non-utf8
}

fn save_tan(app: &App) -> std::io::Result<()> {
    let s = serde_json::to_string(app)?;
    let mut f = File::create(&app.qname)?;

    f.write_all(s.as_bytes())?;
    Ok(())
}
