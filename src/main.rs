mod backend;
mod key;
mod keyboard60;
mod keyboard80;
mod linux;
mod menu;

use std::{
    collections::HashMap,
    fmt::Display,
    io,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use backend::KeyBackend;
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use key::Key;
use linux::X11;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

pub struct KeyUI {
    key: Key,
    size: KeySize,
    size_correction: Option<i16>, // To make layout look consistent
}

pub enum KeySize {
    U05,
    U1,
    U15,
    U175,
    U2,
    U225,
    U250,
    U275,
    U675,
}

// 1 terminal cell = 0.2u

//  1u (len = 5)
// ┏━━━┓
// ┃ A ┃
// ┗━━━┛

//  2u (len = 10)
// ┏━━━━━━━━┓
// ┃  |<-   ┃
// ┗━━━━━━━━┛
impl KeySize {
    fn static_len(&self) -> i16 {
        match self {
            KeySize::U05 => 2,
            KeySize::U1 => 5,
            KeySize::U15 => 7,
            KeySize::U175 => 8,
            KeySize::U2 => 10,
            KeySize::U225 => 11,
            KeySize::U250 => 12,
            KeySize::U275 => 14,
            KeySize::U675 => 34,
        }
    }
}

fn make_row_constraints_static(keys: &[KeyUI]) -> Vec<Constraint> {
    keys.iter()
        .map(|key| {
            Constraint::Length(
                u16::try_from(key.size.static_len() + key.size_correction.unwrap_or(0))
                    .unwrap_or(0),
            )
        })
        .collect()
}

pub enum MenuResult {
    KeyboardSelected(KeyboardSize),
    Terminate,
}

enum KeyState {
    Pressed,
    Released,
    Untouched,
}

pub enum KeyEventType {
    KeyPressed(Key),
    KeyReleased(Key),
}

pub enum ControlEventType {
    Terminate,
    Reset,
}

pub enum AppEvent {
    KeyEvent(KeyEventType),
    ControlEvent(ControlEventType),
    ScreenResize,
}

#[derive(Clone)]
pub enum KeyboardSize {
    Keyboard60,
    Keyboard80,
}

impl Display for KeyboardSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardSize::Keyboard60 => write!(f, "60% layout"),
            KeyboardSize::Keyboard80 => write!(f, "80% layout"),
        }
    }
}

struct App {
    key_states: HashMap<Key, KeyState>,
    event_receiver: Receiver<AppEvent>,
    keyboard_size: KeyboardSize,
}

impl App {
    fn reset(&mut self) -> () {
        self.key_states = HashMap::new()
    }
}

#[derive(Debug)]
pub struct KbtError {
    pub message: String,
}

impl From<io::Error> for KbtError {
    fn from(value: io::Error) -> Self {
        KbtError {
            message: value.to_string(),
        }
    }
}

impl From<Box<dyn std::error::Error>> for KbtError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        KbtError {
            message: value.to_string(),
        }
    }
}

impl From<std::sync::mpsc::SendError<AppEvent>> for KbtError {
    fn from(value: std::sync::mpsc::SendError<AppEvent>) -> Self {
        KbtError {
            message: value.to_string(),
        }
    }
}

fn main() -> Result<(), KbtError> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let menu_result: MenuResult = menu::run_menu(&mut terminal)?;

    match menu_result {
        MenuResult::Terminate => return Ok(()),
        MenuResult::KeyboardSelected(selection) => {
            let (sender, receiver): (Sender<AppEvent>, Receiver<AppEvent>) = channel();
            X11.subscribe(sender.clone())?;
            thread::spawn(move || listen_for_control(sender).unwrap());

            let initial_app = App {
                key_states: HashMap::new(),
                event_receiver: receiver,
                keyboard_size: selection,
            };

            run(&mut terminal, initial_app)
        }
    }?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    println!("bye!");
    Ok(())
}

fn listen_for_control(sender: Sender<AppEvent>) -> Result<(), KbtError> {
    loop {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('c') | KeyCode::Char('q') => match key.modifiers {
                    KeyModifiers::CONTROL => {
                        sender.send(AppEvent::ControlEvent(ControlEventType::Terminate))?;
                    }
                    _ => {}
                },
                KeyCode::Char('r') => match key.modifiers {
                    KeyModifiers::CONTROL => {
                        sender.send(AppEvent::ControlEvent(ControlEventType::Reset))?;
                    }
                    _ => {}
                },
                _ => {}
            },
            Event::Resize(_, _) => sender.send(AppEvent::ScreenResize)?,
            _ => {}
        }
    }
}

