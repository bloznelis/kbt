mod backend;
mod key;
mod keyboard;
mod linux;

use std::{
    collections::HashMap,
    io::{self, Stdout},
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::Duration,
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

struct KeySize {
    size: f32,
}

pub struct KeyUI {
    key: Key,
    size: f32,
}

// https://support.wasdkeyboards.com/hc/en-us/articles/115009701328-Keycap-Size-Compatibility
// some shitty mafs
// 60% one row has 15u
// 1u key takes up
// 100 -> 15u
// x -> 1u
// -------
// x = 100 / 15
// x ~= 6.66666
//////////////////
// 100 -> 15
// x -> 2u
// -------
// x ~= 13.333333
fn calc_percentage(key: &KeyUI) -> u16 {
    ((100 as f32 * key.size) / 15 as f32) as u16
}

fn make_row_constraints(keys: &[KeyUI]) -> Vec<Constraint> {
    keys.iter()
        .map(|key| Constraint::Percentage(calc_percentage(&key)))
        .collect()
}

enum KeyState {
    Pressed,
    Released,
    Untouched,
}

enum KeyEventType {
    KeyPressed(Key),
    KeyReleased(Key),
}

enum ControlEventType {
    Terminate,
}

enum AppEvent {
    KeyEvent(KeyEventType),
    ControlEvent(ControlEventType),
}

struct App {
    key_states: HashMap<Key, KeyState>,
    event_receiver: Receiver<AppEvent>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let (sender, receiver): (Sender<AppEvent>, Receiver<AppEvent>) = channel();
    X11.subscribe(sender.clone())?;
    thread::spawn(move || listen_for_control(sender).unwrap());

    let initial_app = App {
        key_states: HashMap::new(),
        event_receiver: receiver,
    };

    let result = run(&mut terminal, initial_app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn listen_for_control(sender: Sender<AppEvent>) -> io::Result<()> {
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('c') => match key.modifiers {
                    KeyModifiers::CONTROL => {
                        //todo: handle error
                        sender.send(AppEvent::ControlEvent(ControlEventType::Terminate));
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn run<B: Backend>(terminal: &mut Terminal<B>, mut state: App) -> io::Result<()> {
    enable_raw_mode();

    loop {
        terminal.draw(|f| view(f, &state))?;

        let key_update = state.event_receiver.recv().unwrap();
        match key_update {
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
            },
        }
    }
}

fn view<B: Backend>(frame: &mut Frame<B>, state: &App) {
    let terminal_size: Rect = frame.size();
    let row_height = terminal_size.height / 5;

    let rect_0 = Rect::new(0, 0, terminal_size.width, row_height);
    let rect_1 = Rect::new(0, row_height, terminal_size.width, row_height);
    let rect_2 = Rect::new(0, row_height * 2, terminal_size.width, row_height);
    let rect_3 = Rect::new(0, row_height * 3, terminal_size.width, row_height);
    let rect_4 = Rect::new(0, row_height * 4, terminal_size.width, row_height);
    let rect_5 = Rect::new(0, row_height * 5, terminal_size.width, row_height);

    draw_row(&keyboard::R4_1, state, rect_0, frame);
    draw_row(&keyboard::R3, state, rect_1, frame);
    draw_row(&keyboard::R2, state, rect_2, frame);
    draw_row(&keyboard::R1_0, state, rect_3, frame);
    draw_row(&keyboard::R1_1, state, rect_4, frame);
}

fn draw_row<B: Backend>(row_keys: &[KeyUI], state: &App, rect: Rect, frame: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(make_row_constraints(row_keys).as_ref())
        .split(rect);

    for (pos, ui_key) in row_keys.iter().enumerate() {
        let key_state = state
            .key_states
            .get(&ui_key.key)
            .unwrap_or(&KeyState::Untouched);

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

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(border_type);

        let text = Paragraph::new(ui_key.key.to_string())
            .block(block)
            .style(style)
            .alignment(Alignment::Center);

        frame.render_widget(text, chunks[pos])
    }
}
