use clap::ValueEnum;
use crossterm::{style::Color, terminal::WindowSize};
use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};

pub const COLORS: [Color; 7] = [
    Color::Black,
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
];

#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    pub filename: String,
    pub change: u8,
    pub color_column: usize,
    pub command: String,
    pub cursor_column: usize,
    pub cursor_row: usize,
    pub labels: Vec<Label>,
    pub lines: Vec<Line>,
    pub modal_active: usize,
    pub modal_row: usize,
    pub modal_start_column: usize,
    pub modal_start_row: usize,
    pub mode: Mode,
    pub nlines: usize,
    pub offset_row: usize,
    #[serde(skip)]
    pub rng: ThreadRng,
    pub visual_row: usize,
    pub visual_start: usize,
    pub visual_end: usize,
    pub window_height: usize,
    pub window_width: usize,
}

impl App {
    pub fn new(filename: &str, lines: Vec<Line>, labels: Vec<Label>, window: WindowSize, rng: ThreadRng) -> App {
        App {
            filename: filename.to_owned(),
            change: 0,
            color_column: 0,
            command: String::new(),
            cursor_column: 0,
            cursor_row: 0,
            nlines: lines.len(),
            labels,
            lines,
            modal_active: 0,
            modal_row: 0,
            modal_start_column: 0,
            modal_start_row: 0,
            mode: Mode::Normal,
            offset_row: 0,
            rng,
            visual_row: 0,
            visual_start: 0,
            visual_end: 0,
            window_height: window.rows as usize,
            window_width: window.columns as usize,
        }
    }

    // u8 00000000
    //           ^    app.mode
    //          ^   app.offset_row
    //         ^  app.cursor_column
    //        ^ app.cursor_row
    pub fn get_change_flags(&mut self) -> Vec<Change> {
        let mut flags = Vec::new();

        if self.change & 0b0001 > 0 { flags.push(Change::Status); }
        if self.change & 0b0010 > 0 { flags.push(Change::Offset); }
        if self.change & 0b1100 > 0 { flags.push(Change::Cursor); }

        flags
    }

    pub fn get_visual_bounds(&self) -> (usize, usize) {
        let mut tmp = [self.visual_start, self.visual_end];
        tmp.sort();
        (tmp[0], tmp[1])
    }

    pub fn get_current_line(&self) -> &Line {
        &self.lines[self.cursor_row + self.offset_row]
    }

    pub fn get_current_line_width(&mut self) -> usize {
        self.lines[self.cursor_row + self.offset_row].width
    }

    pub fn tag(&mut self) {
        let (s,e) = self.get_visual_bounds();

        if s != e {
            self.lines[self.cursor_row + self.offset_row].tags.push(
                Tag { start: s, end: e, label: self.modal_active}
            );
        }
    }

    pub fn untag(&mut self) {
        let tags = self.lines[self.cursor_row].tags.clone();
        self.lines[self.cursor_row + self.offset_row].tags = tags.into_iter().filter(
            |x| !(x.start <= self.cursor_column && self.cursor_column < x.end)
        ).collect();
    }

    pub fn is_modal(&self) -> bool {
        self.mode == Mode::Modal
    }

    pub fn is_normal(&self) -> bool {
        self.mode == Mode::Normal
    }

    pub fn is_visual(&self) -> bool {
        self.mode == Mode::Visual
    }

    pub fn set_color_mode(&mut self) {
        self.mode = Mode::Color;
    }

    pub fn set_command_mode(&mut self) {
        self.mode = Mode::Command;
    }

    pub fn set_modal_mode(&mut self) {
        self.mode = Mode::Modal;
    }

    pub fn set_name_mode(&mut self) {
        self.mode = Mode::Name;
    }

    pub fn set_normal_mode(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn set_visual_mode(&mut self) {
        self.mode = Mode::Visual;
    }
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum FType {
    Raw,
    Spacy,
    Tan,
}

#[derive(Debug, PartialEq)]
pub enum Change {
    Cursor,
    Offset,
    Status,
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
    Color,
    Command,
    Modal,
    Name,
    Normal,
    Visual,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tag {
    pub start: usize,
    pub end: usize,
    pub label: usize,
}
