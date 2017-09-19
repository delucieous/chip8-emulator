#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate fern;

use clap::App;

use std::process;

mod memory;
mod cpu;
mod emustate;
mod emulator;
mod decoder;

fn main() {
    let yaml = load_yaml!("chip8.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = emulator::Config {
        filename: matches.value_of("ROM").unwrap().to_string(),
        verbose: matches.is_present("verbose")
    };

    if let Err(e) = setup_logging(&config.verbose) {
        error!("Error setting up logging: {}", e);
        process::exit(1);
    };

    if let Err(e) = emulator::run(config) {
        error!("Application error: {}", e);
        process::exit(1);
    }
}

pub fn setup_logging(verbose: &bool) -> Result<(), fern::InitError> {
    
    let mut level = log::LogLevelFilter::Info;
    if *verbose {
        level = log::LogLevelFilter::Debug;
    }
    
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
            "{}[{}] {}",
            record.target(),
            record.level(),
            message
        ))
        })
        .level(level)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}