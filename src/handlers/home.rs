use super::super::app::App;
use super::common_events;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handler(key: KeyEvent, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        } => {
            let current_hovered = app.get_current_route().hovered_block;
            app.set_current_route_state(Some(current_hovered), None);
        }
        _ => {}
    }
}
