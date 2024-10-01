use crate::helper::App;

pub fn handle_v(app: &mut App) {
    app.set_normal_mode();
    app.visual_end = app.cursor_column;
}
