use crate::opcodes::Instruction;
use rand::Rng;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ChipAte {
    //4Kb of memory 1000 memory locations
    pub memory: [u8; 4096],
    // 16 general purpose registers v0-vf
    pub v: [u8; 16],
    //index register
    pub i: u16,
    //program coutner
    pub pc: u16,
    //stack
    pub stack: [u16; 16],
    //stack pointer
    pub sp: u8,

    //display 64 x 32 pixels
    pub display: [u8; 64 * 32],

    pub delay_timer: u8,
    pub sound_timer: u8,
    //hex keypad state 0 to F 8, 4, 6, and 2 keys usually used for directional input
    pub keypad: [u8; 16],
    pub pressed_key: Option<u8>,
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
        let mut chip_ate = ChipAte {
            memory: [0; 4096],
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            display: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            pressed_key: Some(0),
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
            chip_ate.memory[i] = byte;
        }
        chip_ate
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), std::io::Error> {
        let rom = std::fs::read(path)?; // read rom into a byte vector
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = byte; // copying each byte into mem
        }
        Ok(())
    }

    fn fetch(&mut self) -> u16 {
        let low = self.memory[self.pc as usize + 1] as u16; //low byte
        let high = self.memory[self.pc as usize] as u16; // high byte
        self.pc += 2; // advance pc to next instruction
                      //    println!(bits)
        (high << 8) | low //opcode
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ClearScreen => {
                // Set all display pixels to 0 (black)
                self.display = [0; 64 * 32];
            }
            Instruction::Return => {
                // Pop the return address from the stack and set PC to it
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
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
                // Skip next instruction if Vx does not equal byte
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
            Instruction::Shr { vx } => {
                // Shift Vx right by 1, VF gets the bit shifted out
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
                let random_byte = rand::rng().random::<u8>();
                self.v[vx as usize] = random_byte & byte;
            }
            Instruction::Draw { vx, vy, n } => {
                // Draw an n-byte sprite from memory[I] at coordinates (Vx, Vy)
                // Sprites are 8 pixels wide, n pixels tall; VF set to 1 on collision
                let x = self.v[vx as usize] as usize % 64; // Wrap around 64-pixel width
                let y = self.v[vy as usize] as usize % 32; // Wrap around 32-pixel height
                self.v[0xF] = 0; // Reset collision flag
                for row in 0..n as usize {
                    let sprite = self.memory[(self.i + row as u16) as usize]; // Get sprite row
                    for col in 0..8 {
                        if (sprite & (0x80 >> col)) != 0 {
                            // Check each bit (left to right)
                            let idx = (y + row) * 64 + (x + col); // Calculate display index
                            if idx < 64 * 32 {
                                // Ensure within bounds
                                let pixel = &mut self.display[idx];
                                if *pixel == 1 {
                                    self.v[0xF] = 1; // Collision if pixel was already on
                                }
                                *pixel ^= 1; // XOR to toggle pixel state
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
                // wait for a key press; handled in main loop, here we just rewind PC
                if let Some(key) = self.pressed_key {
                    self.v[vx as usize] = key;
                    self.pressed_key = None
                } else {
                    self.pc -= 2; // Retry this instruction until a key is pressed
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
                self.i = (self.v[vx as usize] * 5) as u16;
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
                // Log unrecognized opcodes for debugging
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
    fn get_register(&mut self, reg: u8) -> u8 {
        match reg {
            0x0 => self.v[0],
            0x1 => self.v[1],
            0x2 => self.v[2],
            0x3 => self.v[3],
            0x4 => self.v[4],
            0x5 => self.v[5],
            0x6 => self.v[6],
            0x7 => self.v[7],
            0x8 => self.v[8],
            0x9 => self.v[9],
            0xa => self.v[10],
            0xb => self.v[11],
            0xc => self.v[12],
            0xd => self.v[13],
            0xe => self.v[14],
            0xf => self.v[15],
            _ => {
                panic!("unknown register: V{:X}", reg);
            }
        }
    }
    fn set_register(&mut self, reg: u8, byte: u8) {
        match reg {
            0x0 => self.v[0] = byte,
            0x1 => self.v[1] = byte,
            0x2 => self.v[2] = byte,
            0x3 => self.v[3] = byte,
            0x4 => self.v[4] = byte,
            0x5 => self.v[5] = byte,
            0x6 => self.v[6] = byte,
            0x7 => self.v[7] = byte,
            0x8 => self.v[8] = byte,
            0x9 => self.v[9] = byte,
            0xa => self.v[10] = byte,
            0xb => self.v[11] = byte,
            0xc => self.v[12] = byte,
            0xd => self.v[13] = byte,
            0xe => self.v[14] = byte,
            0xf => self.v[15] = byte,
            _ => {
                panic!("unknown register: V{:X}", reg);
            }
        }
    }

    pub fn render_display(&self) -> String {
        let mut output = String::with_capacity(64 * 32 + 32); // Pre-allocate for pixels + newlines
        for y in 0..32 {
            for x in 0..64 {
                output.push(if self.display[y * 64 + x] == 1 {
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
            // In a real implementation, beep here when sound_timer > 0
        }
    }
}
