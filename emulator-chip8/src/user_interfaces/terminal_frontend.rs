use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::{cursor, event::*, execute, queue, style::*, terminal::*};
use serde_json::Value;

use crate::Emulator;

pub struct TerminalFrontendConfig
{
	pub foreground: Color,
	pub background: Color,
	pub bindings: Vec<(KeyCode, KeyModifiers)>
}

impl TerminalFrontendConfig
{
	pub fn default() -> Self
	{
		let mut result = Self {
			foreground: Color::White,
			background: Color::Black,
			bindings: Vec::new()
		};

		// Make sure the bindings are not set.
		for _ in 0 .. 0x10
		{
			result
				.bindings
				.push((KeyCode::Char('\0'), KeyModifiers::NONE));
		}

		return result;
	}

	pub fn from_json(data: &Value) -> Self
	{
		let mut result = Self::default();

		// Change the defaults if they are changed in the config.
		result.foreground = Color::Rgb {
			r: data["foreground"]["r"].as_u64().unwrap_or(255) as u8,
			g: data["foreground"]["g"].as_u64().unwrap_or(255) as u8,
			b: data["foreground"]["b"].as_u64().unwrap_or(255) as u8
		};
		result.background = Color::Rgb {
			r: data["background"]["r"].as_u64().unwrap_or(0) as u8,
			g: data["background"]["g"].as_u64().unwrap_or(0) as u8,
			b: data["background"]["b"].as_u64().unwrap_or(0) as u8
		};

		// Set the bindings.
		let keys = data["keys"].as_array().unwrap();
		let modifiers = data["keys_modifiers"].as_array().unwrap();

		for index in 0 .. 0x10
		{
			result.bindings[index] = (
				KeyCode::Char(
					keys[index]
						.as_str()
						.unwrap_or("")
						.chars()
						.next()
						.unwrap_or('\0')
				),
				match modifiers[index].as_str().unwrap_or("NONE")
				{
					"NONE" => KeyModifiers::NONE,
					"SHIFT" => KeyModifiers::SHIFT,
					"CONTROL" => KeyModifiers::CONTROL,
					"ALT" => KeyModifiers::ALT,
					_ => KeyModifiers::NONE
				}
			);
		}

		info!("Terminal frontend config loaded successfully from JSON data.");

		return result;
	}
}

pub struct TerminalFrontend
{
	foreground: Color,
	background: Color,
	quit: bool,
	bindings: Vec<(KeyCode, KeyModifiers)>
}

impl TerminalFrontend
{
	pub fn new(config: &TerminalFrontendConfig) -> Self
	{
		execute!(
			stdout(),
			Clear(ClearType::All),
			cursor::Hide,
			PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES),
			SetTitle("remu CHIP-8")
		)
		.unwrap();

		enable_raw_mode().unwrap();

		info!("Terminal CHIP8 frontend initialized successfully.");

		Self {
			foreground: config.foreground,
			background: config.background,
			quit: false,
			bindings: config.bindings.clone()
		}
	}

	fn exit(&mut self)
	{
		self.quit = true;

		disable_raw_mode().unwrap();

		execute!(stdout(), cursor::Show, PopKeyboardEnhancementFlags,).unwrap();
	}

	pub fn update(&mut self, emulator: &mut Emulator, _delta: f64)
	{
		if emulator.get_sound_timer() > 0
		{
			print!("{}", '\x07');
		}

		if poll(Duration::from_millis(1)).unwrap()
		{
			// Read the input.
			match read().unwrap()
			{
				Event::Key(event) =>
				{
					// CTRL + C / CTRL + Z / Escape to close the app.
					if (event.modifiers == KeyModifiers::CONTROL
						&& (event.code == KeyCode::Char('c') || event.code == KeyCode::Char('z')))
						|| event.code == KeyCode::Esc
					{
						info!("Closing on user request...");
						self.exit();
						return;
					}

					for index in 0 .. 0x10
					{
						if (event.code, event.modifiers) == self.bindings[index]
						{
							if event.kind == KeyEventKind::Press
							{
								emulator.press_key(index as u8);
							}
							else
							{
								emulator.release_key(index as u8)
							}
						}
					}
				},
				_ =>
				{}
			}
		}
	}

	pub fn draw(&mut self, emulator: &mut Emulator)
	{
		if !emulator.get_draw_flag() || self.quit
		{
			return;
		}

		emulator.reset_draw_flag();

		queue!(
			stdout(),
			cursor::MoveTo(0, 0),
			SetForegroundColor(self.foreground),
			SetBackgroundColor(self.background)
		)
		.unwrap();

		// Start building each line.
		for y in 0 .. emulator.get_display_height()
		{
			// Start building the line.
			let mut line = String::new();

			// Build each line.
			for x in 0 .. emulator.get_display_width()
			{
				let pixel = emulator.get_display_pixel(x, y);

				if !pixel
				{
					line.push(' ');
				}
				else
				{
					line.push('█');
				}
			}

			// Print the line.
			queue!(stdout(), Print(line), cursor::MoveToNextLine(1)).unwrap();
		}

		queue!(stdout(), ResetColor).unwrap();

		stdout().flush().unwrap();
	}

	#[inline]
	pub fn has_quit(&self) -> bool { return self.quit; }
}
