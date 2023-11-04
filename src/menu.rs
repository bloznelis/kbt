use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::enable_raw_mode,
};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::{KbtError, KeyboardSize, MenuResult};

struct MenuState {
    selections: Vec<KeyboardSize>,
    cursor: usize,
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState {
            selections: vec![
                KeyboardSize::Keyboard60,
                KeyboardSize::Keyboard80,
                KeyboardSize::Keyboard100,
            ],
            cursor: 0,
        }
    }
}

pub fn run_menu<B: Backend>(terminal: &mut Terminal<B>) -> Result<MenuResult, KbtError> {
    enable_raw_mode()?;
    let mut state = MenuState::default();
    let max_selection_idx = state.selections.len() - 1;

    loop {
        terminal.draw(|f| view_menu(f, &state).expect("Failed to draw menu"))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => state.cursor = state.cursor.saturating_sub(1),
                KeyCode::Down | KeyCode::Char('j') => {
                    state.cursor = std::cmp::min(max_selection_idx, state.cursor + 1)
                }
                KeyCode::Enter => {
                    return Ok(MenuResult::KeyboardSelected(
                        state
                            .selections
                            .get(state.cursor)
                            .ok_or(KbtError {
                                message: String::from("Failed to get a menu selection by idx"),
                            })?
                            .clone(),
                    ))
                }
                KeyCode::Char('c') | KeyCode::Char('q') => {
                    if key.modifiers == KeyModifiers::CONTROL {
                        return Ok(MenuResult::Terminate);
                    }
                }
                _ => {}
            }
        }
    }
}

fn view_menu<B: Backend>(frame: &mut Frame<B>, state: &MenuState) -> Result<(), KbtError> {
    let items: Vec<ListItem> = state
        .selections
        .iter()
        .map(|selection| ListItem::new(selection.to_string()))
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(state.cursor));

    let terminal_size: Rect = frame.size();

    let layout_height: u16 = 5;
    let layout_width: u16 = 15;
    let left_padding: u16 = (terminal_size.width / 2) - (layout_width / 2);
    let top_padding: u16 = (terminal_size.height / 2) - (layout_height / 2);

    let rect = Rect::new(left_padding, top_padding, layout_width, layout_height);

    let layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(2)].as_ref())
        .split(rect);

    let title = Paragraph::new("kbt").style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::ITALIC),
    );

    // render title
    frame.render_widget(
        title,
        *layout_chunks.get(0).ok_or(KbtError {
            message: String::from("Failed to get correct layout chunk for title"),
        })?,
    );

    // render list
    frame.render_stateful_widget(
        list,
        *layout_chunks.get(1).ok_or(KbtError {
            message: String::from("Failed to get correct layout chunk for list"),
        })?,
        &mut list_state,
    );

    Ok(())
}
