extern crate pretty_env_logger;
#[macro_use] extern crate log;

use emulator_chip8 as CHIP8;
use CHIP8::Frontend as Frontend;
use CHIP8::Frontends as Frontends;

mod delta_timer;
use delta_timer::DeltaTimer as DeltaTimer;

use crossterm::style::Color as Color;

fn main()
{
    // Recommended to disable in terminal mode.
    pretty_env_logger::init();

    let mut config = CHIP8::Configs::EmulatorConfig::default();
    config.cpu_config.timer.rate = 10000.0;

    let mut emulator = CHIP8::Emulator::new(&config);
    emulator.load("2-ibm-logo.ch8");

    info!("Emulator backend setup completed successfully.");

    let mut ui_config = Frontends::TerminalFrontendConfig::default();
    ui_config.foreground = Color::Rgb { r: (0xBB), g: (0xBB), b: (0xBB) };
    ui_config.background = Color::Rgb { r: (0x11), g: (0x11), b: (0x11) };
    let mut user_interface = Frontends::TerminalFrontend::new(&ui_config);

    let mut delta_timer = DeltaTimer::new();

    while !user_interface.has_quit()
    {
        delta_timer.update();

        emulator.update(delta_timer.get());
        user_interface.update(&emulator, delta_timer.get());

        user_interface.draw(&mut emulator);
    }
}
