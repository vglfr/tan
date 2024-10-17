use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crossterm::{style::Color, terminal};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::app::{App, FType, Label, Line, Tag, COLORS};
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
    end: usize,
    label: String,
    start: usize,
}

type Accumulator = (Vec<Line>, usize, usize);
type Enumerate = (usize, String);

#[allow(private_interfaces)]
pub fn load_file(argv: &Argv) -> std::io::Result<App> {
    match argv.format {
        FType::Raw => load_raw(&argv.name),
        FType::Spacy => load_spacy(&argv.name),
        FType::Tan => load_tan(&argv.name),
    }
}

fn load_raw(filename: &str) -> std::io::Result<App> {
    let window = terminal::window_size()?;
    
    let lines = read_raw(filename).unwrap()
        .lines()
        .enumerate()
        .fold((Vec::new(), 0, window.columns as usize - 2), |acc, (i, x)| virtualize_line(acc, (i, x.unwrap())))
        .0;
    let labels = vec![Label { name: "label1".to_owned(), color: Color::Red, is_active: true, is_visible: true }];

    Ok(App::new(filename, lines, labels, window))
}

fn load_spacy(filename: &str) -> std::io::Result<App> {
    let window = terminal::window_size()?;

    let (text, ents, labels) = read_spacy(filename)?;
    let bare_lines = text
        .split("\n")
        .enumerate()
        .fold((Vec::new(), 0, window.columns as usize - 2), |acc, (i, x)| virtualize_line(acc, (i, x.to_owned())))
        .0;
    let lines = assign_labels(bare_lines, &ents, &labels);

    Ok(App::new(filename, lines, labels, window))
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

fn virtualize_line(acc: Accumulator, item: Enumerate) -> Accumulator {
    let (mut lines, mut absolute_offset, window_width) = acc;
    let (absolute_row, text) = item;

    let mut virtual_offset = 0;

    loop {
        let chunk = text[virtual_offset..std::cmp::min(virtual_offset + window_width, text.len())].to_string();

        let virtual_text = if virtual_offset + chunk.len() < text.len() {
            chunk.trim_end_matches(|x| !char::is_whitespace(x))
        } else {
            &chunk
        };

        let line = Line {
            absolute_offset,
            absolute_row,
            is_virtual: virtual_offset > 0,
            tags: Vec::new(),
            text: virtual_text.to_owned(),
            virtual_offset,
            virtual_row: lines.len(),
            width: virtual_text.len(),
        };

        lines.push(line);

        absolute_offset += virtual_text.len();
        virtual_offset += virtual_text.len();

        if virtual_offset == text.len() {
            absolute_offset += 1;
            break
        }
    }

    (lines, absolute_offset, window_width)
}

fn assign_labels(mut lines: Vec<Line>, ents: &Vec<Ent>, labels: &Vec<Label>) -> Vec<Line> {
    for ent in ents {
        let tmp = lines.iter().position(|x| x.absolute_offset > ent.start);
        if tmp.is_none() {
            dbg!(&ent);
            dbg!(&lines[..8]);
        }
        let n = tmp.unwrap();

        let o = lines[n-1].absolute_offset;
        let w = lines[n-1].width;

        let start = (ent.start - o) % w;
        let end = (ent.end - o) % w;

        if start < end || end == 0 {
            lines[n-1].tags.push(Tag {
                start,
                end,
                label: labels.iter().position(|x| x.name == ent.label).unwrap(),
                has_line_prev: false,
                has_line_next: false,
            });
        } else {
            lines[n-1].tags.push(Tag {
                start,
                end: w,
                label: labels.iter().position(|x| x.name == ent.label).unwrap(),
                has_line_prev: false,
                has_line_next: true,
            });

            lines[n].tags.push(Tag {
                start: 0,
                end,
                label: labels.iter().position(|x| x.name == ent.label).unwrap(),
                has_line_prev: true,
                has_line_next: false,
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
