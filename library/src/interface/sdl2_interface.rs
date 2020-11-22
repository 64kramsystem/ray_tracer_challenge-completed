use sdl2::{
    event::Event, keyboard::Keycode, pixels, rect::Point, render, video::Window, EventPump,
};

use super::image::Image;
use crate::properties::{Color, COLOR_BLACK};

// Interface for drawing to a canvas, and waiting a keypress, intentionally designed to be as simple
// as  possible.
//
// Doesn't handle resize (it won't redraw). It's currently unclear how to do it; invoking canvas.present()
// on WindowResize doesn't have any effect.
//
// Previously, read methods were applied, but testing proved to be a complete trainwreck (see history).
//
pub struct Sdl2Interface {
    event_pump: EventPump,
    canvas: render::Canvas<Window>,

    width: u16,
    height: u16,

    // For simple applications. They'd be cool if set by method chaining, however, they're not needed
    // anymore with camera rendering.
    //
    pub invert_y: bool,
    pub origin: (i16, i16),

    // The SDL2 pixel reading doesn't work as intended (see history), so we keep an internal buffer.
    // The upside is that this can be used, if desired, to trivially redraw on window resize.
    //
    pixels_buffer: Vec<Color>,
}

impl Sdl2Interface {
    // Initializes the canvas, and maximizes the window.
    //
    // origin: (x, y), from the bottom left.
    //
    pub fn init(window_title: &str, width: u16, height: u16) -> Self {
        let sdl_context = sdl2::init().unwrap();

        // The resizing (due to `maximized()`) is handled below, by process_events().
        //
        let window = sdl_context
            .video()
            .unwrap()
            .window(window_title, width as u32, height as u32)
            .maximized()
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        canvas
            .set_logical_size(width as u32, height as u32)
            .unwrap();

        // This is necessary, although not 100% clear why; if not executed, the canvas is not centered.
        //
        event_pump.pump_events();

        let pixels_buffer = vec![COLOR_BLACK; width as usize * height as usize];

        Self {
            event_pump,
            canvas,
            width,
            height,
            invert_y: false,
            origin: (0, 0),
            pixels_buffer,
        }
    }

    // Wait for keypress; if a quit event is received (e.g. Alt+F4 or window close), the program will
    // exit.
    //
    pub fn wait_keypress(&mut self) {
        for event in self.event_pump.wait_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break,
                Event::KeyUp {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break,
                Event::Quit { .. } => std::process::exit(0),
                _ => {}
            }
        }
    }

    // Adjust in two ways:
    //
    // - recenter according to (origin_x, origin_y)
    // - turn the y coordinate upside down, if set (Sdl starts at top left; bottom left is more intuitive)
    //
    fn adjust_coordinates(&self, mut x: i16, mut y: i16) -> (i16, i16) {
        x += self.origin.0;
        y += self.origin.1;

        if self.invert_y {
            y = self.height() as i16 - y - 1
        };

        (x, y)
    }

    // Returns the index of the pixel in the buffer; if the pixel is outside the canvas, None is returned.
    //
    fn pixel_buffer_index(&self, x: i16, y: i16) -> Option<usize> {
        if x >= 0 && x < self.width as i16 && y >= 0 && y < self.height as i16 {
            Some(y as usize * self.width as usize + x as usize)
        } else {
            None
        }
    }
}

impl Image for Sdl2Interface {
    fn new(width: u16, height: u16) -> Self {
        Self::init("Sdl2Interface", width, height)
    }

    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    // Doesn't update the canvas; for that, must invoke update_canvas().
    // Pixels outside the canvas are ignored.
    //
    fn write_pixel(&mut self, x: i16, y: i16, color: Color) {
        let (x, y) = self.adjust_coordinates(x, y);

        if let Some(pixel_buffer_index) = self.pixel_buffer_index(x, y) {
            self.pixels_buffer[pixel_buffer_index] = color;

            let (r, g, b) = color.u8_components();
            let pixel = pixels::Color::RGB(r, g, b);

            self.canvas.set_draw_color(pixel);

            self.canvas
                .draw_point(Point::new(x as i32, y as i32))
                .unwrap();
        }
    }

    // Required after writing pixels.
    //
    fn update(&mut self) {
        self.canvas.present();
    }

    fn to_pixels(&self) -> Vec<&Color> {
        // Inverts the y axis, using rev().
        //
        self.pixels_buffer
            .chunks_exact(self.width as usize)
            .rev()
            .flatten()
            .collect::<Vec<_>>()
    }

    fn pixel_at(&self, x: i16, y: i16) -> Option<&Color> {
        let pixel_buffer_index = self.pixel_buffer_index(x, y);

        if let Some(pixel_index) = pixel_buffer_index {
            Some(&self.pixels_buffer[pixel_index])
        } else {
            None
        }
    }
}
