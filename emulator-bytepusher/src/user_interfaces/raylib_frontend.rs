use raylib::consts::KeyboardKey;

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
    internals: (raylib::RaylibHandle, raylib::RaylibThread),
    quit: bool
}

impl RaylibFrontend
{
    pub fn new(config: &RaylibFrontendConfig) -> Self
    {
        let mut result = Self {
            quit: false,
            internals: raylib::init().size(1080, 580).title("remu BytePusher").build(),
        };

        info!("Raylib frontend initialized successfully.");

        result.internals.0.set_exit_key(Some(KeyboardKey::KEY_ESCAPE));

        return result;
    }

    fn exit(&mut self)
    {
        self.quit = true;
    }

    pub fn update(&mut self, emulator: &mut Emulator, _delta: f64)
    {
        if !emulator.is_running()
        {
            info!("Backend has stopped running, closing app...");
            self.exit();
            return;
        }

        // TODO: updating here.
    }

    pub fn draw(&mut self, emulator: &mut Emulator)
    {
        if self.quit
        {
            return;
        }

        let drawing_context = self.internals.0.begin_drawing(&self.internals.1);

        // TODO: drawing here.
    }

    #[inline]
    pub fn has_quit(&self) -> bool
    {
        return self.quit && !self.internals.0.window_should_close();
    }
}