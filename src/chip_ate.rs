use crate::opcodes::Instruction;
use log::debug;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MEMORY_SIZE: usize = 4096;
const PROGRAM_START: u16 = 0x200;
const FONT_START: u16 = 0x50;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const STACK_SIZE: usize = 16;
const REGISTER_COUNT: usize = 16;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ChipAte {
    pub memory: [u8; MEMORY_SIZE],
    pub v: [u8; REGISTER_COUNT],
    pub i: u16,
    pub pc: u16,
    pub stack: [u16; STACK_SIZE],
    pub sp: u8,
    pub display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keypad: [u8; REGISTER_COUNT],
    pub pressed_key: Option<u8>,
    pub beep_active: Arc<Mutex<bool>>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum CycleStatus {
    Normal,
    WaitingForKey,
}

impl Default for ChipAte {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl ChipAte {
    pub fn new() -> Self {
        let beep_active = Arc::new(Mutex::new(false));
        let beep_thread = beep_active.clone();
        
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(50));
                if let Ok(active) = beep_thread.lock() {
                    if *active {
                        print!("\x07");
                    }
                }
            }
        });
        
        let mut chip_ate = ChipAte {
            memory: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
            v: [0; REGISTER_COUNT],
            i: 0,
            pc: PROGRAM_START,
            sp: 0,
            display: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; REGISTER_COUNT],
            pressed_key: None,
            beep_active,
        };
        const FONTSET: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for (i, &byte) in FONTSET.iter().enumerate() {
            chip_ate.memory[FONT_START as usize + i] = byte;
        }
        chip_ate
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let rom = std::fs::read(path)?;
        if rom.len() > MEMORY_SIZE - PROGRAM_START as usize {
            return Err(format!("ROM too large: {} bytes, max {} bytes", rom.len(), MEMORY_SIZE - PROGRAM_START as usize).into());
        }
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[PROGRAM_START as usize + i] = byte;
        }
        Ok(())
    }

    fn fetch(&mut self) -> u16 {
        if self.pc as usize + 1 >= MEMORY_SIZE {
            return 0;
        }
        let high = self.memory[self.pc as usize] as u16;
        let low = self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        (high << 8) | low
    }

    fn execute(&mut self, instruction: Instruction) {
        debug!("Inside of execute instruction: {:?}", instruction);
        match instruction {
            Instruction::ClearScreen => {
                // Set all display pixels to 0 (black)
                self.display = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT];
            }
            Instruction::Return => {
                // Pop the return address from the stack and set PC to it
                let ret = self.pop();
                self.pc = ret;
            }
            Instruction::Jump { address } => {
                // Set PC to the specified address, changing execution flow
                self.pc = address;
            }
            Instruction::Call { address } => {
                // Push current PC to stack, then jump to subroutine address
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = address;
            }
            Instruction::SkipEq { vx, byte } => {
                // Compare Vx to byte; skip next instruction (2 bytes) if equal
                if self.v[vx as usize] == byte {
                    self.pc += 2;
                }
            }
            Instruction::SkipNe { vx, byte } => {
                if self.v[vx as usize] != byte {
                    self.pc += 2;
                }
            }
            Instruction::SkipEqReg { vx, vy } => {
                // Skip if Vx equals Vy
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 2;
                }
            }
            Instruction::LoadByte { vx, byte } => {
                // Load immediate value into Vx
                self.v[vx as usize] = byte;
            }
            Instruction::AddByte { vx, byte } => {
                // Add byte to Vx with wrapping (no carry flag affected)
                self.v[vx as usize] = self.v[vx as usize].wrapping_add(byte);
            }
            Instruction::LoadReg { vx, vy } => {
                // Copy Vy into Vx
                self.v[vx as usize] = self.v[vy as usize];
            }
            Instruction::Or { vx, vy } => {
                // Bitwise OR between Vx and Vy, result in Vx
                self.v[vx as usize] |= self.v[vy as usize];
            }
            Instruction::And { vx, vy } => {
                // Bitwise AND between Vx and Vy, result in Vx
                self.v[vx as usize] &= self.v[vy as usize];
            }
            Instruction::Xor { vx, vy } => {
                // Bitwise XOR between Vx and Vy, result in Vx
                self.v[vx as usize] ^= self.v[vy as usize];
            }
            Instruction::AddReg { vx, vy } => {
                // Add Vy to Vx, set VF to 1 if carry occurs (overflow)
                let (result, carry) = self.v[vx as usize].overflowing_add(self.v[vy as usize]);
                self.v[vx as usize] = result;
                self.v[0xF] = carry as u8;
            }
            Instruction::Sub { vx, vy } => {
                // Subtract Vy from Vx, set VF to 1 if no borrow (Vx >= Vy)
                let (result, borrow) = self.v[vx as usize].overflowing_sub(self.v[vy as usize]);
                self.v[vx as usize] = result;
                self.v[0xF] = (!borrow) as u8;
            }
            Instruction::Shr { vx, vy: _ } => {
                self.v[0xF] = self.v[vx as usize] & 0x1;
                self.v[vx as usize] >>= 1;
            }
            Instruction::SubN { vx, vy } => {
                // Set Vx to Vy - Vx, VF is 1 if no borrow (Vy >= Vx)
                let (result, borrow) = self.v[vy as usize].overflowing_sub(self.v[vx as usize]);
                self.v[vx as usize] = result;
                self.v[0xF] = (!borrow) as u8;
            }
            Instruction::Shl { vx } => {
                // Shift Vx left by 1, VF gets the bit shifted out
                self.v[0xF] = (self.v[vx as usize] >> 7) & 0x1;
                self.v[vx as usize] <<= 1;
            }
            Instruction::SkipNeReg { vx, vy } => {
                // Skip if Vx does not equal Vy
                if self.v[vx as usize] != self.v[vy as usize] {
                    self.pc += 2;
                }
            }
            Instruction::LoadI { address } => {
                // Set index register I to the specified address
                self.i = address;
            }
            Instruction::JumpV0 { address } => {
                // Jump to address plus V0, useful for computed jumps
                self.pc = address + self.v[0] as u16;
            }
            Instruction::Random { vx, byte } => {
                // Generate a random byte, AND it with byte, store in Vx
                let mut rng = rand::rng();
                let random_byte: u8 = rng.random();
                self.v[vx as usize] = random_byte & byte;
            }
            Instruction::Draw { vx, vy, n } => {
                let x = self.v[vx as usize] as usize % DISPLAY_WIDTH;
                let y = self.v[vy as usize] as usize % DISPLAY_HEIGHT;
                self.v[0xF] = 0;
                for row in 0..n as usize {
                    let sprite = self.memory[(self.i + row as u16) as usize];
                    for col in 0..8 {
                        if (sprite & (0x80 >> col)) != 0 {
                            let idx = (y + row) * DISPLAY_WIDTH + (x + col);
                            if idx < DISPLAY_WIDTH * DISPLAY_HEIGHT {
                                let pixel = &mut self.display[idx];
                                if *pixel == 1 {
                                    self.v[0xF] = 1;
                                }
                                *pixel ^= 1;
                            }
                        }
                    }
                }
            }
            Instruction::SkipKey { vx } => {
                // skip if the key indexed by Vx is pressed
                if self.keypad[self.v[vx as usize] as usize] == 1 {
                    self.pc += 2;
                }
            }
            Instruction::SkipNoKey { vx } => {
                // skip if the key indexed by Vx is not pressed
                if self.keypad[self.v[vx as usize] as usize] == 0 {
                    self.pc += 2;
                }
            }
            Instruction::LoadDelay { vx } => {
                // load current delay timer value into Vx
                self.v[vx as usize] = self.delay_timer;
            }
            Instruction::WaitKey { vx } => {
                if let Some(key) = self.pressed_key {
                    self.v[vx as usize] = key;
                    self.pressed_key = None;
                } else {
                    self.pc -= 2;
                }
            }
            Instruction::SetDelay { vx } => {
                // set delay timer to Vx value
                self.delay_timer = self.v[vx as usize];
            }
            Instruction::SetSound { vx } => {
                // Set sound timer to Vx value
                self.sound_timer = self.v[vx as usize];
            }
            Instruction::AddI { vx } => {
                // Add Vx to I with wrapping around 16-bit range
                self.i = self.i.wrapping_add(self.v[vx as usize] as u16);
            }
            Instruction::LoadFont { vx } => {
                // set I to the memory address of the font sprite for digit Vx
                // each sprite is 5 bytes, so Vx * 5 gives the offset from 0x000
                self.i = FONT_START + (self.v[vx as usize] * 5) as u16;
            }
            Instruction::StoreBCD { vx } => {
                // Convert Vx to binary-coded decimal and store at I, I+1, I+2
                let value = self.v[vx as usize];
                self.memory[self.i as usize] = value / 100; // Hundreds
                self.memory[self.i as usize + 1] = (value / 10) % 10; // Tens
                self.memory[self.i as usize + 2] = value % 10; // Ones
            }
            Instruction::StoreRegs { vx } => {
                // Store registers V0 through Vx into memory starting at I
                for reg in 0..=vx as usize {
                    self.memory[self.i as usize + reg] = self.v[reg];
                }
            }
            Instruction::LoadRegs { vx } => {
                // Load registers V0 through Vx from memory starting at I
                for reg in 0..=vx as usize {
                    self.v[reg] = self.memory[self.i as usize + reg];
                }
            }
            Instruction::Unknown { opcode } => {
                // log unrecognized opcodes for debugging
                println!("Unknown opcode: {:#X}", opcode);
            }
        }
    }
    pub fn cycle(&mut self) -> CycleStatus {
        let opcode = self.fetch();
        let instruction = Instruction::from_opcode(opcode);
        match instruction {
            Instruction::WaitKey { vx } => {
                if let Some(key) = self.pressed_key {
                    self.v[vx as usize] = key;
                    self.pressed_key = None;
                    CycleStatus::Normal
                } else {
                    self.pc -= 2;
                    CycleStatus::WaitingForKey
                }
            }
            _ => {
                self.execute(instruction);
                CycleStatus::Normal
            }
        }
    }

    pub fn set_delay_timer(&mut self, new_time: u8) {
        self.delay_timer = new_time
    }

    pub fn read_delay_timer(&mut self) -> u8 {
        self.delay_timer
    }

    pub fn set_sound_timer(&mut self, new_time: u8) {
        self.sound_timer = new_time
    }
    pub fn push(&mut self, val: u16) {
        if (self.sp as usize) < STACK_SIZE {
            self.stack[self.sp as usize] = val;
            self.sp += 1;
        }
    }
    pub fn pop(&mut self) -> u16 {
        if self.sp > 0 {
            self.sp -= 1;
            self.stack[self.sp as usize]
        } else {
            0
        }
    }

    pub fn render_display(&self) -> String {
        let mut output = String::with_capacity(DISPLAY_WIDTH * DISPLAY_HEIGHT + DISPLAY_HEIGHT);
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                output.push(if self.display[y * DISPLAY_WIDTH + x] == 1 {
                    '█'
                } else {
                    ' '
                });
            }
            output.push('\n');
        }
        output
    }
    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            if let Ok(mut active) = self.beep_active.lock() {
                *active = true;
            }
        } else if let Ok(mut active) = self.beep_active.lock() {
            *active = false;
        }
    }
}
