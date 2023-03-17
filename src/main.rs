mod backend;
mod key;
mod keyboard;
mod linux;

use std::{io, thread, time::Duration};

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
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Terminal,
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

        let r1_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(make_row_constraints(&keyboard::R1).as_ref())
            .split(r1_rect);

        for (pos, ui_key) in keyboard::R1.iter().enumerate() {
            let block = Block::default()
                .title(ui_key.key.to_string())
                .borders(Borders::ALL);

            f.render_widget(block, r1_chunks[pos])
        }

        let r2_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(make_row_constraints(&keyboard::R2).as_ref())
            .split(r2_rect);

        for (pos, ui_key) in keyboard::R2.iter().enumerate() {
            let block = Block::default()
                .title(ui_key.key.to_string())
                .borders(Borders::ALL);

            f.render_widget(block, r2_chunks[pos])
        }

        let r3_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(make_row_constraints(&keyboard::R3).as_ref())
            .split(r3_rect);

        for (pos, ui_key) in keyboard::R3.iter().enumerate() {
            let block = Block::default()
                .title(ui_key.key.to_string())
                .borders(Borders::ALL);

            f.render_widget(block, r3_chunks[pos])
        }
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
