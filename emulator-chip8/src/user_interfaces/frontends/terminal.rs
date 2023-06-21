use std::io::{stdout};

use crate::Frontend as Frontend;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    cursor,
};

pub struct TerminalFrontendConfig
{
    pub foreground: Color,
    pub background: Color
}

impl TerminalFrontendConfig
{
    pub fn default() -> Self
    {
        Self {
            foreground: Color::White,
            background: Color::Black
        }
    }
}

pub struct TerminalFrontend
{
    foreground: Color,
    background: Color,
    quit: bool
}

impl TerminalFrontend
{
    pub fn new(config: &TerminalFrontendConfig) -> Self
    {
        execute!(
            stdout(),
            Clear(ClearType::All),
            cursor::Hide
        ).unwrap();

        Self {
            foreground: config.foreground,
            background: config.background,
            quit: false
        }
    }
}

impl Frontend for TerminalFrontend
{
    fn update(&mut self, emulator:&crate::Emulator, _delta: f64)
    {
        if !emulator.is_running()
        {
            info!("Backend has stopped running, closing app...");
            self.quit = true;

            execute!(
                stdout(),
                cursor::Show
            ).unwrap();
        }
    }

    fn draw(&mut self, emulator:&mut crate::Emulator)
    {
        if !emulator.get_draw_flag()
        {
            return;
        }

        emulator.reset_draw_flag();

        execute!(
            stdout(),
            cursor::MoveTo(0, 0),
            SetForegroundColor(self.foreground),
            SetBackgroundColor(self.background)
        ).unwrap();

        // Start building each line.
        for y in 0..emulator.get_display_height()
        {
            // Start building the line.
            let mut line = String::new();

            // Build each pixel.
            for x in 0..emulator.get_display_width()
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
            execute!(
                stdout(),
                Print(line),
                cursor::MoveToNextLine(1)
            ).unwrap();
        }

        execute!(
            stdout(),
            ResetColor
        ).unwrap();
    }

    #[inline]
    fn has_quit(&self) -> bool
    {
        return self.quit;
    }
}