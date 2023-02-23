const FONT: [u8; 0x50] = [
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

#[derive(Clone)]
pub struct RAM {
    pub memory: [u8; 0x1000],
    pub stack: [u16; 16],
    last_program_end: usize,
    last_sprite_end: usize,
}

impl RAM {
    pub fn new() -> Self {
        let mut memory = Self {
            memory: [0; 0x1000],
            stack: [0; 16],
            last_program_end: 0x200,
            last_sprite_end: 0,
        };
        memory.load_font();
        return memory;
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        let len = program.len();
        for i in 0..len {
            self.memory[i + self.last_program_end] = program[i];
        }
        self.last_program_end = self.last_program_end + len;
    }

    // pub fn load_sprite(&mut self, sprite: Vec<u8>) {
    //     for i in 0..sprite.len() {
    //         self.memory[i + self.last_sprite_end] = sprite[i];
    //     }
    // }

    fn load_font(&mut self) {
        let len = FONT.len();
        for i in 0..len {
            self.memory[i] = FONT[i];
        }
        self.last_sprite_end = len;
    }
}
