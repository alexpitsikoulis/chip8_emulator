mod cpu;
mod memory;

use crate::Message;
use std::sync::mpsc::Receiver;

#[derive(Clone)]
pub struct Emulator {
    cpu: cpu::CPU,
    memory: memory::RAM,
}

impl Emulator {
    pub fn new() -> Self {
        return Self {
            cpu: cpu::CPU::new(),
            memory: memory::RAM::new(),
        };
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory.load_program(program);
    }

    pub fn start(
        &mut self,
        display: &mut [[u8; 128]; 64],
        keyboard_receiver: Receiver<u16>,
        sender: &relm4::Sender<Message>,
    ) {
        self.cpu.run(
            &mut self.memory.memory,
            &mut self.memory.stack,
            display,
            &keyboard_receiver,
            sender,
        );
    }
}
