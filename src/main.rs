use crate::chip_ate::{ChipAte, CycleStatus};
use crate::ui::UI;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::AppEvent;
use ratatui::backend::CrosstermBackend;
use std::env;
use std::io::stdout;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tokio::task;

mod chip_ate;
mod events;
mod opcodes;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //get the rom path
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_path>", args[0]);
        std::process::exit(1);
    }
    let rom_path = &args[1];

    //basic tui setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = ratatui::Terminal::new(backend)?;
    let mut ui = UI::new(terminal);

    let mut chip8 = ChipAte::new();
    chip8.load_rom(rom_path)?;

    let (tx, rx) = mpsc::channel();
    task::spawn(async move {
        loop {
            if let Ok(event) = event::read() {
                if let Event::Key(key_event) = event {
                    let _ = tx.send(key_event);
                }
            }
        }
    });

    // main loop runs at ~60Hz.
    let frame_duration = Duration::from_secs(1) / 60;
    let mut last_timer_update = Instant::now();
    let timer_interval = Duration::from_secs(1) / 60;
    loop {
        let frame_start = Instant::now();

        while let Ok(key_event) = rx.try_recv() {
            match key_event.kind {
                KeyEventKind::Press => {
                    if let KeyCode::Esc = key_event.code {
                        disable_raw_mode()?;
                        execute!(ui.terminal.backend_mut(), LeaveAlternateScreen)?;
                        ui.cleanup()?;
                        return Ok(());
                    }
                    if let Some(key) = map_key(key_event.code) {
                        if chip8.keypad[key as usize] == 0 {
                            chip8.keypad[key as usize] = 1;
                            chip8.pressed_key = Some(key);
                        }
                    }
                }
                KeyEventKind::Release => {
                    if let Some(key) = map_key(key_event.code) {
                        chip8.keypad[key as usize] = 0;
                    }
                }
                _ => {}
            }
        }

        for _ in 0..12 {
            match chip8.cycle() {
                CycleStatus::WaitingForKey => break,
                CycleStatus::Normal => {}
            }
        }

        if last_timer_update.elapsed() >= timer_interval {
            chip8.update_timers();
            last_timer_update = Instant::now();
        }

        ui.render(&chip8.display)?;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }
}

/// ```text
/// 1 2 3 C
/// 4 5 6 D
/// 7 8 9 E
/// A 0 B F
/// ```
fn map_key(code: KeyCode) -> Option<u8> {
    match code {
        KeyCode::Char('1') => Some(0x1),
        KeyCode::Char('2') => Some(0x2),
        KeyCode::Char('3') => Some(0x3),
        KeyCode::Char('4') => Some(0xC),
        KeyCode::Char('q') => Some(0x4),
        KeyCode::Char('w') => Some(0x5),
        KeyCode::Char('e') => Some(0x6),
        KeyCode::Char('r') => Some(0xD),
        KeyCode::Char('a') => Some(0x7),
        KeyCode::Char('s') => Some(0x8),
        KeyCode::Char('d') => Some(0x9),
        KeyCode::Char('f') => Some(0xE),
        KeyCode::Char('z') => Some(0xA),
        KeyCode::Char('x') => Some(0x0),
        KeyCode::Char('c') => Some(0xB),
        KeyCode::Char('v') => Some(0xF),
        // not chip8 standard, but arrow keys for movement.
        KeyCode::Up => Some(0x2),
        KeyCode::Down => Some(0x8),
        KeyCode::Left => Some(0x4),
        KeyCode::Right => Some(0x6),
        //randomly mapped the esc key, doesn't do anything
        KeyCode::Esc => Some(0x77),
        _ => None,
    }
}
