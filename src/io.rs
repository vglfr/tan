use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crossterm::style::Color;
use crossterm::terminal::{self, WindowSize};
use serde::{Deserialize, Serialize};

use crate::helper::{App, FType, Label, Line, Mode};

#[derive(Debug, Deserialize, Serialize)]
struct Spacy {
    text: String,
    ents: Vec<Ent>,

    #[allow(dead_code)]
    #[serde(skip)]
    sents: Option<String>,
    #[allow(dead_code)]
    #[serde(skip)]
    tokens: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Ent {
    end: u32,
    label: String,
    start: u32,
}

pub fn load_file(fname: &str, ftype: FType) -> std::io::Result<App> {
    match ftype {
        FType::Raw => load_raw(fname),
        FType::Spacy => load_spacy(fname),
        FType::Tan => load_tan(fname),
    }

    // prevent from opening non-utf8
}

fn load_raw(fname: &str) -> std::io::Result<App> {
    let lines = read_rlines(fname)?;
    let labels = vec![
        Label { name: "label1".to_owned(), color: Color::Red, is_active: true },
        Label { name: "lab2".to_owned(), color: Color::Yellow, is_active: false },
        Label { name: "label3".to_owned(), color: Color::Blue, is_active: false },
        Label { name: "label4label4".to_owned(), color: Color::Green, is_active: false },
    ];

    let window = terminal::window_size().unwrap();
    Ok(App::new(fname, lines, labels, window))
}

fn load_spacy(fname: &str) -> std::io::Result<App> {
    let (lines, labels, window) = read_slines(fname)?;
    Ok(App::new(fname, lines, labels, window))
}

fn load_tan(fname: &str) -> std::io::Result<App> {
    let s = std::fs::read_to_string(fname)?;
    serde_json::from_str(&s).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
}

fn read_rlines(fname: &str) -> std::io::Result<Vec<Line>> {
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

    Ok(lines)
}

fn read_slines(fname: &str) -> std::io::Result<(Vec<Line>, Vec<Label>, WindowSize)> {
    let f = File::open(fname).unwrap();
    let mut b = BufReader::new(f);

    let spacy: Spacy = serde_json::from_reader(&mut b)?;
    let window = terminal::window_size().unwrap();

    // let lines = s.text.chars().collect::<Vec<char>>(); //.chunks(window.width as usize - 10);

    let mut lines: Vec<Line> = Vec::new();
    let mut n = 0;
    let mut k = 0;

    // terminal::disable_raw_mode()?;

    while n < spacy.text.len() {
        let text = spacy.text[n..std::cmp::min(n + window.columns as usize, spacy.text.len())].to_owned();

        let ttext = if n + text.len() < spacy.text.len() {
            text.trim_end_matches(|x| !char::is_whitespace(x)).trim_end()
        } else {
            &text
        };

        let line = Line { row: k, width: ttext.len() as u16, text: ttext.to_owned(), tags: Vec::new() };
        lines.push(line);

        k += 1;
        n += ttext.len();
    }

    // take window length partition
    // go backwards looking for a whitespace at most window/4 characters
    // if found then copy
    // otherwise just chunk on a long word (special symbol)

    // dbg!(&lines);
    // dbg!(&lines.chunks(window.columns as usize - 10).map(|x| x.iter().collect::<String>()).collect::<Vec<String>>());
    // todo!();

    Ok((lines, Vec::new(), window))
}

pub fn save_tan(app: &mut App) -> std::io::Result<()> {
    let mode = app.mode.clone();
    app.mode = Mode::View;

    if !app.fname.ends_with(".tan") { app.fname.push_str(".tan") };
    let mut f = File::create(&app.fname)?;

    let s = serde_json::to_string(&app)?;
    f.write_all(s.as_bytes())?;

    app.mode = mode;
    Ok(())
}

pub fn dump_debug(app: &App) -> std::io::Result<()> {
    let mut f = File::create("/tmp/dbg.json").unwrap();
    f.write_all(serde_json::to_string(app).unwrap().as_bytes())
}
