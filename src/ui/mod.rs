pub mod circle;
mod util;

use super::app::{ActiveBlock, App, RepeatState, RouteId, RECOMMEND_OPTIONS};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{canvas::Canvas, Block, Borders, Gauge, Paragraph, Row, Table, Tabs};
use tui::widgets::{List, ListItem, ListState, Wrap};
use tui::Frame;
use util::{
    create_artist_string, create_datetime_string, create_tag_string, display_track_progress,
    get_color, get_percentage_width,
};

// table item for render
#[derive(Clone, Debug)]
pub struct TableItem {
    id: String,
    format: Vec<String>,
}

pub struct TableHeader<'a> {
    text: &'a str,
    width: u16,
}

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(6),
            ]
            .as_ref(),
        )
        .margin(2)
        .split(f.size());

    // Search input and help
    draw_input_and_help_box(f, app, parent_layout[0]);

    // Nested main block with potential routes
    draw_routes(f, app, parent_layout[1]);

    // Currently playing
    draw_playing_block(f, app, parent_layout[2]);

    app.block_height = parent_layout[1].height as usize
}

pub fn draw_input_and_help_box<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout_chunk);

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::Search,
        current_route.hovered_block == ActiveBlock::Search,
    );

    let input: String = app.input.iter().collect();

    f.render_widget(
        Paragraph::new(input)
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Spans::from(Span::styled(
                        "Search",
                        get_color(highlight_state),
                    )))
                    .border_style(get_color(highlight_state)),
            ),
        chunks[0],
    );

    let block = Block::default()
        .title(Spans::from(Span::styled(
            "Help",
            Style::default().fg(Color::Gray),
        )))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray));

    f.render_widget(
        Paragraph::new("Type ?")
            .block(block)
            .style(Style::default().fg(Color::Gray)),
        chunks[1],
    );
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    draw_user_block(f, app, chunks[0]);

    let current_route = app.get_current_route();
    // info!("{:?}", current_route);

    match current_route.id {
        RouteId::Error => {
            draw_error_screen(f, app, chunks[1]);
        } // This is handled as a "full screen" route in main.rs
        RouteId::TrackTable => {
            draw_track_table(f, &app, chunks[1]);
        }
        RouteId::Search => {
            draw_search_results(f, app, chunks[1]);
        }
        RouteId::Home => {
            if app.track_table.tracks.len() > 0 {
                draw_track_table(f, &app, chunks[1]);
            } else {
                draw_home(f, app, chunks[1]);
            }
        }
        RouteId::PersonalFm => {
            draw_personal_fm(f, &app, chunks[1]);
        }
        RouteId::Artist => {
            draw_artist_albums(f, app, chunks[1]);
        }
        RouteId::AlbumTracks => {
            // artist's album list
            draw_album_table(f, app, chunks[1]);
        }
        RouteId::AlbumList => {
            // album list
            draw_album_list(f, app, chunks[1]);
        }
        RouteId::ArtistList => {
            // album list
            draw_artist_list(f, app, chunks[1]);
        }
        RouteId::Playlist => {
            draw_playlist_table(f, app, chunks[1]);
        }
        RouteId::DjRadio => {
            draw_djradio_list(f, app, chunks[1]);
        }
        RouteId::DjProgram => {
            draw_dj_program_list(f, app, chunks[1]);
        }
        RouteId::Playing => {
            draw_playing_detail(f, app, chunks[1]);
        }
        RouteId::MyPlaylists => {
            // check track length for show
            if app.track_table.tracks.len() > 0 {
                draw_track_table(f, &app, chunks[1]);
            } else {
                draw_home(f, app, chunks[1]);
            }
        }
    };
}

