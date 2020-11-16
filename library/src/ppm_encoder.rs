use std::io::Write;

use crate::export_to_pixels::ExportToPixels;

pub struct PpmEncoder {}

const MAX_LINE_LENGHT: usize = 70;
const MAX_COLOR_VALUE: u8 = 255;

impl PpmEncoder {
    pub fn export_image<T: ExportToPixels, U: Write>(image: &T, out: &mut U) {
        let (pixels, width) = image.to_pixels();
        let height = pixels.len() as u16 / width;

        let mut buffer = Self::build_header(width, height);

        let mut current_line = String::new();

        for color in pixels {
            let (r, g, b) = color.u8_components();

            for color_component in &[r, g, b] {
                // Avoid conditionals by always having the prefixing space, but must account that when
                // counting the max length, and then trim.
                //
                let r_str = format!(" {}", color_component);

                if current_line.len() + r_str.len() > MAX_LINE_LENGHT + 1 {
                    buffer += &current_line.trim_start();
                    buffer += "\n";
                    current_line.clear();
                }

                current_line += &r_str;
            }
        }

        if !current_line.is_empty() {
            buffer += &current_line.trim();
            buffer += "\n"
        }

        out.write_all(buffer.as_bytes()).unwrap();
    }

    fn build_header(width: u16, height: u16) -> String {
        let mut buffer = String::from("P3\n");

        buffer += &format!("{} {}\n", width, height);

        buffer += &format!("{}\n", MAX_COLOR_VALUE);

        buffer
    }
}
