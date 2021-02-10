use super::super::app::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
// use super::common_events;

pub fn handler(key: KeyEvent, app: &mut App) {
    match key {
        KeyEvent {
            code: KeyCode::Char('t'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            app.fm_trash();
        }
        _ => {}
    }
}