// draw track playing block in the bottom
pub fn draw_playing_block<B>(f: &mut Frame<B>, app: &mut App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .margin(1)
        .split(layout_chunk);

    let state_title = if app.player.is_playing() {
        "Playing"
    } else {
        "Pause "
    };

    let repeat_text = match app.fm_state {
        true => "FM",
        false => match app.repeat_state {
            RepeatState::Off => "Off",
            RepeatState::Track => "Track",
            RepeatState::All => "All",
            RepeatState::Shuffle => "Shuffle",
        },
    };

    let title = format!("{} | Repeat: {}", state_title, repeat_text);

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::PlayBar,
        current_route.hovered_block == ActiveBlock::PlayBar,
    );

    let (track_name, artist_name) = match &app.current_playing {
        Some(track) => (
            track.name.to_owned().unwrap(),
            match &track.artists {
                Some(artists) => create_artist_string(&artists),
                None => "Unknown".to_string(),
            },
        ),
        None => (String::new(), String::new()),
    };

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(Spans::from(Span::styled(title, get_color(highlight_state))))
            .border_style(get_color(highlight_state)),
        layout_chunk,
    );

    f.render_widget(
        Paragraph::new(Span::styled(artist_name, Style::default().fg(Color::White)))
            .style(Style::default().fg(Color::White))
            .block(
                Block::default().title(Spans::from(Span::styled(
                    track_name,
                    Style::default()
                        .fg(Color::LightCyan)
                        .add_modifier(Modifier::BOLD),
                ))),
            ),
        chunks[0],
    );

    let (perc, label) = match app.duration_ms {
        Some(duration_ms) => (
            (app.song_progress_ms as f64 / duration_ms as f64) * 100_f64,
            display_track_progress(app.song_progress_ms, duration_ms),
        ),
        None => (0.0_f64, " ".to_string()),
    };
    f.render_widget(
        Gauge::default()
            .block(Block::default().title(""))
            .style(
                Style::default()
                    .fg(Color::LightCyan)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC | Modifier::BOLD),
            )
            .percent(perc as u16)
            .label(label),
        chunks[1],
    );
}

pub fn draw_user_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(layout_chunk);

    draw_recommend_block(f, app, chunks[0]);
    draw_playlist_block(f, app, chunks[1]);
}

pub fn draw_recommend_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Recommend,
        current_route.hovered_block == ActiveBlock::Recommend,
    );
    draw_selectable_list(
        f,
        layout_chunk,
        "Recommend",
        &RECOMMEND_OPTIONS,
        highlight_state,
        Some(app.recommend.selected_index),
    );
}

pub fn draw_playlist_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let playlist_items = match &app.playlists {
        Some(p) => p
            .iter()
            .map(|item| item.name.as_ref().unwrap().to_owned())
            .collect(),
        None => vec![],
    };

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::MyPlaylists,
        current_route.hovered_block == ActiveBlock::MyPlaylists,
    );

    draw_selectable_list(
        f,
        layout_chunk,
        "Playlists",
        &playlist_items,
        highlight_state,
        app.selected_playlist_index,
    );
}

// draw selectable list
fn draw_selectable_list<B, S>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    title: &str,
    items: &[S],
    highlight_state: (bool, bool),
    selected_index: Option<usize>,
) where
    B: Backend,
    S: std::convert::AsRef<str>,
{
    let list =
        List::new::<Vec<ListItem>>(items.iter().map(|v| ListItem::new(v.as_ref())).collect())
            .block(
                Block::default()
                    .title(Spans::from(Span::styled(title, get_color(highlight_state))))
                    .borders(Borders::ALL)
                    .border_style(get_color(highlight_state)),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(get_color(highlight_state).add_modifier(Modifier::BOLD));
    let mut state = ListState::default();
    state.select(selected_index);
    f.render_stateful_widget(list, layout_chunk, &mut state)
}

// draw track table
pub fn draw_track_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let header = [
        TableHeader {
            text: "ID",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "Title",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Artist",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Album",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
    ];

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::TrackTable,
        current_route.hovered_block == ActiveBlock::TrackTable,
    );

    let mut num = 0;
    let items = app
        .track_table
        .tracks
        .iter()
        .map(|item| {
            num += 1;
            TableItem {
                id: item.id.as_ref().unwrap().to_string(),
                format: vec![
                    num.to_string(),
                    item.name.as_ref().unwrap().to_string(),
                    create_artist_string(&item.artists.to_owned().unwrap()),
                    item.album.to_owned().unwrap().name.unwrap(),
                ],
            }
        })
        .collect::<Vec<TableItem>>();

    // draw track table by draw_table function
    draw_table(
        f,
        app,
        layout_chunk,
        (&app.track_table.name, &header),
        &items,
        app.track_table.selected_index,
        highlight_state,
    )
}

