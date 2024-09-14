use crossterm::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    pub fname: String,
    pub qname: String,
    pub cursor_column: u16,
    pub cursor_row: u16,
    pub height: u16,
    pub is_modal: bool,
    pub is_visual: bool,
    pub labels: Vec<Label>,
    pub lines: Vec<Line>,
    pub visual_row: u16,
    pub visual_start: u16,
    pub visual_end: u16,
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Chunk {
    pub start: u16,
    pub end: u16,
    pub color: Color,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Label {
    pub name: String,
    pub color: Color,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Line {
    pub row: u16,
    pub text: String,
    pub width: u16,
    pub tags: Vec<Tag>,
}

enum Mode {
    View,
    ViewVisual,
    Modal,
    ModalInput,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tag {
    pub start: u16,
    pub end: u16,
    pub label: Label,
}
