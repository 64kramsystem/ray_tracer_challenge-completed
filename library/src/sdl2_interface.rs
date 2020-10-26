use crate::interface::Interface;

use sdl2::{
    event::{Event, WindowEvent},
    pixels,
    rect::Point,
    render,
    video::Window,
    EventPump,
};

// Logical width (resolution as seen by the program).
// SDL2 uses different data types, but u16 is the one that makes sense.
//
pub const CANVAS_WIDTH: u16 = 4; // 1024;
pub const CANVAS_HEIGHT: u16 = 4; // 768;

const TOP_BORDER_START_SIZE: u16 = 0;
const LEFT_BORDER_START_SIZE: u16 = 0;

pub struct Sdl2Interface {
    event_pump: EventPump,
    canvas: render::Canvas<Window>,

    // Compensate for proportions not equal to the screen.
    //
    top_border_size: i32,
    left_border_size: i32,
}

impl Interface for Sdl2Interface {
    fn init(window_title: &str) -> Self {
        let sdl_context = sdl2::init().unwrap();

        // The resizing (due to `maximized()`) is handled below, by process_events().
        //
        let window = sdl_context
            .video()
            .unwrap()
            .window(window_title, CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32)
            .maximized()
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let canvas = window.into_canvas().present_vsync().build().unwrap();

        let mut sdl2_canvas = Sdl2Interface {
            event_pump,
            canvas,
            top_border_size: TOP_BORDER_START_SIZE as i32,
            left_border_size: LEFT_BORDER_START_SIZE as i32,
        };

        sdl2_canvas.process_events(false);

        sdl2_canvas
    }

    fn write_pixel(&mut self, x: u16, y: u16, color: crate::Color) {
        if x < CANVAS_WIDTH && y < CANVAS_HEIGHT {
            let pixel = pixels::Color::RGB(
                (255.0 * color.r) as u8,
                (255.0 * color.g) as u8,
                (255.0 * color.b) as u8,
            );

            self.canvas.set_draw_color(pixel);

            self.canvas
                .draw_point(Point::new(
                    self.left_border_size + x as i32,
                    self.top_border_size + y as i32,
                ))
                .unwrap();
        }
    }

    fn update_canvas(&mut self) {
        self.canvas.present();
    }

    fn wait_keypress(&mut self) {
        self.process_events(true);
    }
}

impl Sdl2Interface {
    fn process_events(&mut self, blocking: bool) {
        loop {
            let event = if blocking {
                Some(self.event_pump.wait_event())
            } else {
                self.event_pump.poll_event()
            };

            match event {
                Some(Event::KeyDown { .. }) => break,
                Some(Event::KeyUp { .. }) => break,
                Some(Event::Window {
                    win_event: WindowEvent::SizeChanged(new_width, new_height),
                    ..
                }) => {
                    self.update_window_dimensions(new_width, new_height);
                }
                Some(Event::Quit { .. }) => std::process::exit(0),
                // This happens only for non-blocking events.
                //
                None => break,
                // Ignore all the other events.
                //
                _ => {}
            }
        }
    }

    // Reacts to window resizing events; takes care of clearing, centering, and updating the scale.
    //
    fn update_window_dimensions(&mut self, window_width: i32, window_height: i32) {
        let min_scale = f32::min(
            (window_width as f32) / (CANVAS_WIDTH as f32).floor(),
            (window_height as f32) / (CANVAS_HEIGHT as f32).floor(),
        );

        self.canvas.set_scale(min_scale, min_scale).unwrap();

        // The FP accuracy is not worth considering.
        //
        self.top_border_size =
            ((window_height as f32 / min_scale) as i32 - CANVAS_HEIGHT as i32) / 2;

        self.left_border_size =
            ((window_width as f32 / min_scale) as i32 - CANVAS_WIDTH as i32) / 2;

        // If we don't clear, if a part of the canvas is not covered due to mismatch between the
        // screen and the window, will have undefined content.
        //
        self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        self.canvas.clear();
    }
}
