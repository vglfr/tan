use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crossterm::style::Color;
use crossterm::terminal::{self, WindowSize};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::helper::{App, FType, Label, Line, Tag, COLORS};
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
}

fn load_raw(filename: &str) -> std::io::Result<App> {
    let rng = rand::thread_rng();
    let window = terminal::window_size()?;
    
    let lines = read_raw(filename).unwrap()
        .lines()
        .enumerate()
        .fold((Vec::new(), 0, window.columns as usize - 2), |acc, (i, x)| virtualize_line(acc, (i, x.unwrap())))
        .0;
    let labels = vec![Label { name: "label1".to_owned(), color: Color::Red, is_active: true, is_visible: true }];

    Ok(App::new(filename, lines, labels, window, rng))
}

fn load_spacy(filename: &str) -> std::io::Result<App> {
    let rng = rand::thread_rng();
    let window = terminal::window_size()?;

    let (text, ents, labels) = read_spacy(filename)?;
    let bare_lines = text
        .split("\n")
        .enumerate()
        .fold((Vec::new(), 0, window.columns as usize - 2), |acc, (i, x)| virtualize_line(acc, (i, x.to_owned())))
        .0;
    let lines = assign_labels(bare_lines, &ents, &labels);

    Ok(App::new(filename, lines, labels, window, rng))
}

fn load_tan(filename: &str) -> std::io::Result<App> {
    let s = std::fs::read_to_string(filename)?;
    serde_json::from_str(&s).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
}

fn read_raw(filename: &str) -> std::io::Result<BufReader<File>> {
    let f = File::open(filename)?;
    Ok(BufReader::new(f))
}

fn read_spacy(filename: &str) -> std::io::Result<(String, Vec<Ent>, Vec<Label>)> {
    let f = File::open(filename)?;
    let mut b = BufReader::new(f);

    let spacy: Spacy = serde_json::from_reader(&mut b)?;
    let labels = parse_labels(&spacy.ents);

    Ok((spacy.text, spacy.ents, labels))
}

type Tmp = (Vec<Line>, usize, usize);
type Tmp2 = (usize, String);

fn virtualize_line(acc: Tmp, item: Tmp2) -> Tmp {
    let (mut lines, absolute_offset, window_width) = acc;
    let (absolute_row, text) = item;

    let mut n = 0;
    let text_len = text.len();

    loop {
        let chunk = text[n..std::cmp::min(n + window_width, text_len)].to_string();

        let virtual_text = if n + chunk.len() < text_len {
            chunk.trim_end_matches(|x| !char::is_whitespace(x))
        } else {
            &chunk
        };

        let line = Line {
            absolute_offset: 0,
            absolute_row: 0,
            is_virtual: if n == 0 { false } else { true },
            tags: Vec::new(),
            text: virtual_text.to_owned(),
            virtual_offset: n,
            virtual_row: 0,
            width: virtual_text.len() as u16,
        };

        lines.push(line);
        n += virtual_text.len();

        if n == text_len { break }
    }

    // .map(|(i, mut x)| { x.virtual_row = i as u16; x })

    (lines, absolute_offset, window_width)
}

fn assign_labels(mut lines: Vec<Line>, ents: &Vec<Ent>, labels: &Vec<Label>) -> Vec<Line> {
    dbg!(&ents[..8]);
    dbg!(&lines[..10]);
    todo!();
    for ent in ents {
        let tmp = lines.iter().position(|x| x.virtual_offset as u64 > ent.start);
        if tmp.is_none() {
            dbg!(&ent);
            dbg!(&lines[..8]);
        }
        let n = tmp.unwrap();

        let o = lines[n-1].virtual_offset;
        let w = lines[n-1].width;

        let start = ((ent.start - o as u64) % w as u64) as u16;
        let end = ((ent.end - o as u64) % w as u64) as u16;

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

    lines
}

fn parse_labels(ents: &Vec<Ent>) -> Vec<Label> {
    let mut labels = Vec::new();
    let mut colors = COLORS.iter().cycle();

    for ent in ents {
        if !labels.iter().map(|x: &Label| &x.name).contains(&ent.label) {
            let label = Label {
                name: ent.label.clone(),
                color: *colors.next().unwrap(),
                is_active: if labels.is_empty() { true } else { false },
                is_visible: true,
            };

            labels.push(label);
        }
    }

    labels
}

pub fn save_tan(app: &mut App) -> std::io::Result<()> {
    let mode = app.mode.clone();
    app.set_normal_mode();

    if !app.filename.ends_with(".tan") { app.filename.push_str(".tan") };
    let mut f = File::create(&app.filename)?;

    let s = serde_json::to_string(&app)?;
    f.write_all(s.as_bytes())?;

    app.mode = mode;
    Ok(())
}

pub fn dump_debug(app: &App) -> std::io::Result<()> {
    let mut f = File::create("/tmp/dbg.json").unwrap();
    f.write_all(serde_json::to_string(app).unwrap().as_bytes())
}
