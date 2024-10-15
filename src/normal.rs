use std::io::Stdout;

use crate::{app::App, render};

pub fn handle_u(app: &mut App, stdout: &mut Stdout) -> std::io::Result<()> {
    app.untag();
    render::render_offset(app, stdout)
}

pub fn handle_v(app: &mut App) {
    app.set_visual_mode();
    app.visual_row = app.cursor_row + app.offset_row;
    app.visual_start = app.cursor_column;
    app.visual_end = app.cursor_column;
}