fn draw_table<B>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
    table_layout: (&str, &[TableHeader]), // (title, header colums)
    items: &[TableItem], // The nested vector must have the same length as the `header_columns`
    selected_index: usize,
    highlight_state: (bool, bool),
) where
    B: Backend,
{
    let selected_style = get_color(highlight_state).add_modifier(Modifier::BOLD);

    // caculate index and row
    let interval = (layout_chunk.height / 2) as usize;
    let (row_items, margin) = if !items.is_empty() {
        let count = (layout_chunk.height - 4) as usize;
        let total = items.len();
        if selected_index >= count - interval && total > count {
            if selected_index >= total - interval {
                let margin = total - count;
                (&items[margin..], margin)
            } else {
                let margin = selected_index + interval - count;
                (&items[margin..], margin)
            }
        } else {
            (items, 0)
        }
    } else {
        (items, 0)
    };

    let rows = row_items.iter().enumerate().map(|(i, item)| {
        // Show this ♥ if the song is liked
        let mut formatted_row = item.format.clone();
        let mut style = Style::default().fg(Color::White); // default styling

        // First check if the song should be highlighted because it is currently playing
        match &app.current_playing {
            Some(track) => {
                if item.id == track.id.unwrap().to_string() {
                    formatted_row[0] = format!("|> {}", &formatted_row[0]);
                    style = Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD);
                }
            }
            None => {}
        }

        // Next check if the item is under selection
        if i == selected_index - margin {
            style = selected_style;
        }

        // Return row styled data
        Row::new(formatted_row).style(style)
    });

    let (title, header_columns) = table_layout;

    let widths: Vec<Constraint> = header_columns
        .iter()
        .map(|h| Constraint::Length(h.width))
        .collect();
    f.render_widget(
        Table::new(rows)
            .header(Row::new(header_columns.iter().map(|h| h.text)))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title(Spans::from(Span::styled(title, get_color(highlight_state))))
                    .border_style(get_color(highlight_state)),
            )
            .style(Style::default().fg(Color::White))
            .widths(&widths),
        layout_chunk,
    );
}

fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .margin(2)
        .split(layout_chunk);

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Home,
        current_route.hovered_block == ActiveBlock::Home,
    );

    f.render_widget(
        Block::default()
            .title(Spans::from(Span::styled("Welcome!", get_color(highlight_state))))
            .borders(Borders::ALL)
            .border_style(get_color(highlight_state)),
        layout_chunk,
    );

    let top_text = Text::styled(
        "
                 __
  ____    ____ _/  |_   ____  _____     ______  ____
 /    \\ _/ __ \\\\   __\\_/ __ \\ \\__  \\   /  ___/_/ __ \\ 
|   |  \\\\  ___/ |  |  \\  ___/  / __ \\_ \\___ \\ \\  ___/
|___|  / \\___  >|__|   \\___  >(____  //____  > \\___  >
     \\/      \\/            \\/      \\/      \\/      \\/
        .__                      .___                        .__
  ____  |  |    ____   __ __   __| _/   _____   __ __  ______|__|  ____
_/ ___\\ |  |   /  _ \\ |  |  \\ / __ |   /     \\ |  |  \\/  ___/|  |_/ ___\\ 
\\  \\___ |  |__(  <_> )|  |  // /_/ |  |  Y Y  \\|  |  /\\___ \\ |  |\\  \\___
 \\___  >|____/ \\____/ |____/ \\____ |  |__|_|  /|____//____  >|__| \\___  >
     \\/                           \\/        \\/            \\/          \\/
                        __              __          .__
_______  __ __  _______/  |_          _/  |_  __ __ |__|
\\_  __ \\|  |  \\/  ___/\\   __\\  ______ \\   __\\|  |  \\|  |
 |  | \\/|  |  /\\___ \\  |  |   /_____/  |  |  |  |  /|  |
 |__|   |____//____  > |__|            |__|  |____/ |__|
                   \\/
            ",
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD),
    );

    // Contains the banner
    f.render_widget(
        Paragraph::new(top_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White))
            .block(Block::default()),
        chunks[0],
    );

    // Canvas::default()
    // .block(
    // Block::default()
    // .borders(Borders::ALL)
    // .title("Welcome!")
    // )
    // .paint(|ctx| {
    // ctx.draw(&Circle::default());
    // })
    // .x_bounds([-180.0, 180.0])
    // .y_bounds([-90.0, 90.0])
    // .render(f, layout_chunk);
}

fn draw_personal_fm<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::PersonalFm,
        current_route.hovered_block == ActiveBlock::PersonalFm,
    );
    let display_block = Block::default()
        .title(Spans::from(Span::styled("PERSONAL FM", get_color(highlight_state))))
        .borders(Borders::ALL)
        .border_style(get_color(highlight_state));

    f.render_widget(
        Paragraph::new("Your Personal FM")
            .style(Style::default().fg(Color::White))
            .block(display_block)
            .wrap(Wrap { trim: true }),
        layout_chunk,
    );
}

pub fn draw_search_results<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::SearchResult,
        current_route.hovered_block == ActiveBlock::SearchResult,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(layout_chunk);

    {
        let songs = match &app.search_results.tracks {
            Some(r) => r
                .iter()
                .map(|item| {
                    format!(
                        "{} - {}",
                        item.name.to_owned().unwrap(),
                        create_artist_string(&item.artists.to_owned().unwrap())
                    )
                })
                .collect(),
            None => vec![],
        };
        let playlists = match &app.search_results.playlists {
            Some(r) => r
                .iter()
                .map(|item| {
                    format!(
                        "{} - {}",
                        item.name.to_owned().unwrap(),
                        item.creator.to_owned().unwrap().nickname.unwrap()
                    )
                })
                .collect(),
            None => vec![],
        };
        let artists = match &app.search_results.artists {
            Some(r) => r.iter().map(|item| item.to_owned().name).collect(),
            None => vec![],
        };
        let albums = match &app.search_results.albums {
            Some(r) => r
                .iter()
                .map(|item| {
                    format!(
                        "{} - {}",
                        item.name.to_owned().unwrap(),
                        create_artist_string(&[item.artist.to_owned().unwrap()])
                    )
                })
                .collect(),
            None => vec![],
        };
        let djradios = match &app.search_results.djradios {
            Some(r) => r
                .iter()
                .map(|item| format!("{}", item.name.to_owned()))
                .collect(),
            None => vec![],
        };

        f.render_widget(
            Tabs::new(
                app.tabs
                    .titles
                    .iter()
                    .map(|v| Spans::from(v.as_str()))
                    .collect(),
            )
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Spans::from(Span::styled("Search Result", get_color(highlight_state))))
                    .border_style(get_color(highlight_state)),
            )
            .select(app.tabs.index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow)),
            chunks[0],
        );

        match app.tabs.index {
            0 => draw_selectable_list(
                f,
                chunks[1],
                "Songs",
                &songs,
                highlight_state,
                Some(app.search_results.selected_tracks_index),
            ),
            1 => draw_selectable_list(
                f,
                chunks[1],
                "Artists",
                &artists,
                highlight_state,
                Some(app.search_results.selected_artists_index),
            ),
            2 => draw_selectable_list(
                f,
                chunks[1],
                "Albums",
                &albums,
                highlight_state,
                Some(app.search_results.selected_albums_index),
            ),
            3 => draw_selectable_list(
                f,
                chunks[1],
                "Playlists",
                &playlists,
                highlight_state,
                Some(app.search_results.selected_playlists_index),
            ),
            4 => draw_selectable_list(
                f,
                chunks[1],
                "djradio",
                &djradios,
                highlight_state,
                Some(app.search_results.selected_djradio_index),
            ),

            _ => {}
        }
    }
}

