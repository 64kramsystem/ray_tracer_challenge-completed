use sdl2::{
    event::Event, keyboard::Keycode, pixels, rect::Point, render, video::Window, EventPump,
};

use crate::{image::Image, Color};

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

    origin_x: i16,
    origin_y: i16,

    // The SDL2 pixel reading doesn't work as intended (see history), so we keep an internal buffer.
    // The upside is that this can be used, if desired, to trivially redraw on window resize.
    //
    pixels_buffer: Vec<crate::Color>,
}

impl Sdl2Interface {
    // Initializes the canvas, and maximizes the window.
    //
    // origin: (x, y), from the bottom left.
    //
    pub fn init(window_title: &str, width: u16, height: u16, origin: (i16, i16)) -> Self {
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

        let pixels_buffer = vec![
            crate::Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
            width as usize * height as usize
        ];

        Self {
            event_pump,
            canvas,
            origin_x: origin.0,
            origin_y: origin.1,
            pixels_buffer,
        }
    }

    pub fn set_origin(&mut self, x: i16, y: i16) {
        self.origin_x = x;
        self.origin_y = y;
    }

    // Writes a pixel at (x, y), where (0, 0) is the bottom left of the canvas.
    // Doesn't update the canvas; for that, must invoke update_canvas().
    // Pixels outside the canvas are ignored.
    //
    pub fn write_pixel(&mut self, x: i16, y: i16, color: crate::Color) {
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

    pub fn update_canvas(&mut self) {
        self.canvas.present();
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

    // Convenience method.
    //
    fn canvas_height(&self) -> i16 {
        self.canvas.logical_size().1 as i16
    }

    // Adjust in two ways:
    //
    // - recenter according to (origin_x, origin_y)
    // - turn the y coordinate upside down (Sdl starts at top left; bottom left is more intuitive)
    //
    fn adjust_coordinates(&self, mut x: i16, mut y: i16) -> (i16, i16) {
        x += self.origin_x;
        y += self.origin_y;

        y = self.canvas_height() - y - 1;

        (x, y)
    }

    // Returns the index of the pixel in the buffer; if the pixel is outside the canvas, None is returned.
    //
    fn pixel_buffer_index(&self, x: i16, y: i16) -> Option<usize> {
        let (width, height) = self.canvas.logical_size();

        if x >= 0 && x < width as i16 && y >= 0 && y < height as i16 {
            Some(y as usize * width as usize + x as usize)
        } else {
            None
        }
    }
}

impl Image for Sdl2Interface {
    fn to_pixels(&self) -> (&Vec<crate::Color>, u16) {
        let (width, _) = self.canvas.logical_size();

        (&self.pixels_buffer, width as u16)
    }

    fn pixel_at(&self, x: i16, y: i16) -> Option<Color> {
        let pixel_buffer_index = self.pixel_buffer_index(x, y);

        if let Some(pixel_index) = pixel_buffer_index {
            Some(self.pixels_buffer[pixel_index])
        } else {
            None
        }
    }
}
