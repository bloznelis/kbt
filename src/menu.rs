use std::io;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::enable_raw_mode
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::KeyboardSize;

struct MenuState {
    selections: Vec<KeyboardSize>,
    cursor: usize,
}

impl Default for MenuState {
    fn default() -> Self {
        MenuState {
            selections: vec![KeyboardSize::Keyboard60, KeyboardSize::Keyboard80],
            cursor: 0,
        }
    }
}

pub fn run_menu<B: Backend>(
    terminal: &mut Terminal<B>,
) -> io::Result<KeyboardSize> {
    enable_raw_mode();
    let mut state = MenuState::default();
    let max_selection_idx = state.selections.len() - 1;


    //panic!("{:?}", max_selection_idx);

    loop {
        terminal.draw(|f| view_menu(f, &state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    state.cursor = state.cursor.checked_sub(1).unwrap_or(0)
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    state.cursor = std::cmp::min(max_selection_idx, state.cursor + 1)
                }
                KeyCode::Enter => return Ok(state.selections.get(state.cursor).unwrap().clone()),
                //todo: unify errors and return exit here
                //KeyCode::Char('c') => match key.modifiers {
                    //KeyModifiers::CONTROL => 
                //}
                _ => {}
            }
        }
    }
}

fn view_menu<B: Backend>(frame: &mut Frame<B>, state: &MenuState) {
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

    let layout_chunks: Vec<Rect> = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Length(2)].as_ref())
        .split(rect);

    let title = Paragraph::new("kbt").style(
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::ITALIC),
    );

    // render title
    frame.render_widget(title, layout_chunks.get(0).unwrap().clone());

    // render list
    frame.render_stateful_widget(list, layout_chunks.get(1).unwrap().clone(), &mut list_state)
}
