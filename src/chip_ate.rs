#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct ChipAte {
    //4Kb of memory 1000 memory locations
    memory: [u8; 4096],
    // 16 general purpose registers
    v: [u8; 16],
    //index register
    i: u16,
    //program coutner
    pc: u16,
    //stack
    stack: [u16; 16],
    //stack pointer
    sp: u8,

    //display 64 x 32 pixels
    display: [u8; 64 * 32],

    delay_timer: u8,
    sound_timer: u8,
    //hex keypad state
    keypad: [u8; 16],
}

impl ChipAte {
    pub fn new(self) -> Self {
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
    fn fetch(&mut self) {}
}
