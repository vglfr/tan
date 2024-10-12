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
    pub color_column: i8,
    pub command: String,
    pub cursor_column: u16,
    pub cursor_row: u16,
    pub labels: Vec<Label>,
    pub lines: Vec<Line>,
    pub modal_active: usize,
    pub modal_row: i8,
    pub modal_start_column: u16,
    pub modal_start_row: u16,
    pub mode: Mode,
    pub nlines: u16,
    pub offset_row: u16,
    #[serde(skip)]
    pub rng: ThreadRng,
    pub visual_row: u16,
    pub visual_start: u16,
    pub visual_end: u16,
    pub window_height: u16,
    pub window_width: u16,
}

impl App {
    pub fn new(filename: &str, lines: Vec<Line>, labels: Vec<Label>, window: WindowSize, rng: ThreadRng) -> App {
        App {
            filename: filename.to_owned(),
            color_column: 0,
            command: String::new(),
            cursor_column: 0,
            cursor_row: 0,
            nlines: lines.len() as u16,
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
            window_height: window.rows,
            window_width: window.columns,
        }
    }

    pub fn get_visual_bounds(&self) -> (u16, u16) {
        let mut a = [self.visual_start, self.visual_end];
        a.sort();
        (a[0], a[1])
    }

    pub fn get_current_line(&mut self) -> &Line {
        &self.lines[(self.cursor_row + self.offset_row) as usize]
    }

    pub fn get_current_line_width(&mut self) -> u16 {
        self.lines[(self.cursor_row + self.offset_row) as usize].width
    }

    pub fn tag(&mut self) {
        let (s,e) = self.get_visual_bounds();

        if s != e {
            self.lines[(self.cursor_row + self.offset_row) as usize].tags.push(
                Tag { start: s, end: e, label: self.modal_active}
            );
        }
    }

    pub fn untag(&mut self) {
        let tags = self.lines[self.cursor_row as usize].tags.clone();
        self.lines[(self.cursor_row + self.offset_row) as usize].tags = tags.into_iter().filter(
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Label {
    pub name: String,
    pub color: Color,
    pub is_active: bool,
    pub is_visible: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Line {
    pub is_virtual: bool,
    pub row: u16,
    pub tags: Vec<Tag>,
    pub text: String,
    pub width: u16,
    pub wrapping_offset: usize,
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
    pub start: u16,
    pub end: u16,
    pub label: usize,
}
