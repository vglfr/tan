use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crossterm::style::Color;
use crossterm::terminal::{self, WindowSize};
use itertools::Itertools;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::helper::{App, FType, Label, Line, Mode, Tag, COLORS};
use crate::Argv;

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
    end: u64,
    label: String,
    start: u64,
}

pub fn load_file(argv: &Argv) -> std::io::Result<App> {
    match argv.format {
        FType::Raw => load_raw(&argv.name),
        FType::Spacy => load_spacy(&argv.name),
        FType::Tan => load_tan(&argv.name),
    }

    // prevent from opening non-utf8
}

fn load_raw(fname: &str) -> std::io::Result<App> {
    let lines = read_rlines(fname)?;
    let labels = vec![
        Label { name: "new label".to_owned(), color: Color::Red, is_active: true, is_visible: true },
    ];

    let window = terminal::window_size().unwrap();
    let rng = rand::thread_rng();
    
    Ok(App::new(fname, lines, labels, window, rng))
}

fn load_spacy(fname: &str) -> std::io::Result<App> {
    let mut rng = rand::thread_rng();
    let (lines, labels, window) = read_slines(fname, &mut rng)?;
    Ok(App::new(fname, lines, labels, window, rng))
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
            char_offset: 0, // todo
            is_wrapping: false,
            row: n,
            tags: Vec::new(),
            text: s.strip_suffix("\n").map(|x| x.to_owned()).unwrap_or(s.clone()),
            width: s.len() as u16 - 1,
        };
        lines.push(l);

        n += 1;
        s.truncate(0);
    }

    Ok(lines)
}

fn read_slines(fname: &str, rng: &mut ThreadRng) -> std::io::Result<(Vec<Line>, Vec<Label>, WindowSize)> {
    let f = File::open(fname).unwrap();
    let mut b = BufReader::new(f);

    let spacy: Spacy = serde_json::from_reader(&mut b)?;
    let window = terminal::window_size().unwrap();

    let mut lines: Vec<Line> = Vec::new();
    let mut n = 0;
    let mut k = 0;

    while n < spacy.text.len() {
        let text = spacy.text[n..std::cmp::min(n + window.columns as usize, spacy.text.len())].to_owned();

        let ttext = if n + text.len() < spacy.text.len() {
            text.trim_end_matches(|x| !char::is_whitespace(x)).trim_end()
        } else {
            &text
        };

        let line = Line {
            char_offset: if lines.is_empty() { 0 } else { lines[lines.len() - 1].char_offset + lines[lines.len() - 1].width as u64 },
            is_wrapping: true,
            row: k,
            tags: Vec::new(),
            text: ttext.to_owned(),
            width: ttext.len() as u16,
        };

        lines.push(line);

        k += 1;
        n += ttext.len();
    }

    let mut labels: Vec<Label> = spacy.ents.iter()
        .map(|x| x.label.clone())
        .unique()
        .map(|x| Label { is_active: false, name: x, color: *COLORS.choose(rng).unwrap(), is_visible: true })
        .collect();
    labels[0].is_active = true;

    for ent in spacy.ents {
        let n = lines.iter().position(|x| x.char_offset > ent.start).unwrap();

        let o = lines[n-1].char_offset;
        let w = lines[n-1].width;

        let start = ((ent.start - o) % w as u64) as u16;
        let end = ((ent.end - o) % w as u64) as u16;

        if start < end || end == 0 {
            lines[n-1].tags.push(Tag {
                start,
                end,
                label: labels.iter().position(|x| x.name == ent.label).unwrap(),
            });
        } else {
            lines[n-1].tags.push(Tag {
                start,
                end: 0,
                label: labels.iter().position(|x| x.name == ent.label).unwrap(),
            });

            lines[n].tags.push(Tag {
                start: 0,
                end,
                label: labels.iter().position(|x| x.name == ent.label).unwrap(),
            });
        }
    }

    // terminal::disable_raw_mode()?;
    // dbg!(&lines);
    // todo!();

    Ok((lines, labels, window))
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
