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
    ClearScreen,
    Return,
    Jump { address: u16 },
    Unknown { opcode: u16 },
}

impl Instruction {
    pub fn from_opcode(opcode: u16) -> Self {
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0x00E0 => Instruction::ClearScreen,
                0x00EE => Instruction::Return,
                _ => Instruction::Unknown { opcode },
            },
            0x1000 => Instruction::Jump {
                address: opcode & 0x0FFF,
            },
            _ => Instruction::Unknown { opcode },
        }
    }
}
