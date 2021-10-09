use super::fontset::FONTSET;
use super::operations;
use super::CHIP8_HEIGHT;
use super::CHIP8_WIDTH;
use std::error;

pub struct Cpu {
    pub memory: [u8; 4096],
    pub register: [u8; 16],
    pub register_i: u16, // memory location register
    pub program_counter: usize,
    pub display: [u8; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize], // display, 64*32 pixels, 0=black, 1=white
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16],
    pub stack_pointer: usize,
    pub keypad: [bool; 16],
    pub redraw_flag: bool,
    pub keypad_waiting: bool,
    pub keypad_waiting_register: u8,
}

pub struct CycleOutput {
    pub redraw_flag: bool,
    pub sound_flag: bool,
}

impl Cpu {
    pub fn new(program: &Vec<u8>) -> Result<Self, Box<dyn error::Error>> {
        let mut cpu = Self {
            memory: [0; 4096],
            register: [0; 16],
            register_i: 0,
            program_counter: 0x200,
            display: [0; (CHIP8_WIDTH * CHIP8_HEIGHT) as usize],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            stack_pointer: 0,
            keypad: [false; 16],
            redraw_flag: false,
            keypad_waiting: false,
            keypad_waiting_register: 0,
        };

        // load fontset into memory
        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }

        // load program into memory
        for i in 0..program.len() {
            cpu.memory[i + 0x200] = program[i];
        }

        Ok(cpu)
    }

    pub fn cycle(&mut self, key_state: &[bool; 16]) -> Result<CycleOutput, Box<dyn error::Error>> {
        for i in 0..16 {
            self.keypad[i] = key_state[i];
        }

        let mut redraw_flag = false;
        let mut sound_flag = false;

        if self.keypad_waiting {
            for i in 0..self.keypad.len() {
                if self.keypad[i] {
                    self.keypad_waiting = false;
                    self.register[self.keypad_waiting_register as usize] = i as u8;
                    break;
                }
            }
        } else {
            operations::execute_instruction(self);

            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
                sound_flag = true;
            }

            redraw_flag = self.redraw_flag;
            self.redraw_flag = false;
            self.program_counter += 2;
        }

        Ok(CycleOutput {
            redraw_flag: redraw_flag,
            sound_flag: sound_flag,
        })
    }
}
