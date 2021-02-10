use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crossterm::event::{read, Event as TermEvent, KeyCode, KeyEvent, KeyModifiers};

pub enum Event<I> {
    Input(I),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<KeyEvent>>,
    // input_handle: thread::JoinHandle<()>,
    // tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: KeyEvent,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let _input_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                if let Ok(k) = read() {
                    if let TermEvent::Key(ke) = k {
                        if let Err(_) = tx.send(Event::Input(ke)) {
                            return;
                        }
                        if ke == config.exit_key {
                            return;
                        }
                    }
                }
            })
        };
        let _tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let tx = tx.clone();
                loop {
                    tx.send(Event::Tick).unwrap();
                    thread::sleep(config.tick_rate);
                }
            })
        };
        Events { rx }
    }

    pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}
