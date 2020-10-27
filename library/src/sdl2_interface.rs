use sdl2::{event::Event, pixels, rect::Point, render, video::Window, EventPump};

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
}

impl Sdl2Interface {
    // Initializes the canvas, and maximizes the window.
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

        Self { event_pump, canvas }
    }

    // Doesn't update the canvas; for that, must invoke update_canvas().
    //
    pub fn write_pixel(&mut self, x: u16, y: u16, color: crate::Color) {
        let pixel = pixels::Color::RGB(
            (255.0 * color.r) as u8,
            (255.0 * color.g) as u8,
            (255.0 * color.b) as u8,
        );

        self.canvas.set_draw_color(pixel);

        self.canvas
            .draw_point(Point::new(x as i32, y as i32))
            .unwrap();
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
}
