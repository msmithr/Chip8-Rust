mod cpu;
mod fontset;
mod operation_map;
mod operations;

#[cfg(test)]
mod operation_tests;

pub use self::cpu::Cpu;

pub const CHIP8_WIDTH: u32 = 64;
pub const CHIP8_HEIGHT: u32 = 32;
