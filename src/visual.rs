use crate::helper::App;

pub fn handle_v(app: &mut App) -> std::io::Result<()> {
    app.set_view_mode();
    app.visual_end = app.cursor_column;
    Ok(())
}