pub fn draw_help_menu<B>(f: &mut Frame<B>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(f.size());

    let white = Style::default().fg(Color::White);
    let gray = Style::default().fg(Color::White);
    let header = vec!["Description", "Event", "Context"];

    let help_doc: Vec<[&str; 3]> = vec![
        ["Increase volume", "+", "General"],
        ["Decrease volume", "-", "General"],
        ["Skip to next track", "n", "General"],
        ["Skip to previous track", "p", "General"],
        ["Toggle repeat mode", "r", "General"],
        ["Move selection left", "h | <Left Arrow Key> ", "General"],
        ["Move selection down", "j | <Down Arrow Key> ", "General"],
        ["Move selection up", "k | <Up Arrow Key> ", "General"],
        ["Move selection right", "l | <Right Arrow Key> ", "General"],
        ["Jump to currently playing album", "a", "General"],
        ["Enter Search", "/", "General"],
        ["Pause/Resume playback", "<Space>", "General"],
        ["Fullsize playbar", "f", "General"],
        [
            "Go back or exit when nowhere left to back to",
            "q",
            "General",
        ],
        ["Enter hover mode", "<Esc>", "General"],
        ["Enter active mode", "<Enter>", "General"],
        ["Like current playing track", "<Ctrl+y>", "General"],
        ["Dislike current playing track", "<Ctrl+d>", "General"],
        ["move track to trash", "<Ctrl+t>", "FM block"],
        ["Delete entire input", "<Ctrl+u>", "Search input"],
        ["Search with input text", "<Enter>", "Search input"],
        ["Jump to start of input", "<Ctrl+a>", "Search input"],
        ["Jump to end of input", "<Ctrl+e>", "Search input"],
        [
            "Subscribe current hover playlist",
            "<Alt+s>",
            "Playlist block",
        ],
        [
            "Unsubscribe current hover playlist",
            "<Alt+d>",
            "Playlist block",
        ],
        ["Jump to next page", "<Ctrl+f>", "Search result | top list"],
        [
            "Jump to previous page",
            "<Ctrl+b>",
            "Search result | top list",
        ],
    ];

    let rows = help_doc
        .iter()
        .map(|item: &[&str; 3]| Row::new(vec![Spans::from(item[0]), Spans::from(item[1]), Spans::from(item[2])]).style(gray));

    f.render_widget(
        Table::new(rows)
            .header(Row::new(header))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(white)
                    .title(Spans::from(Span::styled("Help (press <Esc> to go back)", gray)))
                    .border_style(gray),
            )
            .style(Style::default().fg(Color::White))
            .widths(&[
                Constraint::Length(60),
                Constraint::Length(30),
                Constraint::Length(20),
            ]),
        chunks[0],
    );
}

