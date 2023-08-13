use image::{self};
use image::GenericImageView;
use rltk::Point;

use crate::{WIDTH, HEIGHT};

use self::sprites::{Sprite, Drawable}; // to allow calling .pixels()

mod converter;
mod sprites;

const GLYPH_SIZE: usize = 8;
const GLYPHS_PER_ROW: usize = 16;

pub struct Assets {
    pub cp437: Vec<Sprite>, 
    pub sheet: Sprite
}

impl Assets{
    pub fn new() -> Assets {
        let img = image::open("res/terminal8x8.jpg").expect("File not found!");

        let empty_glyph = Sprite {
            width: GLYPH_SIZE,
            height: GLYPH_SIZE,
            pixels: vec![0; GLYPH_SIZE * GLYPH_SIZE * 4],
        };

        let mut cp: Vec<Sprite> = vec![empty_glyph; 256];
        let mut sheet = Sprite {
            width: 128,
            height: 128,
            pixels: vec![0; 65536],
        };

        let mut pcount = 0;
        for pixel in img.pixels() {
            pcount += 1;
            dbg!(pixel);

            let x: usize = pixel.0 as usize;
            let y: usize = pixel.1 as usize;
            let idx = (x + y * 128) * 4;
            
            let glyph_num = x / GLYPH_SIZE + (GLYPHS_PER_ROW * (y / GLYPH_SIZE));
            let xlocal = x % GLYPH_SIZE;
            let ylocal = y % GLYPH_SIZE;
            let idxlocal = (xlocal + ylocal * GLYPH_SIZE) * 4;

            dbg!(idx);
            dbg!(glyph_num);
            dbg!(xlocal);
            dbg!(ylocal);
            dbg!(idxlocal);
    
            for i in 0..4 {
                sheet.pixels[idx+i] = pixel.2[i];
                cp[glyph_num].pixels[idxlocal + i] = pixel.2[i];
            }
        }

        dbg!(pcount);

        Assets {
            cp437: cp,
            sheet,
        }
    }
}

/// Blit a drawable to the pixel buffer.
pub fn blit<S>(screen: &mut [u8], dest: &Point, sprite: &S)
where
    S: Drawable,
{
    assert!(dest.x + sprite.width() as i32 <= WIDTH);
    assert!(dest.y + sprite.height() as i32 <= HEIGHT);

    let pixels = sprite.pixels();
    let width = sprite.width() * 4;

    let mut s = 0;
    for y in 0..sprite.height() {
        let i = dest.x * 4 + dest.y * WIDTH * 4 + y as i32 * WIDTH * 4;

        // Merge pixels from sprite into screen
        let zipped = screen[i as usize..i as usize + width].iter_mut().zip(&pixels[s..s + width]);
        for (left, &right) in zipped {
            if right > 0 {
                *left = right;
            }
            // *left = right;
        }

        s += width;
    }
}