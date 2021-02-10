use super::super::app::{App, TrackTable};
use super::common_events;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handler(key: KeyEvent, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(selected_album) = &mut app.selected_album {
                let next_index = common_events::on_down_press_handler(
                    &selected_album.tracks,
                    Some(selected_album.selected_index),
                );
                selected_album.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(selected_album) = &mut app.selected_album {
                let next_index = common_events::on_up_press_handler(
                    &selected_album.tracks,
                    Some(selected_album.selected_index),
                );
                selected_album.selected_index = next_index;
            }
        }
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::CONTROL,
        } => {
            if let Some(selected_album) = &mut app.selected_album {
                app.my_playlist = TrackTable {
                    tracks: selected_album.tracks.to_owned(),
                    selected_index: selected_album.selected_index,
                    name: selected_album.album.name.to_owned().unwrap(),
                };
                if let Some(selected_track) = selected_album
                    .tracks
                    .get(selected_album.selected_index)
                    .cloned()
                {
                    app.start_playback(selected_track);
                    app.fm_state = false;
                }
            };
        }
        _ => {}
    }
}
