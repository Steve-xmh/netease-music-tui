use super::super::app::App;
use super::common_events;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handler(key: KeyEvent, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(albumlist) = &mut app.album_list {
                let next_index = common_events::on_down_press_handler(
                    &albumlist.albums,
                    Some(albumlist.selected_index),
                );
                albumlist.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(albumlist) = &mut app.album_list {
                let next_index = common_events::on_up_press_handler(
                    &albumlist.albums,
                    Some(albumlist.selected_index),
                );
                albumlist.selected_index = next_index;
            }
        }
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        } => {
            if let Some(albumlist) = &app.album_list {
                if let Some(album) = albumlist.albums.get(albumlist.selected_index.to_owned()) {
                    let album_id = album.id.to_owned().unwrap();
                    app.get_album_tracks(album_id.to_string());
                }
            };
        }
        KeyEvent {
            code: KeyCode::Char('f'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            let limit = (app.block_height - 4) as i32;
            if let Some(albumlist) = &app.album_list {
                let page = albumlist.selected_page;
                let next_page = (page + 1) as i32;
                app.get_top_albums(limit, next_page)
            };
        }
        KeyEvent {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            let limit = (app.block_height - 4) as i32;
            if let Some(albumlist) = &app.album_list {
                let page = albumlist.selected_page;
                let next_page = if page < 1 { 0 } else { page - 1 } as i32;
                app.get_top_albums(limit, next_page)
            };
        }
        _ => {}
    }
}
