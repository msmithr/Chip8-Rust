mod io;
mod processor;

use crate::io::AudioDriver;
use crate::io::DisplayDriver;
use crate::io::InputDriver;
use crate::processor::Cpu;
use spin_sleep::LoopHelper;
use std::env;
use std::error;
use std::fs;
use std::path;

const TARGET_FREQUENCY: u32 = 600;

fn main() {
    let args: Vec<String> = env::args().collect();
    let called_program = &args[0];

    if args.len() != 2 {
        print_usage_string(called_program);
        return;
    }

    let config = match Config::new(&args) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let program = match read_program(&config.filename) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let sdl_context = sdl2::init().unwrap();
    let mut display_driver = DisplayDriver::new(&sdl_context).unwrap();
    let audio_driver = AudioDriver::new(&sdl_context).unwrap();
    let mut input_driver = InputDriver::new(&sdl_context).unwrap();
    let mut cpu = Cpu::new(&program).unwrap();

    let mut loop_helper = LoopHelper::builder().build_with_target_rate(TARGET_FREQUENCY);

    loop {
        loop_helper.loop_start();

        let key_state = input_driver.poll_events();

        let cycle_output = match cpu.cycle(&key_state) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        if cycle_output.redraw_flag {
            display_driver.render(&cpu.display).unwrap();
        }

        if cycle_output.sound_flag {
            audio_driver.start_beep();
        } else {
            audio_driver.stop_beep();
        }

        loop_helper.loop_sleep();
    }
}

struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn error::Error>> {
        let filename = &args[1];

        if !path::Path::new(filename).exists() {
            return Err(format!("file '{}' does not exist", filename).into());
        }

        Ok(Config {
            filename: filename.clone(),
        })
    }
}

fn print_usage_string(called_program: &str) {
    eprintln!("Usage: {} <program>", called_program);
}

fn read_program(filename: &str) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let program = fs::read(filename)?;
    Ok(program)
}
