use super::super::app::App;
use super::common_events;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handler(key: KeyEvent, app: &mut App) {
    match key {
        k if common_events::right_event(k) => common_events::handle_right_event(app),
        k if common_events::down_event(k) => {
            match &app.playlists {
                Some(p) => {
                    if let Some(selected_playlist_index) = app.selected_playlist_index {
                        let next_index =
                            common_events::on_down_press_handler(&p, Some(selected_playlist_index));
                        app.selected_playlist_index = Some(next_index);
                    }
                }
                None => {}
            };
        }
        k if common_events::up_event(k) => {
            match &app.playlists {
                Some(p) => {
                    let next_index =
                        common_events::on_up_press_handler(&p, app.selected_playlist_index);
                    app.selected_playlist_index = Some(next_index);
                }
                None => {}
            };
        }
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        } => {
            if let (Some(playlists), Some(selected_playlist_index)) =
                (&app.playlists, &app.selected_playlist_index)
            {
                if let Some(selected_playlist) = playlists.get(selected_playlist_index.to_owned()) {
                    let playlist_id = selected_playlist.id.to_owned().unwrap();
                    app.get_playlist_tracks(playlist_id.to_string());
                }
            };
        }
        _ => {}
    }
}
