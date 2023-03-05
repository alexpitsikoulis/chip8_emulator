use relm4::Sender;
use std::{
    thread,
    time::{Duration, SystemTime},
};

use crate::Message;

#[derive(Clone, Copy, Debug)]
pub struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    i: usize,
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        return Self {
            registers: [0; 16],
            program_counter: 0x200,
            i: 0,
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
        };
    }
    pub fn run(
        &mut self,
        memory: &mut [u8; 4096],
        stack: &mut [u16; 16],
        display: &mut [[u8; 128]; 64],
        sender: &Sender<Message>,
    ) {
        loop {
            let op_byte1 = memory[self.program_counter] as u16;
            let op_byte2 = memory[self.program_counter + 1] as u16;
            let opcode = op_byte1 << 8 | op_byte2;

            self.program_counter += 2;

            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let n = (opcode & 0x000F) as u8;
            let kk = (opcode & 0x00FF) as u8;
            let nnn = opcode & 0x0FFF;

            match opcode {
                0x0000 => {
                    sender.emit(Message::ShutDown);
                    return;
                }
                0x00E0 => {
                    Self::clr(sender);
                }
                0x00EE => {
                    self.ret(stack);
                }
                0x1000..=0x1FFF => {
                    self.jmp(nnn);
                }
                0x2000..=0x2FFF => {
                    self.call(nnn, stack);
                }
                0x3000..=0x3FFF => {
                    self.se(x, kk);
                }
                0x4000..=0x4FFF => {
                    self.sne(x, kk);
                }
                0x5000..=0x5FFF => {
                    self.se(x, self.registers[y as usize]);
                }
                0x6000..=0x6FFF => {
                    self.ld(x, kk);
                }
                0x7000..=0x7FFF => {
                    self.add(x, kk);
                }
                0x8000..=0x8FFF => match opcode & 0x000F {
                    0x0 => {
                        self.ld(x, self.registers[y as usize]);
                    }
                    0x1 => {
                        self.or_xy(x, y);
                    }
                    0x2 => {
                        self.and_xy(x, y);
                    }
                    0x3 => {
                        self.xor_xy(x, y);
                    }
                    0x4 => {
                        self.add_xy(x, y);
                    }
                    0x5 => {
                        self.sub_xy(x, y);
                    }
                    0x6 => {
                        self.shr(x);
                    }
                    0x7 => {
                        self.subb(x, y);
                    }
                    0xE => {
                        self.shl(x);
                    }
                    _ => {}
                },
                0x9000..=0x9FF0 => {
                    self.sne(x, self.registers[y as usize]);
                }
                0xA000..=0xAFFF => {
                    self.ld_i(nnn);
                }
                0xB000..=0xBFFF => {
                    self.jmp_0(nnn);
                }
                0xC000..=0xCFFF => {
                    self.rnd(x, kk);
                }
                0xD000..=0xDFFF => self.drw(n, x, y, memory, display, sender),
                0xE09E..=0xEFA1 => {
                    match kk {
                        0x9E => { /* skip if key stored in x is pressed */ }
                        0xA1 => { /* skip if key stored in x is NOT pressed */ }
                        _ => { /* invalid */ }
                    };
                }
                0xF007..=0xFF65 => match kk {
                    0x07 => {
                        self.ld_x_dt(x);
                    }
                    0x0A => { /* wait for key press then store value of key in x */ }
                    0x15 => {
                        self.ld_dt_x(x);
                    }
                    0x18 => {
                        self.ld_st_x(x);
                    }
                    0x1E => {
                        self.add_i(x);
                    }
                    0x29 => {
                        self.ld_f_x(x);
                    }
                    0x33 => {
                        self.ld_b_x(x, memory);
                    }
                    0x55 => {
                        self.ld_0_x_i(x, memory);
                    }
                    0x65 => {
                        self.ld_i_0_x(x, memory);
                    }
                    _ => { /* invalid */ }
                },
                _ => { /* invalid */ }
            }
        }
    }

    fn clr(sender: &Sender<Message>) {
        thread::sleep(Duration::from_secs(1));
        // sender.send(Message::Clr).expect("failed to clear display");
        sender.emit(Message::Clr);
    }

    fn ret(&mut self, stack: &[u16; 16]) {
        if self.stack_pointer == 0 {
            panic!("stack underflow!")
        }
        self.stack_pointer -= 1;
        let call_nnn = stack[self.stack_pointer];
        self.program_counter = call_nnn as usize;
    }

    fn jmp(&mut self, nnn: u16) {
        self.program_counter = nnn as usize;
    }

    fn call(&mut self, nnn: u16, stack: &mut [u16; 16]) {
        let sp = self.stack_pointer;

        if sp > stack.len() {
            panic!("stack overflow!");
        }

        stack[sp] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = nnn as usize;
    }

    fn se(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] == kk {
            self.program_counter += 2;
        }
    }

    fn sne(&mut self, x: u8, kk: u8) {
        if self.registers[x as usize] != kk {
            self.program_counter += 2;
        }
    }

    fn ld(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] = kk;
    }

    fn add(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] += kk;
    }

    fn or_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ | y_;
    }

    fn and_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ & y_;
    }

    fn xor_xy(&mut self, x: u8, y: u8) {
        let x_ = self.registers[x as usize];
        let y_ = self.registers[y as usize];

        self.registers[x as usize] = x_ ^ y_;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, underflow) = arg1.overflowing_sub(arg2);
        self.registers[x as usize] = val;

        if underflow {
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }
    }

    fn shr(&mut self, x: u8) {
        let arg = self.registers[x as usize];

        let lsb = arg & 0x1;

        if lsb == 1 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xf] = 0;
        }

        self.registers[x as usize] = arg >> 1;
    }

    fn subb(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, underflow) = arg1.overflowing_sub(arg2);

        if underflow {
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }

        self.registers[x as usize] = val;
    }

    fn shl(&mut self, x: u8) {
        let arg = self.registers[x as usize];

        let msb = arg & 0x80;

        if msb == 1 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[x as usize] = arg << 1;
    }

    fn ld_i(&mut self, nnn: u16) {
        self.i = nnn as usize;
    }

    fn jmp_0(&mut self, nnn: u16) {
        let v0 = self.registers[0] as usize;
        self.i = (nnn as usize) + v0;
    }

    fn rnd(&mut self, x: u8, kk: u8) {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("failed to get time")
            .as_micros();

        self.registers[x as usize] = (seed as u8) & kk;
    }

    fn drw(
        &mut self,
        n: u8,
        x: u8,
        y: u8,
        memory: &[u8; 0x1000],
        display: &mut [[u8; 128]; 64],
        sender: &Sender<Message>,
    ) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let bytes = &memory[self.i..self.i + n as usize];
        let collision = false;
        for c in 0..n {
            let mut row = arg2 + c;
            if row >= 64 {
                self.registers[0xF] = 1;
                row = row % 64
            }
            for pl in 0..8 {
                let mut col = arg1 + pl;
                let bit = (bytes[c as usize] >> (7 - pl)) & 0x1;
                if col >= 128 {
                    self.registers[0xF] = 1;
                    col = col % 128
                }
                let curr = &mut display[row as usize][col as usize];
                if *curr & bit == 1 {
                    self.registers[0xF] = 1;
                }
                *curr = *curr ^ bit;
                sender.emit(Message::Drw(row, col, bit));
            }
        }
        if !collision {
            self.registers[0xF] = 0;
        }
    }

    fn ld_x_dt(&mut self, x: u8) {
        self.registers[x as usize] = self.delay_timer;
    }

    fn ld_dt_x(&mut self, x: u8) {
        self.delay_timer = self.registers[x as usize];
    }

    fn ld_st_x(&mut self, x: u8) {
        self.sound_timer = self.registers[x as usize];
    }

    fn add_i(&mut self, x: u8) {
        self.i += self.registers[x as usize] as usize;
    }

    fn ld_f_x(&mut self, x: u8) {
        self.i = (self.registers[x as usize] * 5) as usize;
    }

    fn ld_b_x(&mut self, x: u8, memory: &mut [u8]) {
        let mut arg = self.registers[x as usize];
        for pl in 0..3 {
            let mag = u8::pow(10, 2 - pl as u32);
            memory[self.i + pl] = arg / mag;
            arg %= mag;
        }
    }

    fn ld_0_x_i(&mut self, x: u8, memory: &mut [u8]) {
        for reg in 0..=x {
            memory[self.i + reg as usize] = self.registers[reg as usize];
        }
    }

    fn ld_i_0_x(&mut self, x: u8, memory: &mut [u8]) {
        for reg in 0..=x {
            self.registers[reg as usize] = memory[self.i + reg as usize];
        }
    }
}