pub fn draw_playing_detail<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Playing,
        current_route.hovered_block == ActiveBlock::Playing,
    );

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        // .margin(2)
        .split(layout_chunk);

    f.render_widget(
        Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                    .title(Spans::from(Span::styled("Playing", get_color(highlight_state))))
                    .border_style(get_color(highlight_state)),
            )
            .x_bounds([-90.0, 90.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                ctx.draw(&app.playing_circle);
            }),
        chunks[0],
    );

    let lyric_items = match &app.lyric {
        Some(l) => l.iter().map(|item| vec![item.value.to_owned()]).collect(),
        None => vec![],
    };
    let selected_index = app.lyric_index;

    let interval = (layout_chunk.height / 2) as usize;
    let (row_items, margin) = if !lyric_items.is_empty() {
        let count = (layout_chunk.height - 4) as usize;
        let total = lyric_items.len();
        if selected_index >= count - interval && total > count {
            if selected_index >= total - interval {
                let margin = total - count;
                (&lyric_items[margin..], margin)
            } else {
                let margin = selected_index + interval - count;
                (&lyric_items[margin..], margin)
            }
        } else {
            (lyric_items.as_ref(), 0 as usize)
        }
    } else {
        (lyric_items.as_ref(), 0 as usize)
    };

    let header = [TableHeader {
        text: "",
        width: get_percentage_width(layout_chunk.width, 0.5),
    }];

    let selected_style = get_color(highlight_state).add_modifier(Modifier::BOLD);
    let rows = row_items.iter().enumerate().map(|(i, item)| {
        let mut style = Style::default().fg(Color::White); // default styling
        if i == selected_index - margin {
            style = selected_style;
        }
        // Return row styled data
        Row::new(item.clone()).style(style)
    });

    let widths: Vec<Constraint> = header.iter().map(|h| Constraint::Length(h.width)).collect();

    f.render_widget(
        Table::new(rows)
            .header(Row::new(header.iter().map(|h| h.text)))
            .block(
                Block::default()
                    .title(Spans::from(Span::styled("", get_color(highlight_state))))
                    .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
                    .style(Style::default().fg(Color::White))
                    .border_style(get_color(highlight_state)),
            )
            .style(Style::default().fg(Color::White))
            .widths(&widths),
        chunks[1],
    );
}

// list ui struct
struct ListUI {
    selected_index: usize,
    items: Vec<TableItem>,
    title: String,
}

pub fn draw_artist_albums<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Artist,
        current_route.hovered_block == ActiveBlock::Artist,
    );

    let header = [
        TableHeader {
            text: "",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "Album Name",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Artist",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
    ];

    let mut num = 0;
    let album_ui = match &app.artist_albums {
        Some(album_list) => Some(ListUI {
            items: album_list
                .albums
                .iter()
                .map(|item| {
                    num += 1;
                    TableItem {
                        id: item.id.clone().unwrap_or_else(|| 0).to_string(),
                        format: vec![
                            num.to_string(),
                            item.to_owned().name.unwrap().to_string(),
                            item.artist.to_owned().unwrap().name.to_string(),
                        ],
                    }
                })
                .collect::<Vec<TableItem>>(),
            title: format!("Albums",),
            selected_index: album_list.selected_index,
        }),
        None => None,
    };

    if let Some(album_ui) = album_ui {
        draw_table(
            f,
            &app,
            layout_chunk,
            (&album_ui.title, &header),
            &album_ui.items,
            album_ui.selected_index,
            highlight_state,
        );
    };
}

// dtaw album table
pub fn draw_album_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::AlbumTracks,
        current_route.hovered_block == ActiveBlock::AlbumTracks,
    );

    let header = [
        TableHeader {
            text: "",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "Title",
            width: get_percentage_width(layout_chunk.width, 0.8),
        },
    ];

    let mut num = 0;
    let album_ui = match &app.selected_album {
        Some(selected_album) => Some(ListUI {
            items: selected_album
                .tracks
                .iter()
                .map(|item| {
                    num += 1;
                    TableItem {
                        id: item.id.clone().unwrap_or_else(|| 0).to_string(),
                        format: vec![num.to_string(), item.to_owned().name.unwrap().to_string()],
                    }
                })
                .collect::<Vec<TableItem>>(),
            title: format!(
                "{} by {}",
                selected_album.to_owned().album.name.unwrap(),
                create_artist_string(&[selected_album.to_owned().album.artist.unwrap()])
            ),
            selected_index: selected_album.selected_index,
        }),
        None => None,
    };

    if let Some(album_ui) = album_ui {
        draw_table(
            f,
            &app,
            layout_chunk,
            (&album_ui.title, &header),
            &album_ui.items,
            album_ui.selected_index,
            highlight_state,
        );
    };
}

