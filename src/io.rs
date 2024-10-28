use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use anyhow::Result;
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
type Pair = (usize, usize);

#[allow(private_interfaces)]
pub fn load_file(argv: &Argv) -> Result<App> {
    let format = argv.format.clone()
        .unwrap_or_else(||
            if argv.name.ends_with(".tan") { FType::Tan }
            else if argv.name.ends_with(".json") { FType::Spacy }
            else { FType::Raw }
        );

    match format {
        FType::Raw => load_raw(&argv.name),
        FType::Spacy => load_spacy(&argv.name),
        FType::Tan => load_tan(&argv.name),
    }
}

fn load_raw(filename: &str) -> Result<App> {
    let window = terminal::window_size()?;
    
    let lines = File::open(filename)
        .map(BufReader::new)?
        .lines()
        .fold_ok(Vec::new(), |mut acc, x| { acc.push(x); acc })?
        .into_iter()
        .enumerate()
        .fold((Vec::new(), 0, window.columns as usize - 2), virtualize_line)
        .0;
    let labels = vec![Label { name: "label1".to_owned(), color: Color::Red, is_active: true, is_visible: true }];

    Ok(App::new(filename, lines, labels, window))
}

fn load_spacy(filename: &str) -> Result<App> {
    let window = terminal::window_size()?;

    let (text, ents, labels) = read_spacy(filename)?;
    let bare_lines = text.split("\n")
        .map(|x| x.to_owned())
        .enumerate()
        .fold((Vec::new(), 0, window.columns as usize - 2), virtualize_line)
        .0;
    let lines = assign_labels(bare_lines, &ents, &labels);

    Ok(App::new(filename, lines, labels, window))
}

fn load_tan(filename: &str) -> Result<App> {
    let s = std::fs::read_to_string(filename)?;
    serde_json::from_str(&s).map_err(anyhow::Error::from)
}

fn read_spacy(filename: &str) -> Result<(String, Vec<Ent>, Vec<Label>)> {
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
        let label = labels.iter().position(|x| x.name == ent.label).unwrap();

        let intervals = lines.iter().enumerate()
            .filter(|(_,x)| {
                let tag = (ent.start, ent.end);
                let line = (x.absolute_offset, x.absolute_offset + x.width);
                has_interval_overlap(tag, line)
            })
            .map(|(i,x)| {
                let tag = (ent.start, ent.end);
                let line = (x.absolute_offset, x.width);

                let bounds = calculate_tag_bounds(tag, line);
                (i, bounds)
            })
            .collect::<Vec<(usize,Pair)>>();

        for (i,(n,(start,end))) in intervals.iter().enumerate() {
            let tag = Tag {
                start: *start,
                end: *end,
                label,
                has_line_prev: i != 0,
                has_line_next: i != intervals.len() - 1,
            };
            lines[*n].tags.push(tag);
        }
    }

    lines
}

fn has_interval_overlap(tag: Pair, line: Pair) -> bool {
    let (tag_start, tag_end) = tag;
    let (line_start, line_end) = line;

    (tag_end > line_start) ^ (tag_start > line_end)
}

fn calculate_tag_bounds(tag: Pair, line: Pair) -> Pair {
    let (tag_start, tag_end) = tag;
    let (line_offset, line_width) = line;

    let left = line_offset + line_width - tag_start;
    let right = (line_offset + line_width).saturating_sub(tag_end);

    (line_width.saturating_sub(left), line_width - right)
}

// t      ---
// l     -----
//      s2 s1 e1 e2

// t     -----
// l      ---
//      s1 s2 e2 e1

// t     -----
// l        ---
//      s1 s2 e1 e2

// t     -----
// l   ---
//      s2 s1 e2 e1

// t       -----
// l   ---
//      s2 e2 s1 e1

// t  -----
// l        ---
//      s1 e1 s2 e2
#[test]
fn test_has_interval_overlap() {
    assert!(has_interval_overlap((11,13), (10,14)) == true);
    assert!(has_interval_overlap((10,14), (11,13)) == true);
    assert!(has_interval_overlap((10,12), (11,14)) == true);
    assert!(has_interval_overlap((11,13), (10,12)) == true);
    assert!(has_interval_overlap((13,15), (10,12)) == false);
    assert!(has_interval_overlap((10,12), (13,15)) == false);
}

#[test]
fn test_calculate_tag_bounds() {
    assert!(calculate_tag_bounds((100,120), (80,80)) == (20,40));
    assert!(calculate_tag_bounds(( 60,180), (80,80)) == ( 0,80));
    assert!(calculate_tag_bounds(( 60,120), (80,80)) == ( 0,40));
    assert!(calculate_tag_bounds((100,180), (80,80)) == (20,80));
}

fn parse_labels(ents: &Vec<Ent>) -> Vec<Label> {
    let mut labels = Vec::new();
    let mut colors = COLORS.iter().cycle();

    for ent in ents {
        if !labels.iter().map(|x: &Label| &x.name).contains(&ent.label) {
            let label = Label {
                name: ent.label.chars().take(20).collect(),
                color: *colors.next().unwrap(),
                is_active: if labels.is_empty() { true } else { false },
                is_visible: true,
            };

            labels.push(label);
        }
    }

    labels
}

pub fn save_tan(app: &mut App) -> Result<()> {
    let mode = app.mode.clone();
    app.set_normal_mode();

    if !app.filename.ends_with(".tan") { app.filename.push_str(".tan") };
    let mut f = File::create(&app.filename)?;

    let s = serde_json::to_string(&app)?;
    f.write_all(s.as_bytes())?;

    app.mode = mode;
    Ok(())
}

pub fn dump_debug(app: &App) -> Result<()> {
    let mut f = File::create("/tmp/dbg.json")?;
    f.write_all(serde_json::to_string(app)?.as_bytes()).map_err(anyhow::Error::from)
}
