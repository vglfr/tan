use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use crossterm::style::Color;
use crossterm::terminal::{self, WindowSize};
use itertools::Itertools;
use rand::{rngs::ThreadRng, seq::SliceRandom};
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
        .flat_map(|x| virtualize_new_line(x.unwrap(), &window))
        .enumerate()
        .map(|(i, mut x)| { x.row = i as u16; x })
        .collect();
    let labels = vec![Label { name: "label1".to_owned(), color: Color::Red, is_active: true, is_visible: true }];

    Ok(App::new(filename, lines, labels, window, rng))
}

fn load_spacy(filename: &str) -> std::io::Result<App> {
    let rng = rand::thread_rng();
    let window = terminal::window_size()?;

    let (text, ents, labels) = read_spacy(filename)?;
    let lines = text.split("\n").flat_map(|x| virtualize_existing_line(x, &ents, &labels, &window)).collect();

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

fn virtualize_new_line(text: String, window: &WindowSize) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut n = 0;

    let text_len = text.len();
    let max_chunk = window.columns as usize - 2;

    loop {
        let chunk = text[n..std::cmp::min(max_chunk + n, text_len)].to_string();

        let virtual_text = if n + chunk.len() < text_len {
            chunk.trim_end_matches(|x| !char::is_whitespace(x))
        } else {
            &chunk
        };

        let line = Line {
            is_virtual: if n == 0 { false } else { true },
            row: 0,
            tags: Vec::new(),
            text: virtual_text.to_owned(),
            width: virtual_text.len() as u16,
            wrapping_offset: n,
        };

        lines.push(line);
        n += virtual_text.len();

        if n == text_len { break }
    }

    lines
}

fn virtualize_existing_line(line: &str, ents: &Vec<Ent>, labels: &Vec<Label>, window: &WindowSize) -> Vec<Line> {
    // let window = terminal::window_size().unwrap();

    // let mut lines: Vec<Line> = Vec::new();
    // let mut n = 0;
    // let mut k = 0;

    // while n < spacy.text.len() {
    //     let text = spacy.text[n..std::cmp::min(n + window.columns as usize, spacy.text.len())].to_owned();

    //     let ttext = if n + text.len() < spacy.text.len() {
    //         text.trim_end_matches(|x| !char::is_whitespace(x)).trim_end()
    //     } else {
    //         &text
    //     };

    //     let line = Line {
    //         is_virtual: true,
    //         row: k,
    //         tags: Vec::new(),
    //         text: ttext.to_owned(),
    //         width: ttext.len() as u16,
    //         wrapping_whitespace: None,
    //         wrapping_offset: Some(if lines.is_empty() { 0 } else { lines[lines.len() - 1].wrapping_offset.unwrap() + lines[lines.len() - 1].width as u64 }),
    //     };

    //     lines.push(line);

    //     k += 1;
    //     n += ttext.len();
    // }

    // for ent in spacy.ents {
    //     let n = lines.iter().position(|x| x.wrapping_offset.unwrap() > ent.start).unwrap();

    //     let o = lines[n-1].wrapping_offset.unwrap();
    //     let w = lines[n-1].width;

    //     let start = ((ent.start - o) % w as u64) as u16;
    //     let end = ((ent.end - o) % w as u64) as u16;

    //     if start < end || end == 0 {
    //         lines[n-1].tags.push(Tag {
    //             start,
    //             end,
    //             label: labels.iter().position(|x| x.name == ent.label).unwrap(),
    //         });
    //     } else {
    //         lines[n-1].tags.push(Tag {
    //             start,
    //             end: 0,
    //             label: labels.iter().position(|x| x.name == ent.label).unwrap(),
    //         });

    //         lines[n].tags.push(Tag {
    //             start: 0,
    //             end,
    //             label: labels.iter().position(|x| x.name == ent.label).unwrap(),
    //         });
    //     }
    // }

    // let mut labels: Vec<Label> = spacy.ents.iter()
    //     .map(|x| x.label.clone())
    //     .unique()
    //     .map(|x| Label { is_active: false, name: x, color: *COLORS.choose(rng).unwrap(), is_visible: true })
    //     .collect();
    // labels[0].is_active = true;

    // // terminal::disable_raw_mode()?;
    // // dbg!(&lines);
    // // todo!();

    // Ok((lines, labels, window))
    Vec::new()
}

// fn virtualize_line(line: String) -> Vec<Line> {
//     Vec::new()
// }

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
