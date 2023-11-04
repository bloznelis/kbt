mod generic_backend;
mod key;
mod keyboard100;
mod keyboard60;
mod keyboard80;
mod menu;
mod model;
mod view;

use std::{
    collections::HashMap,
    io,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use generic_backend::GenericKeyBackend;
use model::*;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    Terminal,
};
use view::show_to_small_dialog;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {}

pub const KEY_HEIGHT: u16 = 3;

fn main() -> Result<(), KbtError> {
    // simple_logging::log_to_file("kbt.log", LevelFilter::Info)?;
    log::info!("start the app!");
    let _ = Args::parse();

    run().map(|_| println!("bye!"))
}

fn run() -> Result<(), KbtError> {
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let menu_result: MenuResult = menu::run_menu(&mut terminal)?;

    match menu_result {
        MenuResult::Terminate => Ok(()),
        MenuResult::KeyboardSelected(selection) => {
            let (sender, receiver): (Sender<AppEvent>, Receiver<AppEvent>) = channel();
            let (_up_guard, _down_guard) = GenericKeyBackend::subscribe(&sender);
            let handle = thread::spawn(move || listen_for_control(sender));

            let initial_app = App {
                key_states: HashMap::new(),
                event_receiver: receiver,
                keyboard_size: selection,
                layouts: KeyboardLayouts {
                    layout_60: prepare_layout(keyboard60::ROWS.map(|rows| rows.to_vec()).to_vec()),
                    layout_80: prepare_layout(keyboard80::ROWS.map(|rows| rows.to_vec()).to_vec()),
                    layout_100: prepare_layout(
                        keyboard100::ROWS.map(|rows| rows.to_vec()).to_vec(),
                    ),
                },
            };

            let res = run_keyboard(&mut terminal, initial_app);

            match handle.join() {
                Ok(_) => res,
                Err(_) => Err(KbtError {
                    message: String::from("Control listener thread failed to exit"),
                }),
            }
        }
    }?;

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn prepare_layout(rows: Vec<Vec<KeyUI>>) -> KeyboardLayout {
    let rows_count: u16 = u16::try_from(rows.len()).unwrap_or(0);
    let height: u16 = rows_count * KEY_HEIGHT;
    let width: u16 = rows
        .iter()
        .map(|row| row.iter().map(|key| key.size.static_len()).sum())
        .max()
        .unwrap_or(0);

    let rows: Vec<Row> = rows
        .iter()
        .map(|row| Row {
            width: calc_row_width(row),
            keys: row.to_vec(),
        })
        .collect();

    KeyboardLayout {
        rows,
        height,
        width,
        rows_count,
    }
}

fn calc_row_width(row_keys: &[KeyUI]) -> u16 {
    row_keys
        .iter()
        .map(|key| (key.size.static_len() as i16 + key.size_correction.unwrap_or(0)) as u16)
        .sum()
}

fn listen_for_control(sender: Sender<AppEvent>) -> Result<(), KbtError> {
    loop {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('c') | KeyCode::Char('q') => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        sender.send(AppEvent::ControlEvent(ControlEventType::Terminate))?;
                        return Ok(());
                    }
                }
                KeyCode::Char('r') => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        sender.send(AppEvent::ControlEvent(ControlEventType::Reset))?;
                    }
                }
                _ => {}
            },
            Event::Resize(_, _) => sender.send(AppEvent::ScreenResize)?,
            _ => {}
        }
    }
}

fn run_keyboard<B: Backend>(terminal: &mut Terminal<B>, mut state: App) -> Result<(), KbtError> {
    // XXX: Drop first event, which is usually release of Return, to have first draw without keys pressed
    _ = state
        .event_receiver
        .recv_timeout(Duration::from_millis(100));

    loop {
        let does_fit = check_if_fits(terminal.size()?, &state);

        match does_fit {
            SizeCheckResult::Fits => {
                terminal.draw(|f| view::draw(f, &state).expect("Failed to draw miserably"))
            }
            SizeCheckResult::TooSmall => terminal.draw(show_to_small_dialog),
        }?;

        let app_event = state.event_receiver.recv()?;
        match app_event {
            AppEvent::KeyEvent(KeyEventType::KeyPressed(key)) => {
                state.key_states.insert(key, KeyState::Pressed);
            }
            AppEvent::KeyEvent(KeyEventType::KeyReleased(key)) => {
                if let Some(KeyState::Pressed) = state.key_states.get(&key) {
                    state.key_states.insert(key, KeyState::Released);
                    {}
                }
            }
            AppEvent::ControlEvent(control) => match control {
                ControlEventType::Terminate => {
                    return Ok(());
                }
                ControlEventType::Reset => {
                    state.reset();
                }
            },
            AppEvent::ScreenResize => {}
        }
    }
}

enum SizeCheckResult {
    Fits,
    TooSmall,
}

fn check_if_fits(terminal_size: Rect, state: &App) -> SizeCheckResult {
    match state.keyboard_size {
        KeyboardSize::Keyboard60 => {
            if terminal_size.width > state.layouts.layout_60.width
                && terminal_size.height > state.layouts.layout_60.height
            {
                SizeCheckResult::Fits
            } else {
                SizeCheckResult::TooSmall
            }
        }
        KeyboardSize::Keyboard80 => {
            if terminal_size.width > state.layouts.layout_80.width
                && terminal_size.height > state.layouts.layout_80.height
            {
                SizeCheckResult::Fits
            } else {
                SizeCheckResult::TooSmall
            }
        }
        KeyboardSize::Keyboard100 => {
            if terminal_size.width > state.layouts.layout_100.width
                && terminal_size.height > state.layouts.layout_100.height
            {
                SizeCheckResult::Fits
            } else {
                SizeCheckResult::TooSmall
            }
        }
    }
}
