use super::BACKGROUND_COLOR;
use super::CHIP8_HEIGHT;
use super::CHIP8_WIDTH;
use super::PIXEL_COLOR;
use sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::error;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<DisplayDriver, Box<dyn error::Error>> {
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("chip8", CHIP8_WIDTH * 20, CHIP8_HEIGHT * 20)
            .position_centered()
            .build()?;
        let mut canvas = window.into_canvas().present_vsync().build()?;

        canvas.set_scale(20.0, 20.0)?;

        canvas.set_draw_color(Color::RGB(
            BACKGROUND_COLOR.0,
            BACKGROUND_COLOR.1,
            BACKGROUND_COLOR.2,
        ));

        canvas.clear();
        canvas.present();

        Ok(DisplayDriver { canvas: canvas })
    }

    pub fn render(
        &mut self,
        display: &[u8; (CHIP8_HEIGHT * CHIP8_WIDTH) as usize],
    ) -> Result<(), Box<dyn error::Error>> {
        let mut rects = Vec::<Rect>::new();
        self.canvas.clear();

        for x in 0..CHIP8_WIDTH {
            for y in 0..CHIP8_HEIGHT {
                if display[(y * CHIP8_WIDTH + x) as usize] == 1 {
                    rects.push(Rect::new(x as i32, y as i32, 1, 1));
                }
            }
        }

        self.canvas
            .set_draw_color(Color::RGB(PIXEL_COLOR.0, PIXEL_COLOR.1, PIXEL_COLOR.2));

        self.canvas.fill_rects(&rects)?;

        self.canvas.set_draw_color(Color::RGB(
            BACKGROUND_COLOR.0,
            BACKGROUND_COLOR.1,
            BACKGROUND_COLOR.2,
        ));

        self.canvas.present();

        Ok(())
    }
}
