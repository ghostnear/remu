mod system;
pub use system::configs as Configs;
pub use system::components as Components;

use std::io::Read;

mod user_interfaces;
pub use user_interfaces::{RaylibFrontend, RaylibFrontendConfig};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub struct Emulator
{
    cpu: Components::CPU,
    ram: Components::RAM,
    keyboard: Components::Keyboard
}

impl Emulator
{
    pub fn new(config: &Configs::EmulatorConfig) -> Self
    {
        Self {
            cpu: Components::CPU::new(&config.cpu_config),
            ram: Components::RAM::new(&config.ram_config),
            keyboard: Components::Keyboard::new()
        }
    }

    #[inline]
    pub fn press_key(&mut self, key: u8)
    {
        if key >= 16
        {
            error!("Invalid key: {}", key);
            panic!("Invalid key: {}", key);
        }

        self.keyboard.press(key);
    }

    #[inline]
    pub fn release_key(&mut self, key: u8)
    {
        if key >= 16
        {
            error!("Invalid key: {}", key);
            panic!("Invalid key: {}", key);
        }

        self.keyboard.release(key);
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
        info!("Read ROM from path: {}", path);

        self.ram.load_rom_data(&mut buffer);
    }

    #[inline]
    pub fn get_display_width(&self) -> usize
    {
        return 256;
    }

    #[inline]
    pub fn get_display_height(&self) -> usize
    {
        return 256;
    }

    #[inline]
    pub fn get_display_pixel(&self, x: usize, y: usize) -> u32
    {
        let address = ((self.ram.read_byte(5) as usize) << 16) | (y << 8) as usize | x as usize;

        return self.ram.get_color_value(self.ram.read_byte(address as u32));
    }

    // Updates the emulator state by the given ammount of seconds.
    #[inline]
    pub fn update(&mut self, delta: f64)
    {
        if self.cpu.halted()
        {
            return;
        }
        self.cpu.update(&mut self.ram, &mut self.keyboard, delta);
    }

}