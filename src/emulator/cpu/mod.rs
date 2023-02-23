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
        display: &mut [[u8; 256]; 128],
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
                0xE09E..=0xEFA1 => match kk {
                    0x9E => { /* skip if key stored in Vx is pressed */ }
                    0xA1 => { /* skip if key stored in Vx is NOT pressed */ }
                    _ => { /* invalid */ }
                },
                0xF007..=0xFF65 => match kk {
                    0x07 => { /* set Vx = delay timer */ }
                    0x0A => { /* wait for key press then store value of key in Vx */ }
                    0x15 => { /* set delay timer = Vx */ }
                    0x18 => { /* set sound timer = Vx */ }
                    0x1E => { /* set I = I + Vx */ }
                    0x29 => { /* set I = location for sprite representing value in Vx */ }
                    0x33 => { /* The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2. */
                    }
                    0x55 => { /* store values in registers V0 through Vx in memory starting at I */
                    }
                    0x65 => { /* read values from memory starting at I into V0 through Vx */ }
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
        display: &mut [[u8; 256]; 128],
        sender: &Sender<Message>,
    ) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let bytes = &memory[self.i..self.i + n as usize];
        let collision = false;
        for c in 0..n {
            let row = arg2 + c;
            if !collision && row >= 128 {
                self.registers[0xF] = 1;
            }
            for pl in 0..8 {
                let col = arg1 + pl;
                let curr = &mut display[(row % 128) as usize][col as usize];
                let bit = (bytes[c as usize] >> (7 - pl)) & 0x1;
                if !collision && *curr & bit == 1 {
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
}
