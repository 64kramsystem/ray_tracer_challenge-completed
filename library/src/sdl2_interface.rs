use sdl2::{event::Event, pixels, rect::Point, render, video::Window, EventPump};

use crate::export_to_pixels::ExportToPixels;

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

    center_x: i16,
    center_y: i16,

    // The SDL2 pixel reading doesn't work as intended (see history), so we keep an internal buffer.
    // The upside is that this can be used, if desired, to trivially redraw on window resize.
    //
    pixels_buffer: Vec<crate::Color>,
}

impl Sdl2Interface {
    // Initializes the canvas, and maximizes the window.
    //
    // center: (x, y), from the bottom left.
    //
    pub fn init(window_title: &str, width: u16, height: u16, center: (i16, i16)) -> Self {
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
            center_x: center.0,
            center_y: center.1,
            pixels_buffer,
        }
    }

    // Writes a pixel at (x, y), where (0, 0) is the bottom left of the canvas.
    // Doesn't update the canvas; for that, must invoke update_canvas().
    // Pixels outside the canvas are ignored.
    //
    pub fn write_pixel(&mut self, mut x: i16, mut y: i16, color: crate::Color) {
        let (width, height) = self.canvas.logical_size();

        x += self.center_x;
        y += self.center_y;

        y = self.canvas_height() as i16 - y as i16 - 1;

        if x >= 0 && x < width as i16 && y >= 0 && y < height as i16 {
            self.pixels_buffer[y as usize * width as usize + x as usize] = color;

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
                Event::KeyDown { .. } => break,
                Event::KeyUp { .. } => break,
                Event::Quit { .. } => std::process::exit(0),
                _ => {}
            }
        }
    }

    // Convenience method.
    //
    pub fn canvas_height(&self) -> u16 {
        self.canvas.logical_size().1 as u16
    }
}

impl ExportToPixels for Sdl2Interface {
    fn to_pixels(&self) -> (&Vec<crate::Color>, u16) {
        let (width, _) = self.canvas.logical_size();

        (&self.pixels_buffer, width as u16)
    }
}