pub fn draw_playlist_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Playlist,
        current_route.hovered_block == ActiveBlock::Playlist,
    );

    let header = [
        TableHeader {
            text: "",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "Title",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Count",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "Creator",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Tags",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
    ];

    let mut num = match app.playlist_list.to_owned() {
        Some(playlist) => playlist.selected_page * (app.block_height - 4),
        None => 0,
    };
    let playlist_ui = match &app.playlist_list {
        Some(playlist) => Some(ListUI {
            items: playlist
                .playlists
                .iter()
                .map(|item| {
                    num += 1;
                    TableItem {
                        id: item.id.clone().unwrap_or_else(|| 0).to_string(),
                        format: vec![
                            num.to_string(),
                            item.to_owned().name.unwrap().to_string(),
                            item.trackCount.unwrap().to_string(),
                            item.creator
                                .to_owned()
                                .unwrap()
                                .nickname
                                .unwrap()
                                .to_string(),
                            create_tag_string(&item.tags.to_owned().unwrap()),
                        ],
                    }
                })
                .collect::<Vec<TableItem>>(),
            title: format!("Discover Playlists",),
            selected_index: playlist.selected_index,
        }),
        None => None,
    };

    if let Some(playlist_ui) = playlist_ui {
        draw_table(
            f,
            &app,
            layout_chunk,
            (&playlist_ui.title, &header),
            &playlist_ui.items,
            playlist_ui.selected_index,
            highlight_state,
        );
    };
}

pub fn draw_album_list<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::AlbumList,
        current_route.hovered_block == ActiveBlock::AlbumList,
    );

    let header = [
        TableHeader {
            text: "",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "Album Name",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Artist",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
    ];

    let mut num = match app.album_list.to_owned() {
        Some(albumlist) => albumlist.selected_page * (app.block_height - 4),
        None => 0,
    };

    let album_ui = match &app.album_list {
        Some(album_list) => Some(ListUI {
            items: album_list
                .albums
                .iter()
                .map(|item| {
                    num += 1;
                    TableItem {
                        id: item.id.clone().unwrap_or_else(|| 0).to_string(),
                        format: vec![
                            num.to_string(),
                            item.to_owned().name.unwrap().to_string(),
                            item.artist.to_owned().unwrap().name.to_string(),
                        ],
                    }
                })
                .collect::<Vec<TableItem>>(),
            title: format!("Discover Albums",),
            selected_index: album_list.selected_index,
        }),
        None => None,
    };

    if let Some(album_ui) = album_ui {
        draw_table(
            f,
            &app,
            layout_chunk,
            (&album_ui.title, &header),
            &album_ui.items,
            album_ui.selected_index,
            highlight_state,
        );
    };
}

pub fn draw_artist_list<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::ArtistList,
        current_route.hovered_block == ActiveBlock::ArtistList,
    );

    let header = [
        TableHeader {
            text: "",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "Artist",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
    ];

    let mut num = match app.artist_list.to_owned() {
        Some(artistlist) => artistlist.selected_page * (app.block_height - 4),
        None => 0,
    };

    let artist_ui = match &app.artist_list {
        Some(artist_list) => Some(ListUI {
            items: artist_list
                .artists
                .iter()
                .map(|item| {
                    num += 1;
                    TableItem {
                        id: item.id.to_string(),
                        format: vec![num.to_string(), item.to_owned().name],
                    }
                })
                .collect::<Vec<TableItem>>(),
            title: format!("Discover Artists",),
            selected_index: artist_list.selected_index,
        }),
        None => None,
    };

    if let Some(artist_ui) = artist_ui {
        draw_table(
            f,
            &app,
            layout_chunk,
            (&artist_ui.title, &header),
            &artist_ui.items,
            artist_ui.selected_index,
            highlight_state,
        );
    };
}

