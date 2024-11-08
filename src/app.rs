use clap::ValueEnum;
use crossterm::{style::Color, terminal::WindowSize};
use serde::{Deserialize, Serialize};

pub const COLORS: [Color; 18] = [
    Color::AnsiValue(98),
    Color::AnsiValue(176),
    Color::AnsiValue(206),
    Color::AnsiValue(102),
    Color::AnsiValue(212),
    Color::AnsiValue(30),
    Color::AnsiValue(66),
    Color::AnsiValue(60),
    Color::AnsiValue(170),
    Color::AnsiValue(138),
    Color::AnsiValue(24),
    Color::AnsiValue(68),
    Color::AnsiValue(134),
    Color::AnsiValue(168),
    Color::AnsiValue(204),
    Color::AnsiValue(26),
    Color::AnsiValue(32),
    Color::AnsiValue(104),
];

#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    pub filename: String,
    pub change: u8,
    pub command: String,
    pub cursor_column: usize,
    pub cursor_row: usize,
    pub labels: Vec<Label>,
    pub lines: Vec<Line>,
    pub modal_active: usize,
    pub modal_column: usize,
    pub modal_row: usize,
    pub modal_start_column: usize,
    pub modal_start_row: usize,
    pub mode: Mode,
    pub nlines: usize,
    pub offset_row: usize,
    pub rng: usize,
    pub visual: Vec<Visual>,
    pub window_height: usize,
    pub window_width: usize,
}

impl App {
    pub fn new(filename: &str, lines: Vec<Line>, labels: Vec<Label>, window: WindowSize) -> App {
        let window_height = window.rows as usize;
        let window_width = window.columns as usize;

        App {
            filename: filename.to_owned(),
            change: 0,
            command: String::new(),
            cursor_column: 0,
            cursor_row: 0,
            nlines: lines.len(),
            labels,
            lines,
            modal_active: 0,
            modal_column: 0,
            modal_row: 0,
            modal_start_column: window_width / 2 - 20,
            modal_start_row: window_height / 2 - 12,
            mode: Mode::Normal,
            offset_row: 0,
            rng: 0,
            visual: Vec::new(),
            window_height,
            window_width,
        }
    }

    pub fn get_visual_bounds(&self, row: usize) -> (usize, usize) {
        self.visual
            .iter()
            .find(|x| x.row == row)
            .map(|x| {
                let mut tmp = [x.start, x.end];
                tmp.sort();
                (tmp[0], tmp[1] + 1)
            })
            .unwrap_or((0, 0))
    }

    pub fn get_current_line(&self) -> &Line {
        &self.lines[self.cursor_row + self.offset_row]
    }

    pub fn get_current_line_width(&self) -> usize {
        self.lines[self.cursor_row + self.offset_row].width
    }

    pub fn tag(&mut self) {
        let (s, e) = self.get_visual_bounds(self.cursor_row + self.offset_row);

        if e - s > 1 {
            let tag = Tag {
                start: s,
                end: e,
                label: self.modal_active,
                has_line_next: false,
                has_line_prev: false,
            };
            self.lines[self.cursor_row + self.offset_row].tags.push(tag);
            self.change = 0b0011;
        }

        self.visual.clear();
    }

    pub fn untag(&mut self) {
        let position_maybe = self
            .get_current_line()
            .tags
            .iter()
            .position(|x| x.start <= self.cursor_column && self.cursor_column < x.end);

        if let Some(position) = position_maybe {
            let row = self.cursor_row + self.offset_row;
            let tag = self.lines[row].tags.remove(position);

            self.untag_next(&tag, row);
            self.untag_prev(&tag, row);

            self.change = 0b0011;
        }
    }

    fn untag_next(&mut self, tag: &Tag, row: usize) {
        if tag.has_line_next {
            let tag_next = self.lines[row + 1].tags.remove(0);
            self.untag_next(&tag_next, row + 1);
        }
    }

    fn untag_prev(&mut self, tag: &Tag, row: usize) {
        if tag.has_line_prev {
            let tag_prev = self.lines[row - 1]
                .tags
                .pop()
                .expect("Error dereferencing previous tag");
            self.untag_prev(&tag_prev, row - 1);
        }
    }
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum FType {
    Plain,
    Spacy,
    Tan,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    pub name: String,
    pub color: Color,
    pub is_active: bool,
    pub is_visible: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Line {
    pub absolute_offset: usize,
    pub absolute_row: usize,
    pub is_virtual: bool,
    pub tags: Vec<Tag>,
    pub text: String,
    pub virtual_offset: usize,
    pub virtual_row: usize,
    pub width: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Mode {
    Command,
    Modal,
    Name,
    Normal,
    Visual,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    pub start: usize,
    pub end: usize,
    pub label: usize,
    pub has_line_prev: bool,
    pub has_line_next: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Visual {
    pub row: usize,
    pub start: usize,
    pub end: usize,
}
