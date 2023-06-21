extern crate pretty_env_logger;
#[macro_use] extern crate log;

use emulator_chip8 as CHIP8;
use CHIP8::Frontend as Frontend;
use CHIP8::Frontends as Frontends;

mod delta_timer;
use delta_timer::DeltaTimer as DeltaTimer;

fn main()
{
    // This should be disabled in terminal display mode.
    pretty_env_logger::init();

    let mut config = CHIP8::Configs::EmulatorConfig::default();
    config.cpu_config.timer.rate = 10000.0;

    let mut emulator = CHIP8::Emulator::new(&config);
    emulator.load("6-keypad.ch8");

    info!("Emulator backend setup completed successfully.");

    let ui_config = Frontends::TerminalFrontendConfig::default();
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
