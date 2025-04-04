//use crate::chip_ate::{ChipAte, CycleStatus};
//use crate::ui::UI;
//use crossterm::{
//    event::{self, Event, KeyCode, KeyEventKind},
//    execute,
//    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
//};
//use ratatui::backend::CrosstermBackend;
//use std::io::stdout;
//use std::sync::mpsc;
//use std::time::{Duration, Instant};
//use tokio::task;
//
//mod chip_ate;
//mod opcodes;
//mod ui;
//
//#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
//    // Initialize terminal
//    enable_raw_mode()?;
//    let mut stdout = stdout();
//    execute!(stdout, EnterAlternateScreen)?;
//    let backend = CrosstermBackend::new(stdout);
//    let terminal = ratatui::Terminal::new(backend)?;
//    let mut ui = UI::new(terminal);
//
//    // Initialize emulator
//    let mut chip8 = ChipAte::new();
//    chip8.load_rom("horseyJump.ch8")?; // Replace with path to a Chip-8 ROM
//
//    // Channel for asynchronous key events
//    let (tx, rx) = mpsc::channel();
//
//    // Spawn input handling task
//    task::spawn(async move {
//        loop {
//            if let Ok(event) = event::read() {
//                if let Event::Key(key_event) = event {
//                    let _ = tx.send(key_event);
//                }
//            }
//        }
//    });
//    // Main loop
//    let frame_duration = Duration::from_secs(1) / 60; // 60 Hz
//    let mut last_timer_update = Instant::now();
//    let timer_interval = Duration::from_secs(1) / 60; // 60 Hz timer updates
//    loop {
//        let frame_start = Instant::now();
//
//        // Handle input
//        while let Ok(key_event) = rx.try_recv() {
//            match key_event.kind {
//                KeyEventKind::Press => match key_event.code {
//                    KeyCode::Char('q') => {
//                        disable_raw_mode()?;
//                        execute!(ui.terminal.backend_mut(), LeaveAlternateScreen)?;
//                        ui.cleanup()?;
//                        return Ok(());
//                    }
//                    code => {
//                        if let Some(key) = map_key(code) {
//                            chip8.keypad[key as usize] = 1;
//                            chip8.pressed_key = Some(key);
//                        }
//                    }
//                },
//                KeyEventKind::Release => {
//                    if let Some(key) = map_key(key_event.code) {
//                        chip8.keypad[key as usize] = 0;
//                    }
//                }
//                _ => {}
//            }
//        }
//
//        // Execute CPU cycles (~700 Hz, so ~12 cycles per 60 Hz frame)
//        let mut cycles_this_frame = 0;
//        for _ in 0..12 {
//            match chip8.cycle() {
//                CycleStatus::WaitingForKey => break,
//                CycleStatus::Normal => cycles_this_frame += 1,
//            }
//        }
//
//        // Update timers at 60 Hz
//        if last_timer_update.elapsed() >= timer_interval {
//            chip8.update_timers();
//            last_timer_update = Instant::now();
//        }
//
//        // Render display
//        let display_text = chip8.render_display();
//        ui.render(&display_text)?;
//
//        // Maintain 60 Hz frame rate
//        let elapsed = frame_start.elapsed();
//        if elapsed < frame_duration {
//            std::thread::sleep(frame_duration - elapsed);
//        }
//    }
//
//    // Main loop
//    let frame_duration = Duration::from_secs(1) / 60; // 60 Hz
//    loop {
//        let frame_start = Instant::now();
//
//        // Handle input
//        while let Ok(key_event) = rx.try_recv() {
//            match key_event.kind {
//                KeyEventKind::Press => match key_event.code {
//                    KeyCode::Char('q') => {
//                        disable_raw_mode()?;
//                        execute!(ui.terminal.backend_mut(), LeaveAlternateScreen)?;
//                        ui.cleanup()?;
//                        return Ok(());
//                    }
//                    code => {
//                        if let Some(key) = map_key(code) {
//                            chip8.keypad[key as usize] = 1;
//                            chip8.pressed_key = Some(key);
//                        }
//                    }
//                },
//                KeyEventKind::Release => {
//                    if let Some(key) = map_key(key_event.code) {
//                        chip8.keypad[key as usize] = 0;
//                    }
//                }
//                _ => {}
//            }
//        }
//
//        // Execute CPU cycles (approx 700 Hz, so ~12 cycles per 60 Hz frame)
//        for _ in 0..12 {
//            if chip8.cycle() == CycleStatus::WaitingForKey {
//                break;
//            }
//        }
//
//        // Update timers
//        chip8.update_timers();
//
//        // Render display
//        let display_text = chip8.render_display();
//        ui.render(&display_text)?;
//
//        // Maintain 60 Hz
//        let elapsed = frame_start.elapsed();
//        if elapsed < frame_duration {
//            std::thread::sleep(frame_duration - elapsed);
//        }
//    }
//}
//
///// Maps keyboard keys to Chip-8 keypad (original layout: 123C, 456D, 789E, A0BF).
//fn map_key(code: KeyCode) -> Option<u8> {
//    match code {
//        KeyCode::Char('1') => Some(0x1),
//        KeyCode::Char('2') => Some(0x2),
//        KeyCode::Char('3') => Some(0x3),
//        KeyCode::Char('4') => Some(0xC),
//        KeyCode::Char('q') => Some(0x4),
//        KeyCode::Char('w') => Some(0x5),
//        KeyCode::Char('e') => Some(0x6),
//        KeyCode::Char('r') => Some(0xD),
//        KeyCode::Char('a') => Some(0x7),
//        KeyCode::Char('s') => Some(0x8),
//        KeyCode::Char('d') => Some(0x9),
//        KeyCode::Char('f') => Some(0xE),
//        KeyCode::Char('z') => Some(0xA),
//        KeyCode::Char('x') => Some(0x0),
//        KeyCode::Char('c') => Some(0xB),
//        KeyCode::Char('v') => Some(0xF),
//        _ => None,
//    }
//}
use crate::chip_ate::{ChipAte, CycleStatus};
use crate::ui::UI;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use std::env;
use std::io::{self, stdout};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use tokio::task;

