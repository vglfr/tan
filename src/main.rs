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
    color: String,
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
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;

    let lines = load_file()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    print_lines(&lines, &mut stdout)?;
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // let mut v = false;

    // let mut v_start = Position { x: 0, y: 0 };
    // let mut v_end = Position { x: 0, y: 0 };

    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => match c {
                    'q' => break,
                    'w' => save_tan(&lines)?,
                    'h' => {
                        execute!(stdout, cursor::MoveLeft(1))?
                    },
                    'j' => execute!(stdout, cursor::MoveDown(1))?,
                    'k' => execute!(stdout, cursor::MoveUp(1))?,
                    'l' => execute!(stdout, cursor::MoveRight(1))?,
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

//                     // toggle selection
//                     KeyCode::Char('v') => {
//                         if v {
//                             v = false;
//                             v_end = curpos;
//                         } else {
//                             v = true;
//                             v_start = curpos;
//                             v_end = curpos;
//                         }
//                     }

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

fn print_lines(lines: &Vec<Line>, stdout: &mut Stdout) -> std::io::Result<()> {
    for line in lines {
        for tag in &line.tags {
            queue!(
                stdout,
                cursor::MoveTo(tag.start as u16, line.row),
                style::SetBackgroundColor(if tag.label.name == "0" { Color::Reset } else { Color::Red }),
                style::Print(&line.text[tag.start..tag.end])
            )?;
        }
    }

    stdout.flush()?;
    Ok(())
}

fn load_file() -> std::io::Result<Vec<Line>> {
    let fname = std::env::args().nth(1).unwrap_or("flake.nix".to_string());
    if std::path::Path::new("flake.nix.tan").exists() {
        load_tan(fname)
    } else {
        load_src(fname)
    }
}

fn load_tan(fname: String) -> std::io::Result<Vec<Line>> {
    let s = std::fs::read_to_string(fname)?;
    serde_json::from_str(&s).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
}

fn load_src(fname: String) -> std::io::Result<Vec<Line>> {
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
            tags: vec![Tag { start: 0, end: s.len() - 1, label: Label { name: "0".to_owned(), color: "0".to_owned() } }],
        };
        n += 1;

        if n == 2 {
            l.tags = vec![
                Tag { start: 0, end: 2, label: Label { name: "0".to_owned(), color: "0".to_owned() } },
                Tag { start: 2, end: 8, label: Label { name: "foo".to_owned(), color: "bar".to_owned() } },
                Tag { start: 8, end: 12, label: Label { name: "0".to_owned(), color: "0".to_owned() } },
            ];
        }

        ls.push(l);
        s.truncate(0);
    }

    Ok(ls)

    // prevent from opening non-utf8
}

fn save_tan(lines: &Vec<Line>) -> std::io::Result<()> {
    let s = serde_json::to_string(lines)?;
    let mut f = File::create("flake.nix.tan")?;

    f.write_all(s.as_bytes())?;
    Ok(())
}
