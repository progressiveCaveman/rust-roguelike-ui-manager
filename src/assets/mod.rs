use image::GenericImageView;
use image::{self};

use self::sprites::Sprite;

pub mod cp437_converter;
pub mod sprites;

const GLYPH_SIZE: usize = 8;
const GLYPHS_PER_ROW: usize = 16;

pub struct Assets {
    pub cp437: Vec<Sprite>,
}

impl Assets {
    pub fn new() -> Assets {
        let img = image::open("res/terminal8x8.jpg").expect("File not found!");

        let empty_glyph = Sprite {
            width: GLYPH_SIZE,
            height: GLYPH_SIZE,
            pixels: vec![0; GLYPH_SIZE * GLYPH_SIZE * 4],
        };

        let mut cp: Vec<Sprite> = vec![empty_glyph; 256];

        for pixel in img.pixels() {
            let x: usize = pixel.0 as usize;
            let y: usize = pixel.1 as usize;

            let glyph_num = x / GLYPH_SIZE + (GLYPHS_PER_ROW * (y / GLYPH_SIZE));
            let xlocal = x % GLYPH_SIZE;
            let ylocal = y % GLYPH_SIZE;
            let idxlocal = (xlocal + ylocal * GLYPH_SIZE) * 4;

            for i in 0..4 {
                cp[glyph_num].pixels[idxlocal + i] = pixel.2[i];
            }
        }

        Assets { cp437: cp }
    }
}