fn run<B: Backend>(terminal: &mut Terminal<B>, mut state: App) -> Result<(), KbtError> {
    enable_raw_mode()?;

    loop {
        let app_event = state.event_receiver.recv().unwrap();
        match app_event {
            AppEvent::KeyEvent(KeyEventType::KeyPressed(key)) => {
                state.key_states.insert(key, KeyState::Pressed);
            }
            AppEvent::KeyEvent(KeyEventType::KeyReleased(key)) => {
                state.key_states.insert(key, KeyState::Released);
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

        let does_fit = check_if_fits(terminal.size()?, &state);

        match does_fit {
            SizeCheckResult::Fits => terminal.draw(|f| view(f, &state)),
            SizeCheckResult::TooSmall => terminal.draw(|f| show_to_small_dialog(f)),
        }?;
    }
}

enum SizeCheckResult {
    Fits,
    TooSmall,
}

fn check_if_fits(terminal_size: Rect, state: &App) -> SizeCheckResult {
    match state.keyboard_size {
        KeyboardSize::Keyboard60 => {
            if terminal_size.width > 80 && terminal_size.height > 20 {
                SizeCheckResult::Fits
            } else {
                SizeCheckResult::TooSmall
            }
        }
        KeyboardSize::Keyboard80 => {
            if terminal_size.width > 93 && terminal_size.height > 22 {
                SizeCheckResult::Fits
            } else {
                SizeCheckResult::TooSmall
            }
        }
    }
}

fn show_to_small_dialog<B: Backend>(frame: &mut Frame<B>) {
    let terminal_size = frame.size();

    let message = "window is too small :(";

    let message_height: u16 = 1;
    let message_width: u16 = 25;
    let left_padding: u16 = (terminal_size.width / 2) - (message_width / 2);
    let top_padding: u16 = (terminal_size.height / 2) - (message_height / 2);

    let rect = Rect::new(left_padding, top_padding, message_width, message_height);

    let title = Paragraph::new(message).style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::ITALIC),
    );

    frame.render_widget(title, rect);
}

fn calc_static_row_len(row_keys: &[KeyUI]) -> u16 {
    row_keys
        .iter()
        .map(|key| {
            u16::try_from(key.size.static_len() + key.size_correction.unwrap_or(0)).unwrap_or(0)
        })
        .sum()
}

fn view<B: Backend>(frame: &mut Frame<B>, state: &App) {
    match state.keyboard_size {
        KeyboardSize::Keyboard80 => draw_80(frame, state),
        KeyboardSize::Keyboard60 => draw_60(frame, state),
    }
}

fn draw_80<B: Backend>(frame: &mut Frame<B>, state: &App) {
    let terminal_size: Rect = frame.size();

    let rows = keyboard80::ROWS;
    let rows_count: u16 = 6;

    let row_height: u16 = 3;
    let layout_height: u16 = 3 * rows_count;
    let layout_width: u16 = 93;
    let left_padding: u16 = (terminal_size.width / 2) - (layout_width / 2);
    let top_padding: u16 = (terminal_size.height / 2) - (layout_height / 2);

    for (idx, row) in rows.iter().enumerate() {
        let idx: u16 = u16::try_from(idx).unwrap();
        let row_width: u16 = calc_static_row_len(row);
        let y_offset: u16 = (row_height * idx) + top_padding;
        let rect = Rect::new(left_padding, y_offset, row_width, row_height);

        draw_row(row, state, rect, frame)
    }

    if state.key_states.values().filter(|a| matches!(a, KeyState::Released | KeyState::Pressed)).count() < 5 {
        draw_help(top_padding + (row_height * rows_count) + 3, frame);
    }

}

fn draw_60<B: Backend>(frame: &mut Frame<B>, state: &App) {
    let terminal_size: Rect = frame.size();

    let rows = keyboard60::ROWS;
    let rows_count: u16 = 5;
    // 60% layout:
    // width = 75 cells
    // height = 15 cells
    let row_height: u16 = 3;
    let layout_height: u16 = 3 * rows_count;
    let layout_width: u16 = 75;
    let left_padding: u16 = (terminal_size.width / 2) - (layout_width / 2);
    let top_padding: u16 = (terminal_size.height / 2) - (layout_height / 2);

    for (idx, row) in rows.iter().enumerate() {
        let idx: u16 = u16::try_from(idx).unwrap();
        let row_width: u16 = calc_static_row_len(row);
        let y_offset: u16 = (row_height * idx) + top_padding;
        let rect = Rect::new(left_padding, y_offset, row_width, row_height);

        draw_row(row, state, rect, frame)
    }

    if state.key_states.values().filter(|a| matches!(a, KeyState::Released | KeyState::Pressed)).count() < 5 {
        draw_help(top_padding + (row_height * rows_count) + 3, frame);
    }
}

fn draw_row<B: Backend>(row_keys: &[KeyUI], state: &App, rect: Rect, frame: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(make_row_constraints_static(row_keys).as_ref())
        .split(rect);

    for (pos, ui_key) in row_keys.iter().enumerate() {
        let key_state = state
            .key_states
            .get(&ui_key.key)
            .unwrap_or(&KeyState::Untouched);

        let borders = match ui_key.key {
            Key::Separator => Borders::NONE,
            _ => Borders::ALL,
        };

        let border_type = match key_state {
            KeyState::Pressed => BorderType::Double,
            KeyState::Released => BorderType::Thick,
            KeyState::Untouched => BorderType::Plain,
        };

        let style = match key_state {
            KeyState::Pressed => Style::default().fg(Color::Yellow),
            KeyState::Released => Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
            KeyState::Untouched => Style::default(),
        };

        let block = Block::default().borders(borders).border_type(border_type);

        let text = Paragraph::new(ui_key.key.to_string())
            .block(block)
            .style(style)
            .alignment(Alignment::Center);

        frame.render_widget(text, chunks[pos])
    }
}

fn draw_help<B: Backend>(y_offset: u16, frame: &mut Frame<B>) {
    let terminal_size = frame.size();
    let message = "ctrl+r to restart, ctrl+q to quit";
    let message_len = 33;
    let message_height = 1;

    let x_offset: u16 = (terminal_size.width / 2) - (message_len / 2);

    let rect = Rect::new(x_offset, y_offset, message_len, message_height);

    let help = Paragraph::new(message).style(
        Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::DIM)
    );

    frame.render_widget(help, rect);
}
