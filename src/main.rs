mod backend;
mod key;
mod keyboard;
mod linux;

use std::{
    io::{self, Stdout},
    thread,
    time::Duration,
};

use backend::KeyBackend;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use key::Key;
use linux::X11;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
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

fn draw_row(row_keys: &[KeyUI], rect: Rect, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(make_row_constraints(row_keys).as_ref())
        .split(rect);

    for (pos, ui_key) in row_keys.iter().enumerate() {
        let block = Block::default().borders(Borders::ALL);
        let text = Paragraph::new(ui_key.key.to_string())
            .block(block)
            .alignment(Alignment::Center);

        frame.render_widget(text, chunks[pos])
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let receiver = X11.subscribe()?;

    // loop {
    //     let key = receiver.recv()?;
    //     println!("received {:?}", key)
    // }

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    let rect = Rect::new(0, 0, 114, 10);
    // rect.

    terminal.draw(|f| {
        let terminal_size: Rect = f.size();
        println!("terminal rect {:?}", terminal_size);
        let row_height = terminal_size.height / 5;
        let r1_rect = Rect::new(0, 0, terminal_size.width, row_height);
        let r2_rect = Rect::new(0, row_height, terminal_size.width, row_height);
        let r3_rect = Rect::new(0, row_height * 2, terminal_size.width, row_height);
        println!("r1 rect {:?}", r1_rect);
        println!("r2 rect {:?}", r2_rect);

        draw_row(&keyboard::R1, r1_rect, f);
        draw_row(&keyboard::R2, r2_rect, f);
        draw_row(&keyboard::R3, r3_rect, f);
    })?;

    thread::sleep(Duration::from_millis(10000));

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
