use crate::chip_ate::{ChipAte, CycleStatus};
use crate::ui::UI;
use crossterm::{
    event::{KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::AppEvent;
use ratatui::backend::CrosstermBackend;
use std::env;
use std::io::stdout;
use std::time::{Duration, Instant};
use tracing::{info, Level};
mod chip_ate;
mod events;
mod opcodes;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create("tui.log")?;
    // only log to file
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(file)
        .init();
    println!("after builder");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        eprintln!("usage: {} <rom_path> [cycles_per_frame]", args[0]);
        std::process::exit(1);
    }
    let rom_path = &args[1];
    let cycles_per_frame = if args.len() == 3 {
        args[2].parse().unwrap_or(12)
    } else {
        12
    };

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = ratatui::Terminal::new(backend)?;
    let mut ui = UI::new(terminal);

    let mut chip8 = ChipAte::new();
    chip8.load_rom(rom_path)?;

    let frame_duration = Duration::from_secs_f64(1.0 / 60.0);
    let mut last_timer_update = Instant::now();
    let timer_interval = Duration::from_secs_f64(1.0 / 60.0);
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let mut event_handler = events::AppEventHandler::new(1000 / 60, shutdown_rx);

    loop {
        let frame_start = Instant::now();

        while let Ok(event) =
            tokio::time::timeout(Duration::from_millis(1), event_handler.next()).await
        {
            match event {
                Some(AppEvent::Tick) => {
                    if last_timer_update.elapsed() >= timer_interval {
                        chip8.update_timers();
                        last_timer_update = Instant::now();
                    }
                }
                Some(AppEvent::Key(event)) => match event.kind {
                    KeyEventKind::Press => {
                        if let KeyCode::Esc = event.code {
                            let _ = shutdown_tx.send(());
                            disable_raw_mode()?;
                            execute!(ui.terminal.backend_mut(), LeaveAlternateScreen)?;
                            ui.cleanup()?;
                            return Ok(());
                        }
                        if let Some(key) = map_key(event.code) {
                            chip8.keypad[key as usize] = 1;
                        }
                    }
                    KeyEventKind::Release => {
                        if let Some(key) = map_key(event.code) {
                            chip8.keypad[key as usize] = 0;
                        }
                    }
                    _ => {}
                },
                None => {}
            }
        }

        for _ in 0..cycles_per_frame {
            match chip8.cycle() {
                CycleStatus::WaitingForKey => break,
                CycleStatus::Normal => {}
            }
        }

        ui.render(&chip8.display)?;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            tokio::time::sleep(frame_duration - elapsed).await;
        }
    }
}

/// Maps keyboard input to Chip-8 keypad values.
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
        // Not Chip-8 standard, but arrow keys for movement
        KeyCode::Up => Some(0x2),
        KeyCode::Down => Some(0x8),
        KeyCode::Left => Some(0x4),
        KeyCode::Right => Some(0x6),
        KeyCode::Esc => None,
        _ => None,
    }
}
