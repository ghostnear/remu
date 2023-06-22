use std::io::{stdout, Write};

use std::time::Duration;

use crate::Frontend as Frontend;

use crossterm::event::KeyModifiers;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    cursor,
    event::{poll, read, Event, KeyCode}
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
            cursor::Hide,
        ).unwrap();

        enable_raw_mode().unwrap();

        Self {
            foreground: config.foreground,
            background: config.background,
            quit: false
        }
    }

    fn exit(&mut self)
    {
        self.quit = true;

        disable_raw_mode().unwrap();

        execute!(
            stdout(),
            cursor::Show
        ).unwrap();
    }
}

impl Frontend for TerminalFrontend
{
    fn update(&mut self, emulator:&crate::Emulator, _delta: f64)
    {
        if !emulator.is_running()
        {
            info!("Backend has stopped running, closing app...");
            self.exit();
            return;
        }

        if poll(Duration::from_millis(1)).unwrap() {

            // Read the input.
            match read().unwrap()
            {
                Event::Key(event) =>
                {
                    // CTRL + C / CTRL + Z / Escape to close the app.
                    if (event.modifiers == KeyModifiers::CONTROL && (event.code == KeyCode::Char('c') || event.code == KeyCode::Char('z'))) || event.code == KeyCode::Esc
                    {
                        info!("Closing on user request...");
                        self.exit();
                        return;
                    }

                    trace!("{:?}", event);
                }
                _ => {}
            }
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

        stdout().flush().unwrap();
    }

    #[inline]
    fn has_quit(&self) -> bool
    {
        return self.quit;
    }
}