use std::io::{stdout, Write};

use std::time::Duration;

use crate::Frontend as Frontend;

use crossterm::event::{KeyModifiers, PopKeyboardEnhancementFlags};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    cursor,
    event::{poll, read, Event, KeyCode, KeyEventKind, PushKeyboardEnhancementFlags, KeyboardEnhancementFlags}
};

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

        // TODO: maybe not like this lol.
        result.bindings.push((KeyCode::Char('x'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('1'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('2'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('3'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('q'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('w'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('e'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('a'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('s'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('d'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('z'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('c'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('4'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('r'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('f'), KeyModifiers::NONE));
        result.bindings.push((KeyCode::Char('v'), KeyModifiers::NONE));

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
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
        ).unwrap();

        enable_raw_mode().unwrap();

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

        execute!(
            stdout(),
            cursor::Show,
            PopKeyboardEnhancementFlags,
        ).unwrap();
    }
}

impl Frontend for TerminalFrontend
{
    fn update(&mut self, emulator:&mut crate::Emulator, _delta: f64)
    {
        if !emulator.is_running()
        {
            info!("Backend has stopped running, closing app...");
            self.exit();
            return;
        }

        if poll(Duration::from_millis(1)).unwrap()
        {
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
                    
                    for index in 0..0x10
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