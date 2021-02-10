mod album_tracks;
mod albumlist;
mod artist;
mod artistlist;
mod common_events;
mod djprogram;
mod djradio;
mod empty;
mod fm;
mod home;
mod my_playlist;
mod playlist;
mod recommend;
mod search;
mod search_results;
mod track;

use super::app::{Action, ActiveBlock, App, RouteId};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, PartialEq, Debug)]
pub enum TrackState {
    Forword,
    Backword,
}

pub fn handle_app(key: KeyEvent, app: &mut App) {
    // get current route
    let current_route = app.get_current_route();
    match current_route.active_block {
        ActiveBlock::Search => match key {
            KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                app.hover_mode();
            }
            _ => {
                search::handler(key, app);
            }
        },
        _ => match key.modifiers {
            // means space
            KeyModifiers::CONTROL => match key.code {
                KeyCode::Char('y') => {
                    app.like_current(Action::Subscribe);
                }
                KeyCode::Char('d') => {
                    app.like_current(Action::Unsubscribe);
                }
                _ => handle_block_events(key, app),
            },
            _ => match key.code {
                KeyCode::Char(' ') => {
                    if app.player.is_playing() {
                        app.player.pause();
                    } else {
                        app.player.play();
                    }
                }
                KeyCode::Char('-') => {
                    app.player.decrease_volume();
                }
                KeyCode::Char('+') => {
                    app.player.increase_volume();
                }
                KeyCode::Char('n') => {
                    app.skip_track(TrackState::Forword);
                }
                KeyCode::Char('p') => {
                    app.skip_track(TrackState::Backword);
                }
                KeyCode::Char('/') => {
                    app.set_current_route_state(
                        Some(ActiveBlock::Search),
                        Some(ActiveBlock::Search),
                    );
                }
                KeyCode::Char('r') => {
                    app.repeat();
                }
                KeyCode::Char('?') => {
                    app.set_current_route_state(Some(ActiveBlock::Help), None);
                }
                KeyCode::Char('f') => {
                    let current_route = app.get_current_route();
                    match current_route.id {
                        RouteId::Playing => {}
                        _ => {
                            app.push_navigation_stack(RouteId::Playing, ActiveBlock::Playing);
                        }
                    }
                }
                KeyCode::Char('>') => {
                    app.player.seek_forwards();
                }
                KeyCode::Char('<') => {
                    app.player.seek_backwards();
                }
                KeyCode::Esc => {
                    app.hover_mode();
                }
                KeyCode::Char('a') => {
                    let album_id = match &app.current_playing {
                        Some(track) => track.album.to_owned().unwrap().id,
                        None => None,
                    };
                    app.get_album_tracks(album_id.unwrap().to_string());
                }
                _ => handle_block_events(key, app),
            },
        },
    }
}

// handle current block events
fn handle_block_events(key: KeyEvent, app: &mut App) {
    // get current route
    let current_route = app.get_current_route();

    match current_route.active_block {
        ActiveBlock::MyPlaylists => {
            my_playlist::handler(key, app);
        }
        ActiveBlock::TrackTable => {
            track::handler(key, app);
        }
        ActiveBlock::Recommend => {
            recommend::handler(key, app);
        }
        ActiveBlock::Empty => {
            empty::handler(key, app);
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
        ActiveBlock::Search => {
            search::handler(key, app);
        }
        ActiveBlock::Artist => {
            artist::handler(key, app);
        }
        ActiveBlock::AlbumTracks => {
            album_tracks::handler(key, app);
        }
        ActiveBlock::Playlist => {
            playlist::handler(key, app);
        }
        ActiveBlock::AlbumList => {
            albumlist::handler(key, app);
        }
        ActiveBlock::ArtistList => {
            artistlist::handler(key, app);
        }
        ActiveBlock::SearchResult => {
            search_results::handler(key, app);
        }
        ActiveBlock::PersonalFm => {
            fm::handler(key, app);
        }
        ActiveBlock::DjRadio => {
            djradio::handler(key, app);
        }
        ActiveBlock::DjProgram => {
            djprogram::handler(key, app);
        }
        _ => {}
    }
}
