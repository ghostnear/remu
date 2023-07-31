mod system;
pub use system::components as Components;
pub use system::configs as Configs;

mod user_interfaces;
use std::io::Read;

use emulator_common::GenericDownTimer;
pub use user_interfaces::*;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub struct Emulator
{
	cpu: Components::CPU,
	ram: Components::RAM,
	display: Components::Display,
	keyboard: Components::Keyboard,
	sound: GenericDownTimer,
	delta: GenericDownTimer
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
			sound: GenericDownTimer::new(&config.sound_timer_config),
			delta: GenericDownTimer::new(&config.delta_timer_config)
		};
		result.cpu.set_pc(config.ram_config.start as u16);
		return result;
	}

	#[inline]
	pub fn get_sound_timer(&self) -> u8 { return self.sound.get() as u8; }

	#[inline]
	pub fn get_draw_flag(&self) -> bool { self.display.get_flag() }

	#[inline]
	pub fn reset_draw_flag(&mut self) { self.display.reset_flag(); }

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
	pub fn is_running(&self) -> bool { return !self.cpu.halted(); }

	#[inline]
	pub fn load(&mut self, path: &str)
	{
		let mut rom = std::fs::File::open(path).expect("Unable to open ROM for loading!");

		let mut buffer: Vec<u8> = Vec::new();
		rom.read_to_end(&mut buffer).unwrap();
		info!("Read ROM from path: {}", path);

		self.ram.load_rom_data(&mut buffer);
	}

	#[inline]
	pub fn get_display_width(&self) -> u8 { return self.display.get_width(); }

	#[inline]
	pub fn get_display_height(&self) -> u8 { return self.display.get_height(); }

	#[inline]
	pub fn get_display_pixel(&self, x: u8, y: u8) -> bool { return self.display.get_pixel(x, y); }

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

		self.cpu.update(
			&mut self.ram,
			&mut self.display,
			&mut self.keyboard,
			&mut self.delta,
			&mut self.sound,
			delta
		);
	}
}
