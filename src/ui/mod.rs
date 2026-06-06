pub mod layout;
pub mod theme;
pub mod widgets;

use crate::app::AppState;
use ratatui::prelude::*;

pub fn render(f: &mut Frame, app: &AppState) {
    layout::render_main_layout(f, app);
}
