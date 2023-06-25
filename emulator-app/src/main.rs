extern crate env_logger;
#[macro_use] extern crate log;
use env_logger::{Builder, Env, Target};
use std::fs::OpenOptions;

use emulator_chip8 as CHIP8;
use emulator_bytepusher as BYTEPUSHER;

mod delta_timer;
use delta_timer::DeltaTimer;

extern crate serde_json;
use serde_json::Value;

fn setup_logging(extensive_logging: bool)
{
    let mut log_level = "trace";
    if !extensive_logging
    {
        log_level = "warn";
    }

    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", log_level)
        .write_style_or("MY_LOG_STYLE", "always");

    let file = OpenOptions::new().create(true).write(true).truncate(true).open("last.log").unwrap();
    Builder::from_env(env).target(Target::Pipe(Box::new(file))).init();
}

fn setup_bytepusher(platform: &Value)
{
    // Setup emulator.
    let config = BYTEPUSHER::Configs::EmulatorConfig::from_json(&platform["backend_config"]);
    let mut emulator = BYTEPUSHER::Emulator::new(&config);
    emulator.load(platform["rom"].as_str().unwrap_or("none"));

    info!("Emulator backend setup completed successfully.");

    // UI setup.
    let mut delta_timer = DeltaTimer::new();
    let ui_config = BYTEPUSHER::RaylibFrontendConfig::from_json(&platform["frontend_config"]);
    let mut user_interface = BYTEPUSHER::RaylibFrontend::new(&ui_config);

    while !user_interface.has_quit()
    {
        delta_timer.update();

        user_interface.update(&mut emulator, delta_timer.get());
        emulator.update(delta_timer.get());
        
        user_interface.draw(&mut emulator);
    }
}

fn setup_chip8(platform: &Value)
{
    // Setup emulator.
    let config = CHIP8::Configs::EmulatorConfig::from_json(&platform["backend_config"]);
    let mut emulator = CHIP8::Emulator::new(&config);
    emulator.load(platform["rom"].as_str().unwrap_or("none"));

    info!("Emulator backend setup completed successfully.");

    // UI setup.
    let mut delta_timer = DeltaTimer::new();
    match platform["frontend"].as_str().unwrap_or("none")
    {
        "terminal" => {
            
            let ui_config = CHIP8::TerminalFrontendConfig::from_json(&platform["frontend_config"]);
            let mut user_interface = CHIP8::TerminalFrontend::new(&ui_config);

            while !user_interface.has_quit()
            {
                delta_timer.update();

                user_interface.update(&mut emulator, delta_timer.get());
                emulator.update(delta_timer.get());
                
                user_interface.draw(&mut emulator);
            }
        },

        "raylib" => {

            let ui_config = CHIP8::RaylibFrontendConfig::from_json(&platform["frontend_config"]);
            let mut user_interface = CHIP8::RaylibFrontend::new(&ui_config);

            while !user_interface.has_quit()
            {
                delta_timer.update();

                user_interface.update(&mut emulator, delta_timer.get());
                emulator.update(delta_timer.get());
                
                user_interface.draw(&mut emulator);
            }

        },
        
        _ => {
            error!("Invalid CHIP8 frontend specified!");
            panic!("Invalid CHIP8 frontend specified!");
        }
    }
}

fn setup_emulator(platform: &Value)
{
    let name = platform["name"].as_str().unwrap_or("none").to_uppercase();

    match name.as_str()
    {
        "BYTEPUSHER" => {
            setup_bytepusher(platform);
        },

        "CHIP8" => {
            setup_chip8(platform);
        },

        _ => {
            error!("Invalid platform specified: {}!", name);
            panic!("Invalid platform specified: {}!", name);
        }
    }
}

fn main()
{
    let arguments = std::env::args().nth(1);
    if arguments.is_none()
    {
        println!("No config file specified!");
        return;
    }

    let config_path = arguments.unwrap();
    let argument_data = std::fs::read_to_string(config_path.clone()).expect("Could not read config file!");
    let json_data: Value = serde_json::from_str(&argument_data).unwrap();

    setup_logging(json_data["extensive_logging"].as_bool().unwrap_or(false));

    info!("Used config from path {}.", config_path.clone());

    setup_emulator(&json_data["platform"]);
}
