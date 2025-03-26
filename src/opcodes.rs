// opcodes are 16 bits (2 bytes).
// each hex digit represents 4 bits or a "nibble"
// 16-bit opcode has 4 hex digits 0x1ABC
// first nibble indicates the instruction type
// & to compare bit by bit
//  0xF000 = 1111 0000 0000 0000  Isolate first nibble
//  0x00FF = 0000 0000 1111 1111 Isolate lower 8 bits
//  0x0FFF = 0000 1111 1111 1111 Isolates lower 12 bits
//

#[derive(Debug)]
pub enum Instruction {
    ClearScreen,           // 00E0
    Return,                //00EE returns from subroutine by popping from the stack
    Jump { address: u16 }, //1nnn sets PC to address nnn
    Call { address: u16 }, //2nnn calls subroutine at address nnn pushes current pc to
    //stack
    SkipEq { vx: u8, byte: u8 }, //3xkk skips next instruction if register Vx equals kk
    SkipNe { vx: u8, byte: u8 }, //4xkk skips next instruction if Vx != kk
    SkipEqReg { vx: u8, vy: u8 }, //5xy0 skips next inst. if Vx == Vy
    LoadByte { vx: u8, byte: u8 }, //6xkk loads immed. value kk into register Vx
    AddByte { vx: u8, byte: u8 }, //7xkk + immediate value kk to Vx without carry flag
    LoadReg { vx: u8, vy: u8 },  //8xy0 copies val at Vy into Vx
    Or { vx: u8, vy: u8 },       //8xy1 performs bitwise OR between Vx and Vy, stores in Vx
    And { vx: u8, vy: u8 },      //8xy2 bitwise AND between Vx and Vy, stores in Vx
    Xor { vx: u8, vy: u8 },      //8xy3 bitwise XOR between Vx and Vy stores in Vx
    AddReg { vx: u8, vy: u8 },   //8xy4 adds Vy to vx sets VF to 1 if carry occurs
    Sub { vx: u8, vy: u8 },      //8xy5 - Vy from Vx sets VF to 1 if no borrow
    Shr { vx: u8 },              //8xy6 shits Vx right by 1 VF gets LSB
    SubN { vx: u8, vy: u8 },     //8xy7 sets Vx to Vy - Vx, VF is 1 if no borrow Shl { vx: u8 },
    Shl { vx: u8 },              // 8xyE: Shifts Vx left by 1, VF gets the most significant bit
    SkipNeReg { vx: u8, vy: u8 }, //9xy0 skips next instruct. if Vx != Vy
    LoadI { address: u16 },      //Annn sets index register I to address nnn
    JumpV0 { address: u16 },     //Bnnn jumps to nnn + value in V0
    Random { vx: u8, byte: u8 }, // Cxkk sets Vx to a random byte ANDed with kk
    Draw { vx: u8, vy: u8, n: u8 }, //Dxyn Draws an n-byte spirt at (Vx, Vy), VF set on
    //collision
    SkipKey { vx: u8 },   //Ex9E skips next instruc if key in Vx is pressed
    SkipNoKey { vx: u8 }, //ExA1 skips next instruc. if key in Vx is not pressed
    LoadDelay { vx: u8 }, //Fx07 loads delay timer to value in Vx
    WaitKey { vx: u8 },   //Fx0A waits for key press stores value in Vx
    SetDelay { vx: u8 },  //Fx15 sets dealy timer to value in Vx
    SetSound { vx: u8 },  //Fx18 sets sound timer to value in Vx
    AddI { vx: u8 },      //Fx1E adds Vx to I
    LoadFont { vx: u8 },  //Fx29 Sets I to mem address of the font sprite for digit
    //Vx
    StoreBCD { vx: u8 },  //Fx33 stores binary decimal of Vx at I, I+1 I+2
    StoreRegs { vx: u8 }, //Fx55 stores reg V0 through Vx into mem starting at I
    LoadRegs { vx: u8 },  //Fx65 Loads reg V0 through Vx from Mem starting at I
    Unknown { opcode: u16 },
}

