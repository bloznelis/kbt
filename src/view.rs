use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::key::Key;
use crate::model::{KbtError, KeyState, KeyUI, KeyboardLayout, KeyboardSize, VerticalKeyPart};
use crate::{App, KEY_HEIGHT};

pub fn show_to_small_dialog(frame: &mut Frame) {
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

pub fn draw(frame: &mut Frame, state: &App) -> Result<(), KbtError> {
    match state.keyboard_size {
        KeyboardSize::Keyboard60 => draw_layout(frame, state, &state.layouts.layout_60),
        KeyboardSize::Keyboard80 => draw_layout(frame, state, &state.layouts.layout_80),
        KeyboardSize::Keyboard100 => draw_layout(frame, state, &state.layouts.layout_100),
    }
}

fn draw_layout(frame: &mut Frame, state: &App, layout: &KeyboardLayout) -> Result<(), KbtError> {
    let terminal_size: Rect = frame.size();

    let left_padding: u16 = (terminal_size.width / 2) - (layout.width / 2);
    let top_padding: u16 = (terminal_size.height / 2) - (layout.height / 2);

    for (idx, row) in layout.rows.iter().enumerate() {
        let idx: u16 = u16::try_from(idx)?;
        let y_offset: u16 = (KEY_HEIGHT * idx) + top_padding;
        let keyboard_rect = Rect::new(left_padding, y_offset, row.width, KEY_HEIGHT);

        draw_row(&row.keys, state, keyboard_rect, frame)
    }

    let less_than_5_pressed = state
        .key_states
        .values()
        .filter(|key_state| matches!(key_state, KeyState::Released | KeyState::Pressed))
        .count()
        < 5;

    if less_than_5_pressed {
        draw_help(top_padding + layout.height + 3, frame);
    }

    Ok(())
}

fn draw_row(row_keys: &[KeyUI], state: &App, keyboard_rect: Rect, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(make_row_constraints(row_keys).as_ref())
        .split(keyboard_rect);

    for (x_pos, ui_key) in row_keys.iter().enumerate() {
        let key_state = state
            .key_states
            .get(&ui_key.key)
            .unwrap_or(&KeyState::Untouched);

        let borders = match (ui_key.key, ui_key.vertical_key_part) {
            (Key::Separator, _) => Borders::NONE,
            (_, Some(VerticalKeyPart::Top)) => Borders::LEFT | Borders::RIGHT | Borders::TOP,
            (_, Some(VerticalKeyPart::Bottom)) => Borders::LEFT | Borders::RIGHT | Borders::BOTTOM,
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

        let label = if let Some(VerticalKeyPart::Bottom) = ui_key.vertical_key_part {
            String::new()
        } else {
            ui_key.key.to_string()
        };

        let text = Paragraph::new(label)
            .block(block)
            .style(style)
            .alignment(Alignment::Center);

        frame.render_widget(text, chunks[x_pos])
    }
}

fn make_row_constraints(keys: &[KeyUI]) -> Vec<Constraint> {
    keys.iter()
        .map(|key| {
            Constraint::Length(
                u16::try_from(key.size.static_len() as i16 + key.size_correction.unwrap_or(0))
                    .unwrap_or(0),
            )
        })
        .collect()
}

fn draw_help(y_offset: u16, frame: &mut Frame) {
    let terminal_size = frame.size();
    let fits = frame.size().height > y_offset;

    if fits {
        let message = "ctrl+r to restart, ctrl+q to quit";
        let message_len = 33;
        let message_height = 1;

        let x_offset: u16 = (terminal_size.width / 2) - (message_len / 2);

        let rect = Rect::new(x_offset, y_offset, message_len, message_height);

        let help = Paragraph::new(message)
            .style(Style::default().fg(Color::Gray).add_modifier(Modifier::DIM));

        frame.render_widget(help, rect);
    }
}
