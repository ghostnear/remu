extern crate env_logger;
#[macro_use] extern crate log;
use env_logger::{Builder, Env, Target};
use std::fs::OpenOptions;

use emulator_chip8 as CHIP8;
use CHIP8::Frontend as Frontend;
use CHIP8::Frontends as Frontends;

mod delta_timer;
use delta_timer::DeltaTimer as DeltaTimer;

use crossterm::style::Color as Color;

fn setup_logging()
{
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    let file = OpenOptions::new().create(true).write(true).truncate(true).open("last.log").unwrap();
    Builder::from_env(env).target(Target::Pipe(Box::new(file))).init();
}

fn main()
{
    setup_logging();

    // Setup emulator.
    let mut config = CHIP8::Configs::EmulatorConfig::default();
    config.cpu_config.timer.rate = 1000.0;
    let mut emulator = CHIP8::Emulator::new(&config);
    emulator.load("roms/6-keypad.ch8");

    info!("Emulator backend setup completed successfully.");

    // UI setup.
    let mut ui_config = Frontends::TerminalFrontendConfig::default();
    ui_config.foreground = Color::Rgb { r: (0xBB), g: (0xBB), b: (0xBB) };
    ui_config.background = Color::Rgb { r: (0x11), g: (0x11), b: (0x11) };
    let mut user_interface = Frontends::TerminalFrontend::new(&ui_config);

    // Delta timing.
    let mut delta_timer = DeltaTimer::new();

    // Main loop of the app.
    while !user_interface.has_quit()
    {
        delta_timer.update();

        user_interface.update(&mut emulator, delta_timer.get());
        emulator.update(delta_timer.get());
        
        user_interface.draw(&mut emulator);
    }
}