impl Instruction {
    pub fn from_opcode(opcode: u16) -> Self {
        //extract first nibble to get instruction family
        let first_nibble = opcode & 0xF000;

        let x = ((opcode & 0x0F00) >> 8) as u8; //second nib = index of register Vx (0-F)
        let y = ((opcode & 0x00F0) >> 4) as u8; //third nib index of register Vy (0-F)
        let n = (opcode & 0x000F) as u8; //fourth nib immed value
        let nnn = opcode & 0x0FFF; // last 12 bits, = address
        let kk = (opcode & 0x00FF) as u8; // 8 bit imed value last nib

        match first_nibble {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => Instruction::ClearScreen,
                0x00EE => Instruction::Return,
                _ => Instruction::Unknown { opcode },
            },
            0x1000 => Instruction::Jump { address: nnn }, // jump to add nnn
            0x2000 => Instruction::Call { address: nnn }, // call sub at nn
            0x3000 => Instruction::SkipEq { vx: x, byte: kk }, //skip if Vx == kk
            0x4000 => Instruction::SkipNe { vx: x, byte: kk }, //skip if Vx != kk
            0x5000 => {
                // requires last nibble to be 0
                if n == 0 {
                    Instruction::SkipEqReg { vx: x, vy: y }
                } else {
                    Instruction::Unknown { opcode }
                }
            }
            0x6000 => Instruction::LoadByte { vx: x, byte: kk }, //load kk into Vx
            0x7000 => Instruction::AddByte { vx: x, byte: kk },  //add kk to Vx
            0x8000 => {
                // 0x8 are alu ops distinguished by last nib
                match n {
                    0x0 => Instruction::LoadReg { vx: x, vy: y }, // Vx = Vy
                    0x1 => Instruction::Or { vx: x, vy: y },      // Vx = Vx | Vy
                    0x2 => Instruction::And { vx: x, vy: y },     // Vx = Vx & Vy
                    0x3 => Instruction::Xor { vx: x, vy: y },     // Vx = Vx ^ Vy
                    0x4 => Instruction::AddReg { vx: x, vy: y },  // Vx = Vx + Vy
                    // (with carry)
                    0x5 => Instruction::Sub { vx: x, vy: y }, // Vx = Vx - Vy
                    // (with borrow)
                    0x6 => Instruction::Shr { vx: x }, // Vx = Vx >> i
                    0x7 => Instruction::SubN { vx: x, vy: y }, // Vx = Vy - Vx
                    // (with brrow)
                    0xE => Instruction::Shl { vx: x }, // Vx = Vx << i
                    _ => Instruction::Unknown { opcode },
                }
            }
            0x9000 => {
                // requires last nibble to be 0
                if n == 0 {
                    Instruction::SkipNeReg { vx: x, vy: y }
                } else {
                    Instruction::Unknown { opcode }
                }
            }
            0xA000 => Instruction::LoadI { address: nnn }, // I = nnn
            0xB000 => Instruction::JumpV0 { address: nnn }, // PC = nnn + V0
            0xC000 => Instruction::Random { vx: x, byte: kk }, // Vx = rand() & kk
            0xD000 => Instruction::Draw { vx: x, vy: y, n }, // Draw spite at (Vx, Vy)
            0xE000 => {
                match opcode & 0x00FF {
                    0x009E => Instruction::SkipKey { vx: x }, // skip if Vx
                    // pressed
                    0x00A1 => Instruction::SkipNoKey { vx: x }, // skip if key
                    // Vx not pressed
                    _ => Instruction::Unknown { opcode },
                }
            }
            0xF000 => {
                // Fxxx opcodes are misc, and identified by last byte
                match opcode & 0x00FF {
                    0x0007 => Instruction::LoadDelay { vx: x }, // Vx = delay timer
                    0x000A => Instruction::WaitKey { vx: x },   // wait for key, store in Vx
                    0x0015 => Instruction::SetDelay { vx: x },  // delay timer = Vx
                    0x0018 => Instruction::SetSound { vx: x },  // sound timer = Vx
                    0x001E => Instruction::AddI { vx: x },      // I = I + Vx
                    0x0029 => Instruction::LoadFont { vx: x },  // I = font sprite for Vx
                    0x0033 => Instruction::StoreBCD { vx: x },  // store BCD of Vx
                    0x0055 => Instruction::StoreRegs { vx: x }, // store V0-Vx in memory
                    0x0065 => Instruction::LoadRegs { vx: x },  //load V0-Vx from memory
                    _ => Instruction::Unknown { opcode },
                }
            }
            _ => Instruction::Unknown { opcode },
        }
    }
}
