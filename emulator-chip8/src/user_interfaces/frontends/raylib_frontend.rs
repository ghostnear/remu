use std::mem::transmute;
use raylib::{prelude::*, consts::KeyboardKey};

use crate::Frontend;

use serde_json::Value;

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
            foreground: Color { r: 255, g: 255, b: 255, a: 255 },
            background: Color { r: 0, g: 0, b: 0, a: 255 },
            bindings: Vec::new()
        };

        // Make sure the bindings are not set.
        for _ in 0..0x10
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

        for index in 0..0x10
        {
            // TODO: find a better way to do this, it feels disgusting.
            let key = keys[index].as_str().unwrap_or("").chars().next().unwrap_or('\0').to_ascii_uppercase();
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
    internals: (raylib::RaylibHandle, raylib::RaylibThread),
    quit: bool,
    bindings: Vec<KeyboardKey>
}

impl RaylibFrontend
{
    pub fn new(config: &RaylibFrontendConfig) -> Self
    {
        let mut result =Self {
            foreground: config.foreground,
            background: config.background,
            quit: false,
            internals: raylib::init().size(1080, 580).title("remu CHIP8").build(),
            bindings: config.bindings.clone()
        };

        info!("Raylib frontend initialized successfully.");

        result.internals.0.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));

        return result;
    }

    fn exit(&mut self)
    {
        self.quit = true;
    }
}

impl Frontend for RaylibFrontend
{
    fn update(&mut self, emulator:&mut crate::Emulator, _delta: f64)
    {
        if !emulator.is_running()
        {
            info!("Backend has stopped running, closing app...");
            self.exit();
            return;
        }

        for index in 0..0x10 as u8
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

    fn draw(&mut self, emulator:&mut crate::Emulator)
    {
        if !emulator.get_draw_flag() || self.quit
        {
            return;
        }

        let mut drawing_context = self.internals.0.begin_drawing(&self.internals.1);

        drawing_context.clear_background(self.background);

        let scale = (
            drawing_context.get_screen_width() as f32 / emulator.get_display_width() as f32,
            drawing_context.get_screen_height() as f32 / emulator.get_display_height() as f32
        );

        for y in 0..emulator.get_display_height()
        {
            for x in 0..emulator.get_display_width()
            {
                if emulator.get_display_pixel(x, y)
                {
                    drawing_context.draw_rectangle(
                        (x as f32 * scale.0) as i32,
                        (y as f32 * scale.1) as i32,
                        scale.0 as i32 + 1,
                        scale.1 as i32 + 1,
                        self.foreground
                    );
                }
            }
        }
    }

    #[inline]
    fn has_quit(&self) -> bool
    {
        return self.quit && !self.internals.0.window_should_close();
    }
}