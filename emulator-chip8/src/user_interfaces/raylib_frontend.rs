use std::mem::transmute;

use raylib::{consts::KeyboardKey, prelude::*};
use serde_json::Value;

use crate::Emulator;

pub struct RaylibFrontendConfig
{
	pub foreground: Color,
	pub background: Color,
	pub bindings: Vec<KeyboardKey>
}

impl RaylibFrontendConfig
{
	pub fn default() -> Self
	{
		let mut result = Self {
			foreground: Color {
				r: 255,
				g: 255,
				b: 255,
				a: 255
			},
			background: Color {
				r: 0,
				g: 0,
				b: 0,
				a: 255
			},
			bindings: Vec::new()
		};

		// Make sure the bindings are not set.
		for _ in 0 .. 0x10
		{
			result.bindings.push(KeyboardKey::KEY_NULL);
		}

		return result;
	}

	pub fn from_json(data: &Value) -> Self
	{
		let mut result = Self::default();

		// Change the defaults if they are changed in the config.
		result.foreground = Color {
			r: data["foreground"]["r"].as_u64().unwrap_or(255) as u8,
			g: data["foreground"]["g"].as_u64().unwrap_or(255) as u8,
			b: data["foreground"]["b"].as_u64().unwrap_or(255) as u8,
			a: 255
		};
		result.background = Color {
			r: data["background"]["r"].as_u64().unwrap_or(0) as u8,
			g: data["background"]["g"].as_u64().unwrap_or(0) as u8,
			b: data["background"]["b"].as_u64().unwrap_or(0) as u8,
			a: 255
		};

		// Set the bindings.
		let keys = data["keys"].as_array().unwrap();

		for index in 0 .. 0x10
		{
			let key = keys[index]
				.as_str()
				.unwrap_or("")
				.chars()
				.next()
				.unwrap_or('\0')
				.to_ascii_uppercase();
			result.bindings[index] = unsafe { transmute(key as u32) };
		}

		info!("Raylib frontend config loaded successfully from JSON data.");

		return result;
	}
}

pub struct RaylibFrontend
{
	foreground: Color,
	background: Color,
	output: RenderTexture2D,
	internals: (raylib::RaylibHandle, raylib::RaylibThread),
	bindings: Vec<KeyboardKey>
}

impl RaylibFrontend
{
	pub fn new(config: &RaylibFrontendConfig) -> Self
	{
		let mut result_internals = raylib::init().size(1080, 580).title("remu CHIP8").build();

		let mut result = Self {
			foreground: config.foreground,
			background: config.background,
			output: result_internals
				.0
				.load_render_texture(&result_internals.1, 64, 32)
				.unwrap(),
			internals: result_internals,
			bindings: config.bindings.clone()
		};

		info!("Raylib CHIP8 frontend initialized successfully.");

		result
			.internals
			.0
			.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));

		return result;
	}

	pub fn update(&mut self, emulator: &mut Emulator, _delta: f64)
	{
		for index in 0 .. 0x10 as u8
		{
			if self.internals.0.is_key_down(self.bindings[index as usize])
			{
				emulator.press_key(index);
			}
			else
			{
				emulator.release_key(index);
			}
		}
	}

	pub fn draw(&mut self, emulator: &mut Emulator)
	{
		if !emulator.get_draw_flag()
		{
			return;
		}

		let mut binding = &mut self.internals.0;
		let mut output_context = binding.begin_texture_mode(&self.internals.1, &mut self.output);

		for y in 0 .. emulator.get_display_height()
		{
			for x in 0 .. emulator.get_display_width()
			{
				let mut resulting_color = self.background;

				if emulator.get_display_pixel(x, y)
				{
					resulting_color = self.foreground;
				}

				output_context.draw_pixel(
					x as i32,
					(emulator.get_display_height() - (y + 1)) as i32,
					resulting_color
				);
			}
		}

		drop(output_context);

		let mut screen_context = self.internals.0.begin_drawing(&self.internals.1);

		screen_context.draw_texture_pro(
			&self.output,
			Rectangle::new(0.0, 0.0, 64.0, 32.0),
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

		drop(screen_context);
	}

	#[inline]
	pub fn has_quit(&self) -> bool { return self.internals.0.window_should_close(); }
}
