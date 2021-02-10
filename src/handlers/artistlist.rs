use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::super::app::App;
use super::common_events;

pub fn handler(key: KeyEvent, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(artistlist) = &mut app.artist_list {
                let next_index = common_events::on_down_press_handler(
                    &artistlist.artists,
                    Some(artistlist.selected_index),
                );
                artistlist.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(artistlist) = &mut app.artist_list {
                let next_index = common_events::on_up_press_handler(
                    &artistlist.artists,
                    Some(artistlist.selected_index),
                );
                artistlist.selected_index = next_index;
            }
        }
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        } => {
            if let Some(artistlist) = &app.artist_list {
                if let Some(artist) = artistlist.artists.get(artistlist.selected_index.to_owned()) {
                    let artist_id = artist.id;
                    app.get_artist_albums(artist_id.to_string());
                }
            };
        }
        KeyEvent {
            code: KeyCode::Char('f'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            let limit = (app.block_height - 4) as i32;
            if let Some(artistlist) = &app.artist_list {
                let page = artistlist.selected_page;
                let next_page = (page + 1) as i32;
                app.get_top_artists(limit, next_page)
            };
        }
        KeyEvent {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            let limit = (app.block_height - 4) as i32;
            if let Some(artistlist) = &app.artist_list {
                let page = artistlist.selected_page;
                let next_page = if page < 1 { 0 } else { page - 1 } as i32;
                app.get_top_artists(limit, next_page)
            };
        }
        _ => {}
    }
}
