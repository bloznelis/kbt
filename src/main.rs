mod generic_backend;
mod key;
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
};

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode, KeyModifiers},
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

fn main() -> Result<(), KbtError> {
    // simple_logging::log_to_file("kbt.log", LevelFilter::Info)?;
    log::info!("start the app!");
    let _ = Args::parse();

    run().map(|_| println!("bye!"))
}

fn run() -> Result<(), KbtError> {
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let menu_result: MenuResult = menu::run_menu(&mut terminal)?;

    match menu_result {
        MenuResult::Terminate => Ok(()),
        MenuResult::KeyboardSelected(selection) => {
            let (sender, receiver): (Sender<AppEvent>, Receiver<AppEvent>) = channel();
            let (_up_guard, _down_guard) = GenericKeyBackend::subscribe(&sender);
            thread::spawn(move || listen_for_control(sender).unwrap());

            let initial_app = App {
                key_states: HashMap::new(),
                event_receiver: receiver,
                keyboard_size: selection,
            };

            run_keyboard(&mut terminal, initial_app)
        }
    }?;

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn listen_for_control(sender: Sender<AppEvent>) -> Result<(), KbtError> {
    loop {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('c') | KeyCode::Char('q') => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        sender.send(AppEvent::ControlEvent(ControlEventType::Terminate))?;
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
    loop {
        let does_fit = check_if_fits(terminal.size()?, &state);

        match does_fit {
            SizeCheckResult::Fits => terminal.draw(|f| view::draw(f, &state)),
            SizeCheckResult::TooSmall => terminal.draw(|f| show_to_small_dialog(f)),
        }?;

        let app_event = state.event_receiver.recv().unwrap();
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
            if terminal_size.width > 80 && terminal_size.height > 22 {
                SizeCheckResult::Fits
            } else {
                SizeCheckResult::TooSmall
            }
        }
        KeyboardSize::Keyboard80 => {
            if terminal_size.width > 93 && terminal_size.height > 24 {
                SizeCheckResult::Fits
            } else {
                SizeCheckResult::TooSmall
            }
        }
    }
}
