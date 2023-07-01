use crate::{Configs, Components};

use emulator_common::GenericTimer;

pub struct CPU
{
    pc: u32,

    halt_flag: bool,

    timer: GenericTimer
}

impl CPU
{
    pub fn new(config: &Configs::CPUConfig) -> Self
    {
        Self {
            pc: 0,
            timer: GenericTimer::new(&config.timer),
            halt_flag: false
        }
    }

    #[inline]
    pub fn halted(&self) -> bool
    {
        return self.halt_flag;
    }
    
    #[inline]
    pub fn step(&mut self, ram: &mut Components::RAM)
    {
        self.pc = ram.read_triple_byte(2);

        for _ in 0..65536
        {
            ram.write_byte(ram.read_triple_byte(self.pc + 3), ram.read_byte(ram.read_triple_byte(self.pc)));
            self.pc = ram.read_triple_byte(self.pc + 6);
        }
    }

    pub fn update(&mut self, ram: &mut Components::RAM, _keyboard: &mut Components::Keyboard, delta: f64)
    {
        self.timer.update(delta);
        
        // We are ready to execute the opcode.
        if self.timer.get() == 0
        {
            self.timer.set(1);
            self.step(ram);
        }
    }

}