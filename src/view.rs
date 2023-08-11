use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Rect, Layout, Direction, Alignment, Constraint};
use ratatui::style::{Style, Color, Modifier};
use ratatui::widgets::{Paragraph, Borders, BorderType, Block};

use crate::key::Key;
use crate::{App, keyboard80, keyboard60};
use crate::model::{KeyUI, KeyboardSize, KeyState, KbtError};

pub fn show_to_small_dialog<B: Backend>(frame: &mut Frame<B>) {
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

pub fn draw<B: Backend>(frame: &mut Frame<B>, state: &App) -> Result<(), KbtError> {
    match state.keyboard_size {
        KeyboardSize::Keyboard80 => draw_80(frame, state),
        KeyboardSize::Keyboard60 => draw_60(frame, state),
    }
}

fn draw_80<B: Backend>(frame: &mut Frame<B>, state: &App) -> Result<(), KbtError>{
    let terminal_size: Rect = frame.size();

    let rows = keyboard80::ROWS;
    let rows_count: u16 = 6;

    let row_height: u16 = 3;
    let layout_height: u16 = 3 * rows_count;
    let layout_width: u16 = 93;
    let left_padding: u16 = (terminal_size.width / 2) - (layout_width / 2);
    let top_padding: u16 = (terminal_size.height / 2) - (layout_height / 2);

    for (idx, row) in rows.iter().enumerate() {
        let idx: u16 = u16::try_from(idx)?;
        let row_width: u16 = calc_static_row_len(row);
        let y_offset: u16 = (row_height * idx) + top_padding;
        let rect = Rect::new(left_padding, y_offset, row_width, row_height);

        draw_row(row, state, rect, frame)
    }

    let less_than_5_pressed = state
        .key_states
        .values()
        .filter(|a| matches!(a, KeyState::Released | KeyState::Pressed))
        .count() < 5;

    if less_than_5_pressed {
        draw_help(top_padding + (row_height * rows_count) + 3, frame);
    }

    Ok(())
}

fn draw_60<B: Backend>(frame: &mut Frame<B>, state: &App) -> Result<(), KbtError> {
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
        let idx: u16 = u16::try_from(idx)?;
        let row_width: u16 = calc_static_row_len(row);
        let y_offset: u16 = (row_height * idx) + top_padding;
        let rect = Rect::new(left_padding, y_offset, row_width, row_height);

        draw_row(row, state, rect, frame)
    }

    let less_than_5_pressed = state
        .key_states
        .values()
        .filter(|a| matches!(a, KeyState::Released | KeyState::Pressed))
        .count() < 5;

    if less_than_5_pressed {
        draw_help(top_padding + (row_height * rows_count) + 3, frame);
    }

    Ok(())
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

fn draw_help<B: Backend>(y_offset: u16, frame: &mut Frame<B>) {
    let terminal_size = frame.size();
    let message = "ctrl+r to restart, ctrl+q to quit";
    let message_len = 33;
    let message_height = 1;

    let x_offset: u16 = (terminal_size.width / 2) - (message_len / 2);

    let rect = Rect::new(x_offset, y_offset, message_len, message_height);

    let help =
        Paragraph::new(message).style(Style::default().fg(Color::Gray).add_modifier(Modifier::DIM));

    frame.render_widget(help, rect);
}
