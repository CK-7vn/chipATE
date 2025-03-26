pub mod chip_ate;
pub mod opcodes;
pub mod ui;

// main.rs
// if using ratatui input is going to need to be async
use chip_ate::ChipAte;
use crossterm::event::KeyCode;
use opcodes::Instruction;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn map_key(key_code: KeyCode) -> Option<u8> {
    match key_code {
        KeyCode::Char('x') | KeyCode::Char('X') => Some(0x0),
        KeyCode::Char('1') => Some(0x1),
        KeyCode::Char('2') => Some(0x2),
        KeyCode::Char('3') => Some(0x3),
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(0x4),
        KeyCode::Char('w') | KeyCode::Char('W') => Some(0x5),
        KeyCode::Char('e') | KeyCode::Char('E') => Some(0x6),
        KeyCode::Char('a') | KeyCode::Char('A') => Some(0x7),
        KeyCode::Char('s') | KeyCode::Char('S') => Some(0x8),
        KeyCode::Char('d') | KeyCode::Char('D') => Some(0x9),
        KeyCode::Char('z') | KeyCode::Char('Z') => Some(0xA),
        KeyCode::Char('c') | KeyCode::Char('C') => Some(0xB),
        KeyCode::Char('4') => Some(0xC),
        KeyCode::Char('r') | KeyCode::Char('R') => Some(0xD),
        KeyCode::Char('f') | KeyCode::Char('F') => Some(0xE),
        KeyCode::Char('v') | KeyCode::Char('V') => Some(0xF),
        _ => None,
    }
}
fn main() {
    // Initialize the emulator
    let mut chip = ChipAte::new();
    // Load a ROM (replace with your ROM path)
    chip.load_rom("test_opcode.ch8")
        .expect("Failed to load ROM");

    // Set up a window with minifb, scaling the 64x32 display by 10x for visibility

    // Buffer for rendering: 32-bit colors (0xFFFFFF = white, 0x000000 = black)
    let mut buffer: Vec<u32> = vec![0; 64 * 32];
    let mut last_timer = Instant::now(); // Track time for 60Hz timer updates

    // handle the WaitKey (Fx0A) instruction specially
    let prev_pc = chip.pc; // Save PC before cycle
    chip.cycle(); // run one CPU cycle
    if let Instruction::WaitKey { vx } = Instruction::from_opcode(
        (chip.memory[prev_pc as usize] as u16) << 8 | chip.memory[prev_pc as usize + 1] as u16,
    ) {
        let mut key_pressed = false;
        for i in 0..16 {
            if chip.keypad[i] == 1 {
                chip.v[vx as usize] = i as u8; // Store pressed key in Vx
                chip.pc += 2; // Move past instruction
                key_pressed = true;
                break;
            }
        }
        if !key_pressed {
            chip.pc = prev_pc; // Rewind PC to retry until a key is pressed
        }
    }

    // Update timers at 60Hz (every 16ms)
    if last_timer.elapsed() >= Duration::from_millis(16) {
        if chip.delay_timer > 0 {
            chip.delay_timer -= 1; // decrement delay timer
        }
        if chip.sound_timer > 0 {
            chip.sound_timer -= 1; // decrement sound timer
        }
        last_timer = Instant::now(); // reset timer
    }

    // Control CPU speed to ~500Hz (2ms per cycle)
    std::thread::sleep(Duration::from_millis(2));
}
