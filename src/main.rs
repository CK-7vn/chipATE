use crate::chip_ate::{ChipAte, CycleStatus};
use crate::ui::UI;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use sdl2::keyboard::Keycode;
use std::env;
use std::io::stdout;
use std::time::{Duration, Instant};
use tokio::sync::oneshot;
use tokio::task::LocalSet;

mod chip_ate;
mod events;
mod opcodes;
mod ui;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_file = std::fs::File::create("tui.log")?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(log_file)
        .init();
    println!("After logging initialization.");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rom_path> [cycles_per_frame]", args[0]);
        std::process::exit(1);
    }
    let rom_path = &args[1];
    let cycles_per_frame: usize = if args.len() >= 3 {
        args[2].parse().unwrap_or(12)
    } else {
        12
    };

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let terminal = ratatui::Terminal::new(backend)?;
    let mut ui = UI::new(terminal);

    let mut chip8 = ChipAte::new();
    if let Err(e) = chip8.load_rom(rom_path) {
        eprintln!("Failed to load ROM: {}", e);
        std::process::exit(1);
    }

    let (_shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let local = LocalSet::new();
    local.spawn_local(async move {
        let mut event_handler = events::AppEventHandler::new(16, shutdown_rx);

        let frame_duration = Duration::from_secs_f64(1.0 / 60.0);
        let mut last_timer_update = Instant::now();
        let timer_interval = Duration::from_secs_f64(1.0 / 60.0);

        'main_loop: loop {
            let frame_start = Instant::now();

            while let Ok(event) =
                tokio::time::timeout(Duration::from_millis(1), event_handler.next()).await
            {
                if let Some(app_event) = event {
                    match app_event {
                        events::AppEvent::Tick => {}
                        events::AppEvent::Key { key, pressed } => {
                            if key == Keycode::Escape {
                                break 'main_loop;
                            }
                            if let Some(mapped_key) = map_sdl_key(key) {
                                if pressed {
                                    chip8.keypad[mapped_key as usize] = 1;
                                    chip8.pressed_key = Some(mapped_key);
                                } else {
                                    chip8.keypad[mapped_key as usize] = 0;
                                }
                            }
                        }
                    }
                } else {
                    break;
                }
            }

            for _ in 0..cycles_per_frame {
                match chip8.cycle() {
                    CycleStatus::WaitingForKey => break,
                    CycleStatus::Normal => {}
                }
            }

            if last_timer_update.elapsed() >= timer_interval {
                chip8.update_timers();
                last_timer_update = Instant::now();
            }

            if let Err(e) = ui.render(&chip8.display) {
                eprintln!("UI render error: {:?}", e);
            }

            let elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                tokio::time::sleep(frame_duration - elapsed).await;
            }
        }

        disable_raw_mode().expect("Failed to disable raw mode");
        execute!(ui.terminal.backend_mut(), LeaveAlternateScreen)
            .expect("Failed to leave alternate screen");
        ui.cleanup().expect("UI cleanup failed");
    });

    local.await;

    Ok(())
}

fn map_sdl_key(key: Keycode) -> Option<u8> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
