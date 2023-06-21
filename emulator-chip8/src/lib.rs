mod system;
pub use crate::system::configs as Configs;
pub use crate::system::components as Components;

mod user_interfaces;
pub use crate::user_interfaces::Frontend as Frontend;
pub use crate::user_interfaces::Frontends as Frontends;

use std::io::Read;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub struct Emulator
{
    cpu: Components::CPU,
    ram: Components::RAM,
    display: Components::Display,
    keyboard: Components::Keyboard,
    sound: Components::Timer,
    delta: Components::Timer,
}

impl Emulator
{
    pub fn new(config: &Configs::EmulatorConfig) -> Self
    {
        let mut result = Self {
            cpu: Components::CPU::new(&config.cpu_config),
            ram: Components::RAM::new(&config.ram_config),
            display: Components::Display::new(&config.display_config),
            keyboard: Components::Keyboard::new(),
            sound: Components::Timer::new(&config.sound_timer_config),
            delta: Components::Timer::new(&config.delta_timer_config)
        };
        result.cpu.set_pc(config.ram_config.start as u16);
        return result;
    }

    #[inline]
    pub fn get_draw_flag(&self) -> bool
    {
        self.display.get_flag()
    }

    #[inline]
    pub fn reset_draw_flag(&mut self)
    {
        self.display.reset_flag();
    }

    #[inline]
    pub fn is_running(&self) -> bool
    {
        return !self.cpu.halted();
    }

    #[inline]
    pub fn load(&mut self, path: &str)
    {
        let mut rom = std::fs::File::open(path).expect("Unable to open ROM for loading!");
        let mut buffer : Vec<u8> = Vec::new();
        rom.read_to_end(&mut buffer).unwrap();
        self.ram.load_rom_data(&mut buffer);
    }

    #[inline]
    pub fn get_display_width(&self) -> u8
    {
        return self.display.get_width();
    }

    #[inline]
    pub fn get_display_height(&self) -> u8
    {
        return self.display.get_height();
    }

    #[inline]
    pub fn get_display_pixel(&self, x: u8, y: u8) -> bool
    {
        return self.display.get_pixel(x, y);
    }

    // Updates the emulator state by the given ammount of seconds.
    #[inline]
    pub fn update(&mut self, delta: f64)
    {
        if self.cpu.halted()
        {
            return;
        }

        self.sound.update(delta);
        self.delta.update(delta);
        
        self.cpu.update(&mut self.ram, &mut self.display, delta);
    }

}