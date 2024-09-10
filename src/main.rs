use std::fs::File;
use std::io::{BufRead, BufReader, Stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    queue,
    style,
    terminal,
};

// struct Label {
//     name: String,
//     color: String,
// }

struct Line {
    n: u16,
    text: String,
    width: u16,
}

// struct Tag {
//     s: (Line, u16),
//     e: (Line, u16),
//     label: Label,
// }

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;

    let ls = load_content_lines()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    print_lines(ls, &mut stdout)?;

    loop {
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => match c {
                    'q' => break,
                    'h' => {
                        execute!(stdout, cursor::MoveLeft(1))?
                    },
                    'j' => execute!(stdout, cursor::MoveDown(1))?,
                    'k' => execute!(stdout, cursor::MoveUp(1))?,
                    'l' => execute!(stdout, cursor::MoveRight(1))?,
//                     // :q :w :x
//                     KeyCode::Char('q') => { break; }
//                     // KeyCode::Char('s') => { break; }
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

fn load_content_lines() -> std::io::Result<Vec<Line>> {
    let fname = std::env::args().nth(1).unwrap_or("flake.nix".to_string());
    let f = File::open(fname).unwrap();
    let mut b = BufReader::new(f);

    let mut ls = Vec::new();
    let mut s = String::new();
    let mut n = 0;

    while b.read_line(&mut s).unwrap_or(0) > 0 {
        let l = Line { n: n, text: s.strip_suffix("\n").map(|x| x.to_owned()).unwrap_or(s.clone()), width: s.len() as u16 - 1 };
        n += 1;

        ls.push(l);
        s.truncate(0);
    }

    Ok(ls)

    // prevent from opening non-utf8
}

fn print_lines(ls: Vec<Line>, stdout: &mut Stdout) -> std::io::Result<()> {
    for l in ls {
        queue!(stdout, cursor::MoveTo(0, l.n), style::Print(l.text))?;
    }

    stdout.flush()?;
    Ok(())
}

// fn main() -> std::io::Result<()> {
//     // let mut v = false;

//     // let mut v_start = Position { x: 0, y: 0 };
//     // let mut v_end = Position { x: 0, y: 0 };

//     //     if event::poll(std::time::Duration::from_millis(16))? {
//     //         if let event::Event::Key(key) = event::read()? {
//     //             if key.kind == KeyEventKind::Press {
//     //                 match key.code {
//     //                     // :q :w :x
//     //                     KeyCode::Char('q') => { break; }
//     //                     // KeyCode::Char('s') => { break; }
//     //                     // KeyCode::Char('x') => { break; }

//     //                     // hjkl movements
//     //                     KeyCode::Char('h') => {
//     //                         if curpos.x > 0 {
//     //                             curpos.x -= 1;
//     //                         } else if curpos.y > 0 {
//     //                             curpos.x = area.width - 1;
//     //                             curpos.y -= 1;
//     //                         }
//     //                     }
//     //                     KeyCode::Char('j') => { curpos.y += if curpos.y < area.height { 1 } else { 0 }; }
//     //                     KeyCode::Char('k') => { curpos.y -= if curpos.y > 0 { 1 } else { 0 }; }
//     //                     KeyCode::Char('l') => {
//     //                         if curpos.x < area.width - 1 {
//     //                             curpos.x += 1;
//     //                         } else if curpos.y < area.height {
//     //                             curpos.x = 0;
//     //                             curpos.y += 1;
//     //                         }
//     //                     }

//     //                     // wb{} movement (later WB + g-setb)
//     //                     // KeyCode::Char('w') => { break; }
//     //                     // KeyCode::Char('b') => { break; }
//     //                     // KeyCode::Char('{') => { break; }
//     //                     // KeyCode::Char('}') => { break; }

//     //                     // toggle selection
//     //                     KeyCode::Char('v') => {
//     //                         if v {
//     //                             v = false;
//     //                             v_end = curpos;
//     //                         } else {
//     //                             v = true;
//     //                             v_start = curpos;
//     //                             v_end = curpos;
//     //                         }
//     //                     }

//     //                     // tag-untag
//     //                     // KeyCode::Char('t') => { break; }
//     //                     // KeyCode::Char('u') => { break; }

//     //                     _ => {}
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }

// }


