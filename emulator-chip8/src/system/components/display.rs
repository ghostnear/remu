use crate::Configs;

pub struct Display
{
	width: u8,
	height: u8,
	memory: Vec<u8>,
	draw_flag: bool
}

impl Display
{
	pub fn new(config: &Configs::DisplayConfig) -> Self
	{
		let mut result = Self {
			width: config.width,
			height: config.height,
			memory: vec![0; (config.width as usize * config.height as usize) / 8],
			draw_flag: true
		};

		for index in 0 .. (config.width as usize * config.height as usize) / 8
		{
			result.memory[index] = rand::random::<u8>();
		}

		result.clear();

		return result;
	}

	#[inline]
	pub fn clear(&mut self)
	{
		for index in 0 .. self.memory.len()
		{
			self.memory[index] = 0;
		}
	}

	#[inline]
	pub fn get_pixel(&self, x: u8, y: u8) -> bool
	{
		if x >= self.width || y >= self.height
		{
			return false;
		}

		let index = (y as usize * self.width as usize + x as usize) / 8;
		let bit = 7 - (x % 8);
		return (self.memory[index] >> bit) & 1 == 1;
	}

	// Returns true if a collision occurred. Used for setting V[0xF].
	#[inline]
	pub fn set_pixel(&mut self, x: u8, y: u8, value: bool) -> bool
	{
		if x >= self.width || y >= self.height || !value
		{
			return false;
		}

		let index = (y as usize * self.width as usize + x as usize) / 8;
		let bit = 7 - (x % 8);

		let old_value = (self.memory[index] >> bit) & 1 == 1;

		self.memory[index] ^= 1 << bit;

		return old_value;
	}

	#[inline]
	pub fn get_width(&self) -> u8 { return self.width; }

	#[inline]
	pub fn get_height(&self) -> u8 { return self.height; }

	#[inline]
	pub fn set_flag(&mut self) { self.draw_flag = true; }

	#[inline]
	pub fn get_flag(&self) -> bool { return self.draw_flag; }

	#[inline]
	pub fn reset_flag(&mut self) { self.draw_flag = false; }
}
