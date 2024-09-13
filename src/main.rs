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

#[derive(Debug, Deserialize, Serialize)]
struct Tag {
    start: usize,
    end: usize,
    label: Label,
}

fn main() -> std::io::Result<()> {
    let fname = std::env::args().nth(1).unwrap_or("flake.nix".to_string());
    let qname = format!("data/{fname}.tan");

    let mut v = false;
    let mut v_start = 0;

    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;

    let lines = load_file(&fname, &qname)?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    execute!(stdout, cursor::MoveTo(0, 0))?;
    render_screen(&lines, &mut stdout, v, v_start)?;

    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => match c {
                    'q' => break,
                    'w' => save_tan(&lines, &qname)?,
                    'h' => {
                        execute!(stdout, cursor::MoveLeft(1))?;
                        if v { render_screen(&lines, &mut stdout, v, v_start)?; }
                    },
                    'j' => {
                        execute!(stdout, cursor::MoveDown(1))?;
                    },
                    'k' => {
                        execute!(stdout, cursor::MoveUp(1))?;
                    },
                    'l' => {
                        execute!(stdout, cursor::MoveRight(1))?;
                        if v { render_screen(&lines, &mut stdout, v, v_start)?; }
                    },
//                     // :q :w :x
//                     KeyCode::Char('q') => { break; }
//                     // KeyCode::Char('w') => { break; }
//                     // KeyCode::Char('x') => { break; }

//                     // hjkl movements
//                     KeyCode::Char('h') => {
//                         if curpos.x > 0 {
//                             curpos.x -= 1;
//                         } else if curpos.y > 0 {
//                             curpos.x = area.width - 1;
//                             curpos.y -= 1;
//                         }
//                     }
//                     KeyCode::Char('j') => { curpos.y += if curpos.y < area.height { 1 } else { 0 }; }
//                     KeyCode::Char('k') => { curpos.y -= if curpos.y > 0 { 1 } else { 0 }; }
//                     KeyCode::Char('l') => {
//                         if curpos.x < area.width - 1 {
//                             curpos.x += 1;
//                         } else if curpos.y < area.height {
//                             curpos.x = 0;
//                             curpos.y += 1;
//                         }
//                     }

//                     // wb{} movement (later WB + g-setb)
//                     // KeyCode::Char('w') => { break; }
//                     // KeyCode::Char('b') => { break; }
//                     // KeyCode::Char('{') => { break; }
//                     // KeyCode::Char('}') => { break; }

                    // toggle selection
                    'v' => {
                        if v {
                            v = false;
                            execute!(stdout, cursor::SetCursorStyle::SteadyBlock)?;
                        } else {
                            v = true;
                            v_start = cursor::position().unwrap().0;

                            execute!(stdout, cursor::SetCursorStyle::SteadyUnderScore)?;
                            render_screen(&lines, &mut stdout, v, v_start)?;
                        }
                    }

//                     // tag-untag
//                     // KeyCode::Char('t') => { break; }
//                     // KeyCode::Char('u') => { break; }
                    _ => (),
                    },
                _ => (),
            },
            _ => (),
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn tag() {
}

fn untag() {
}

fn render_screen(lines: &Vec<Line>, stdout: &mut Stdout, v: bool, v_start: u16) -> std::io::Result<()> {
    let (v_end, row) = cursor::position().unwrap();
    execute!(stdout, cursor::SavePosition)?;

    for line in lines {
        let mut tmp: Vec<Tag> = vec![];
        let chunks = if v && line.row == row {
            let mut a = [v_start as usize, v_end as usize];
            a.sort();

            tmp.push(Tag { start: 0, end: a[0], label: Label { name: "0".to_owned(), color: Color::Reset }});
            tmp.push(Tag { start: a[0], end: a[1] + 1, label: Label { name: "0".to_owned(), color: Color::Yellow }});
            tmp.push(Tag { start: a[1] + 1, end: line.width as usize, label: Label { name: "0".to_owned(), color: Color::Reset }});

            // let s = serde_json::to_string(&tmp)?;
            // let mut f = File::create("/tmp/dbg.json")?;
            // f.write_all(s.as_bytes())?;
            // f.write_all(serde_json::to_string(&line)?.as_bytes())?;

            &tmp
        } else { &line.tags };

        for chunk in chunks {
            queue!(
                stdout,
                cursor::MoveTo(chunk.start as u16, line.row),
                style::SetBackgroundColor(chunk.label.color),
                style::Print(&line.text[chunk.start..chunk.end])
            )?;
        }
    }

    execute!(stdout, cursor::RestorePosition)?;
    stdout.flush()?;

    Ok(())
}

fn load_file(fname: &str, qname: &str) -> std::io::Result<Vec<Line>> {
    if std::path::Path::new(&qname).exists() {
        load_tan(qname)
    } else {
        load_src(fname)
    }
}

fn load_tan(fname: &str) -> std::io::Result<Vec<Line>> {
    let s = std::fs::read_to_string(fname)?;
    serde_json::from_str(&s).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
}

fn load_src(fname: &str) -> std::io::Result<Vec<Line>> {
    let f = File::open(fname).unwrap();
    let mut b = BufReader::new(f);

    let mut ls = Vec::new();
    let mut s = String::new();
    let mut n = 0;

    while b.read_line(&mut s).unwrap_or(0) > 0 {
        let mut l = Line {
            row: n,
            text: s.strip_suffix("\n").map(|x| x.to_owned()).unwrap_or(s.clone()),
            width: s.len() as u16 - 1,
            tags: vec![Tag { start: 0, end: s.len() - 1, label: Label { name: "0".to_owned(), color: Color::Reset } }],
        };
        n += 1;

        if n == 2 {
            l.tags = vec![
                Tag { start: 0, end: 2, label: Label { name: "0".to_owned(), color: Color::Reset } },
                Tag { start: 2, end: 8, label: Label { name: "foo".to_owned(), color: Color::Red } },
                Tag { start: 8, end: 12, label: Label { name: "0".to_owned(), color: Color::Reset } },
            ];
        }

        ls.push(l);
        s.truncate(0);
    }

    Ok(ls)

    // prevent from opening non-utf8
}

fn save_tan(lines: &Vec<Line>, qname: &str) -> std::io::Result<()> {
    let s = serde_json::to_string(lines)?;
    let mut f = File::create(qname)?;

    f.write_all(s.as_bytes())?;
    Ok(())
}
