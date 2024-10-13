use crossterm::cursor::{MoveTo, MoveToColumn};

pub fn move_to(col: usize, row: usize) -> MoveTo {
    MoveTo(col as u16, row as u16)
}

pub fn move_to_column(col: usize) -> MoveToColumn {
    MoveToColumn(col as u16)
}
