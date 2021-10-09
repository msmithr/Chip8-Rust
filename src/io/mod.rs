mod audio_driver;
mod display_driver;
mod input_driver;

pub use self::audio_driver::AudioDriver;
pub use self::display_driver::DisplayDriver;
pub use self::input_driver::InputDriver;

pub const BACKGROUND_COLOR: (u8, u8, u8) = (0, 0, 0);
pub const PIXEL_COLOR: (u8, u8, u8) = (173, 140, 255);
pub const CHIP8_WIDTH: u32 = 64;
pub const CHIP8_HEIGHT: u32 = 32;
