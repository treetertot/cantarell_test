use std::{io::Read, format};

use ab_glyph::{Font, point};
use fontconfig::Fontconfig;
use image::{GrayImage, Luma};

static CHARS: &str = "ABCDEFGHIJKLMNOPabcdefghijklmnop";

fn main() {
    let fc = Fontconfig::new().unwrap();
    let cantarell_path = fc.find("cantarell", None).expect("Failed to find font 'cantarell'").path;
    let mut cantarell_file = std::fs::File::open(&cantarell_path).expect("cannot open font 'cantarell'");
    let cantarell = {
        let mut buf = Vec::new();
        cantarell_file.read_to_end(&mut buf).expect("unable to read font file 'cantarell'");
        ab_glyph::FontVec::try_from_vec(buf).expect("could not read font index 0 in collection cantarell")
    };

    for c in CHARS.chars() {
        let cap_h = cantarell.glyph_id(c).with_scale_and_position(1000.0, point(0., 0.));
        let width = cap_h.scale.x as u32;
        let height = cap_h.scale.y as u32;
        let mut image = GrayImage::new(width, height);

        if let Some(h) = cantarell.outline_glyph(cap_h) {
            h.draw(|x, y, c| {
                let byte = (c * 255.) as u8;
                image.put_pixel(x, y, Luma([byte]));
            });
        }
        let name = format!("{}.png", c);
        image.save(&name).unwrap();
    }
}
