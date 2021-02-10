use super::super::app::{ActiveBlock, App};
use super::common_events;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handler(key: KeyEvent, app: &mut App) {
    match key {
        KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        } => {
            let current_hovered = app.get_current_route().hovered_block;
            app.set_current_route_state(Some(current_hovered), None);
        }
        k if common_events::left_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::Artist
            | ActiveBlock::AlbumList
            | ActiveBlock::AlbumTracks
            | ActiveBlock::Home
            | ActiveBlock::SearchResult
            | ActiveBlock::Playlist
            | ActiveBlock::PersonalFm
            | ActiveBlock::Playing
            | ActiveBlock::DjRadio
            | ActiveBlock::DjProgram
            | ActiveBlock::TrackTable => {
                app.set_current_route_state(None, Some(ActiveBlock::Recommend));
            }
            _ => {}
        },
        k if common_events::up_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::MyPlaylists => {
                app.set_current_route_state(None, Some(ActiveBlock::Recommend));
            }
            // ActiveBlock::PlayBar => {
            // app.set_current_route_state(None, Some(ActiveBlock::MyPlaylists));
            // }
            ActiveBlock::Recommend => {
                app.set_current_route_state(None, Some(ActiveBlock::Search));
            }
            ActiveBlock::Artist
            | ActiveBlock::AlbumList
            | ActiveBlock::AlbumTracks
            | ActiveBlock::Home
            | ActiveBlock::SearchResult
            | ActiveBlock::Playlist
            | ActiveBlock::PersonalFm
            | ActiveBlock::TrackTable => {
                app.set_current_route_state(None, Some(ActiveBlock::Search));
            }
            _ => {}
        },
        k if common_events::down_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::Recommend => {
                app.set_current_route_state(None, Some(ActiveBlock::MyPlaylists));
            }
            ActiveBlock::Search => {
                app.set_current_route_state(None, Some(ActiveBlock::Recommend));
            }
            _ => {}
        },
        k if common_events::right_event(k) => common_events::handle_right_event(app),
        _ => {}
    }
}
