use raylib::{color::Color, consts::KeyboardKey, prelude::*};
use serde_json::Value;

use crate::Emulator;

pub struct RaylibFrontendConfig {}

impl RaylibFrontendConfig
{
	pub fn default() -> Self
	{
		let result = Self {};
		return result;
	}

	pub fn from_json(_data: &Value) -> Self
	{
		let result = Self::default();

		info!("Raylib frontend config loaded successfully from JSON data.");

		return result;
	}
}

pub struct RaylibFrontend
{
	internals: (raylib::RaylibHandle, raylib::RaylibThread),
	output: RenderTexture2D
}

impl RaylibFrontend
{
	pub fn new(_config: &RaylibFrontendConfig) -> Self
	{
		let mut result_internals = raylib::init()
			.size(768, 768)
			.title("remu BytePusher")
			.vsync()
			.build();

		let mut result = Self {
			output: result_internals
				.0
				.load_render_texture(&result_internals.1, 256, 256)
				.unwrap(),
			internals: result_internals
		};

		info!("Raylib frontend initialized successfully.");

		result
			.internals
			.0
			.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));

		return result;
	}

	pub fn update(&mut self, emulator: &mut Emulator, delta: f64) { emulator.update(delta); }

	pub fn draw(&mut self, emulator: &mut Emulator)
	{
		let mut binding = &mut self.internals.0;
		let mut output_context = binding.begin_texture_mode(&self.internals.1, &mut self.output);

		for y in 0 .. emulator.get_display_height()
		{
			for x in 0 .. emulator.get_display_width()
			{
				let pixel = emulator.get_display_pixel(x, y);

				output_context.draw_pixel(
					x as i32,
					(emulator.get_display_height() - (y + 1)) as i32,
					Color::from((
						((pixel & 0xFF0000) >> 16) as u8,
						((pixel & 0xFF00) >> 8) as u8,
						(pixel & 0xFF) as u8,
						0xFF as u8
					))
				);
			}
		}

		drop(output_context);

		let mut screen_context = self.internals.0.begin_drawing(&self.internals.1);

		screen_context.draw_texture_pro(
			&self.output,
			Rectangle::new(0.0, 0.0, 256.0, 256.0),
			Rectangle::new(
				0.0,
				0.0,
				screen_context.get_screen_width() as f32,
				screen_context.get_screen_height() as f32
			),
			Vector2::default(),
			0.0,
			Color::WHITE
		);

		screen_context.draw_fps(0, 0);

		drop(screen_context);
	}

	#[inline]
	pub fn has_quit(&self) -> bool { return self.internals.0.window_should_close(); }
}
