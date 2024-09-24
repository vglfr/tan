use crossterm::style::Color;
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct App {
    pub fname: String,
    pub qname: String,
    pub color_column: i8,
    pub cursor_column: u16,
    pub cursor_row: u16,
    pub labels: Vec<Label>,
    pub lines: Vec<Line>,
    pub modal_row: i8,
    pub modal_start_column: u16,
    pub modal_start_row: u16,
    pub mode: Mode,
    pub nlines: u16,
    pub offset_column: u16,
    pub offset_row: u16,
    pub visual_row: u16,
    pub visual_start: u16,
    pub visual_end: u16,
    pub window_height: u16,
    pub window_width: u16,
}

impl App {
    pub fn get_visual_bounds(&self) -> (u16, u16) {
        let mut a = [self.visual_start, self.visual_end];
        a.sort();
        (a[0], a[1] + 1)
    }

    pub fn tag(&mut self) {
        let (s,e) = self.get_visual_bounds();
        self.lines[self.visual_row as usize].tags.push(Tag { start: s, end: e, label: Label { name: "tag1".to_owned(), color: Color::Red }});
    }

    pub fn untag(&mut self) {
        let tags = self.lines[self.cursor_row as usize].tags.clone();
        self.lines[self.cursor_row as usize].tags = tags.into_iter().filter(|x| !(x.start <= self.cursor_column && self.cursor_column <= x.end)).collect::<_>();
    }

    pub fn is_modal(&self) -> bool {
        self.mode == Mode::Modal
    }

    pub fn is_view(&self) -> bool {
        self.mode == Mode::View
    }

    pub fn is_visual(&self) -> bool {
        self.mode == Mode::Visual
    }

    pub fn set_color_mode(&mut self) {
        self.mode = Mode::Color;
    }

    pub fn set_modal_mode(&mut self) {
        self.mode = Mode::Modal;
    }

    pub fn set_name_mode(&mut self) {
        self.mode = Mode::Name;
    }

    pub fn set_view_mode(&mut self) {
        self.mode = Mode::View;
    }

    pub fn set_visual_mode(&mut self) {
        self.mode = Mode::Visual;
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Label {
    pub name: String,
    pub color: Color,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Line {
    pub row: u16,
    pub text: String,
    pub width: u16,
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Mode {
    Color,
    Modal,
    Name,
    View,
    Visual,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tag {
    pub start: u16,
    pub end: u16,
    pub label: Label,
}