pub fn draw_djradio_list<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::DjRadio,
        current_route.hovered_block == ActiveBlock::DjRadio,
    );

    let header = [
        TableHeader {
            text: "",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "DjRadio Name",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
    ];

    let mut num = match app.djradio_list.to_owned() {
        Some(djradio_list) => djradio_list.selected_page * (app.block_height - 4),
        None => 0,
    };

    let djradio_ui = match &app.djradio_list {
        Some(djradio_list) => Some(ListUI {
            items: djradio_list
                .djradios
                .iter()
                .map(|item| {
                    num += 1;
                    TableItem {
                        id: item.id.to_string(),
                        format: vec![num.to_string(), item.to_owned().name],
                    }
                })
                .collect::<Vec<TableItem>>(),
            title: format!("My Subscribe DjRadio",),
            selected_index: djradio_list.selected_index,
        }),
        None => None,
    };

    if let Some(artist_ui) = djradio_ui {
        draw_table(
            f,
            &app,
            layout_chunk,
            (&artist_ui.title, &header),
            &artist_ui.items,
            artist_ui.selected_index,
            highlight_state,
        );
    };
}

pub fn draw_dj_program_list<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::DjProgram,
        current_route.hovered_block == ActiveBlock::DjProgram,
    );

    let header = [
        TableHeader {
            text: "",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "Dj Program Name",
            width: get_percentage_width(layout_chunk.width, 0.5),
        },
        TableHeader {
            text: "listener Count",
            width: get_percentage_width(layout_chunk.width, 0.2),
        },
        TableHeader {
            text: "Date",
            width: get_percentage_width(layout_chunk.width, 0.2),
        },
    ];

    let program_ui = match &app.program_list {
        Some(program_list) => Some(ListUI {
            items: program_list
                .dj_programs
                .iter()
                .map(|item| {
                    let num_string = match &app.current_playing {
                        Some(track) => {
                            if item.mainSong.id.to_string() == track.id.unwrap().to_string() {
                                format!("|> {}", item.serialNum.to_string())
                            } else {
                                item.serialNum.to_string()
                            }
                        }
                        None => item.serialNum.to_string(),
                    };
                    TableItem {
                        id: item.id.to_string(),
                        format: vec![
                            num_string,
                            item.mainSong.name.to_string(),
                            item.listenerCount.to_string(),
                            create_datetime_string(item.createTime),
                        ],
                    }
                })
                .collect::<Vec<TableItem>>(),
            title: program_list.to_owned().name,
            selected_index: program_list.selected_index,
        }),
        None => None,
    };

    if let Some(program_ui) = program_ui {
        draw_table(
            f,
            &app,
            layout_chunk,
            (&program_ui.title, &header),
            &program_ui.items,
            program_ui.selected_index,
            highlight_state,
        );
    };
}

pub fn draw_error_screen<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(layout_chunk);

    let playing_text = vec![
        Spans::from(Span::raw("Api response: ")),
        Spans::from(Span::styled(
            &app.error_msg,
            Style::default().fg(Color::LightRed),
        )),
        Spans::from(Span::styled(
            "\nPress `e` to return",
            Style::default().fg(Color::Gray),
        )),
    ];

    f.render_widget(
        Paragraph::new(playing_text)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Spans::from(Span::styled("Error Page", Style::default().fg(Color::Red))))
                    .border_style(Style::default().fg(Color::Red)),
            ),
        chunks[0],
    );
}

pub fn draw_msg<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(f.size());
    let child_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    let msg = Span::styled(&app.msg, Style::default().fg(Color::Cyan));

    f.render_widget(
        Paragraph::new(msg)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            ),
        child_chunks[1],
    );
}
