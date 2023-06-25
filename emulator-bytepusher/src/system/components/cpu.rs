use crate::{Configs, Components};

pub struct CPU
{
    pc: u16,

    halt_flag: bool,

    vsync: Components::Timer,

    timer: Components::Timer
}

impl CPU
{
    pub fn new(config: &Configs::CPUConfig) -> Self
    {
        Self {
            pc: 0,
            timer: Components::Timer::new(&config.timer),
            vsync: Components::Timer::new(&Configs::TimerConfig {
                rate: 60.0
            }),
            halt_flag: false
        }
    }

    #[inline]
    pub fn halted(&self) -> bool
    {
        return self.halt_flag;
    }
    
    #[inline]
    pub fn step(&mut self, ram: &mut Components::RAM, keyboard: &mut Components::Keyboard, delta: &mut Components::Timer, sound: &mut Components::Timer)
    {
        
    }

    pub fn update(&mut self, ram: &mut Components::RAM, keyboard: &mut Components::Keyboard, delta_timer: &mut Components::Timer, sound_timer: &mut Components::Timer, delta: f64)
    {
        self.timer.update(delta);
        self.vsync.update(delta);
        
        // We are ready to execute the opcode.
        if self.timer.get() == 0
        {
            self.timer.set(1);
            self.step(ram, keyboard, delta_timer, sound_timer);
        }
    }

}