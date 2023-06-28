use raylib::{consts::KeyboardKey, prelude::*, color::Color};

use serde_json::Value;

use crate::Emulator;

pub struct RaylibFrontendConfig
{
}

impl RaylibFrontendConfig
{
    pub fn default() -> Self
    {
        let result = Self {};
        return result;
    }

    pub fn from_json(data: &Value) -> Self
    {
        let result = Self::default();

        info!("Raylib frontend config loaded successfully from JSON data.");

        return result;
    }
}

pub struct RaylibFrontend
{
    internals: (raylib::RaylibHandle, raylib::RaylibThread)
}

impl RaylibFrontend
{
    pub fn new(config: &RaylibFrontendConfig) -> Self
    {
        let mut result = Self {
            internals: raylib::init().size(1080, 580).title("remu BytePusher").build(),
        };

        info!("Raylib frontend initialized successfully.");

        result.internals.0.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));

        return result;
    }

    pub fn update(&mut self, emulator: &mut Emulator, _delta: f64)
    {
        // TODO: updating here.
    }

    pub fn draw(&mut self, emulator: &mut Emulator)
    {
        let mut drawing_context = self.internals.0.begin_drawing(&self.internals.1);

        drawing_context.clear_background(Color::BLACK);
    }

    #[inline]
    pub fn has_quit(&self) -> bool
    {
        return self.internals.0.window_should_close();
    }
}