mod chip_ate;
mod opcodes;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ### Parse Command-Line Arguments
    // Ensure the user provides a ROM path as an argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <rom_path>", args[0]);
        std::process::exit(1);
    }
    let rom_path = &args[1];

    // ### Initialize Terminal
    // Set up raw mode and alternate screen for a clean UI
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = ratatui::Terminal::new(backend)?;
    let mut ui = UI::new(terminal);

    // ### Initialize Emulator
    // Create a new Chip-8 instance and load the ROM
    let mut chip8 = ChipAte::new();
    chip8.load_rom(rom_path)?;

    // ### Set Up Asynchronous Input Handling
    // Create a channel for key events and spawn a task to read them
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

    // ### Main Loop
    // Run at 60 Hz, handling input, CPU cycles, timers, and rendering
    let frame_duration = Duration::from_secs(1) / 60; // 60 Hz
    let mut last_timer_update = Instant::now();
    let timer_interval = Duration::from_secs(1) / 60; // 60 Hz timer updates
    loop {
        let frame_start = Instant::now();

        // #### Handle Input
        // Process key events from the channel
        while let Ok(key_event) = rx.try_recv() {
            match key_event.kind {
                KeyEventKind::Press => match key_event.code {
                    KeyCode::Char('q') => {
                        // Exit cleanly on 'q'
                        disable_raw_mode()?;
                        execute!(ui.terminal.backend_mut(), LeaveAlternateScreen)?;
                        ui.cleanup()?;
                        return Ok(());
                    }
                    code => {
                        // Map keypress to Chip-8 keypad
                        if let Some(key) = map_key(code) {
                            chip8.keypad[key as usize] = 1;
                            chip8.pressed_key = Some(key);
                        }
                    }
                },
                KeyEventKind::Release => {
                    // Clear key state on release
                    if let Some(key) = map_key(key_event.code) {
                        chip8.keypad[key as usize] = 0;
                    }
                }
                _ => {}
            }
        }

        // #### Execute CPU Cycles
        // Run ~12 cycles per frame to approximate 700 Hz
        let mut cycles_this_frame = 0;
        for _ in 0..12 {
            match chip8.cycle() {
                CycleStatus::WaitingForKey => break,
                CycleStatus::Normal => cycles_this_frame += 1,
            }
        }

        // #### Update Timers
        // Update delay and sound timers at 60 Hz
        if last_timer_update.elapsed() >= timer_interval {
            chip8.update_timers();
            last_timer_update = Instant::now();
        }

        // #### Render Display
        ui.render(&chip8.display)?;

        // #### Maintain Frame Rate
        // Sleep to ensure 60 Hz
        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
    }
}

/// Maps keyboard keys to Chip-8 keypad values.
///
/// The original Chip-8 keypad layout is:
/// ```text
/// 1 2 3 C
/// 4 5 6 D
/// 7 8 9 E
/// A 0 B F
/// ```
/// This function maps modern keyboard keys to these values, with additional support for arrow keys.
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
        // Optional arrow key mappings for directional controls
        KeyCode::Up => Some(0x2),
        KeyCode::Down => Some(0x8),
        KeyCode::Left => Some(0x4),
        KeyCode::Right => Some(0x6),
        _ => None,
    }
}
