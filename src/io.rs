use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crossterm::style::Color;
use crossterm::terminal;

use crate::helper::{App, Label, Line, Mode};

pub fn load_file(fname: &str, qname: &str) -> std::io::Result<App> {
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

    let window = terminal::window_size().unwrap();

    let app = App {
        fname: fname.to_owned(),
        qname: qname.to_owned(),
        color_column: 0,
        cursor_column: 0,
        cursor_row: 0,
        nlines: lines.len() as u16,
        labels: vec![
            Label { name: "label1".to_owned(), color: Color::Red },
            Label { name: "lab2".to_owned(), color: Color::Yellow },
            Label { name: "label3".to_owned(), color: Color::Blue },
            Label { name: "label4label4".to_owned(), color: Color::Green },
        ],
        lines,
        modal_row: 0,
        modal_start_column: 0,
        modal_start_row: 0,
        mode: Mode::View,
        offset_column: 0,
        offset_row: 0,
        visual_row: 0,
        visual_start: 0,
        visual_end: 0,
        window_height: window.rows,
        window_width: window.columns,
    };

    Ok(app)

    // prevent from opening non-utf8
}

pub fn save_tan(app: &App) -> std::io::Result<()> {
    let mut a = (*app).clone();
    a.mode = Mode::View;

    let s = serde_json::to_string(&a)?;
    let mut f = File::create(&a.qname)?;

    f.write_all(s.as_bytes())?;
    Ok(())
}

pub fn dump_debug(app: &App) -> std::io::Result<()> {
    let mut f = File::create("/tmp/dbg.json").unwrap();
    f.write_all(serde_json::to_string(app).unwrap().as_bytes())
}
