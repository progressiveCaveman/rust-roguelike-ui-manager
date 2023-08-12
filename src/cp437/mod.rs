use image::{self};
use image::GenericImageView;
use rltk::Point;

use crate::{WIDTH, HEIGHT};

use self::sprites::{Sprite, Drawable}; // to allow calling .pixels()

mod converter;
mod sprites;

const glyph_size: usize = 8;
const glyphlen: usize = glyph_size * glyph_size * 4;
const glyphs_per_row: usize = 16;

pub struct Assets {
    pub cp437: Vec<Sprite>, 
    pub sheet: Sprite
}

impl Assets{
    pub fn new() -> Assets {
        let img = image::open("res/terminal8x8.jpg").expect("File not found!");

        let empty_glyph = Sprite {
            width: glyph_size,
            height: glyph_size,
            pixels: vec![0; glyph_size * glyph_size * 4],
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
            
            let glyph_num = x / glyph_size + (glyphs_per_row * (y / glyph_size));
            let xlocal = x % glyph_size;
            let ylocal = y % glyph_size;
            let idxlocal = (xlocal + ylocal * glyph_size) * 4;

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




        // // type Sprite = (usize, usize, [u8; glyphlen]); // TODO use Rc<Rgba<u8>> instead

        // let arr: [u8; glyphlen] = [0; glyphlen];
        // let empty_glyph: Sprite = (glyph_size, glyph_size, arr);
        // // let mut sheet: (usize, usize, [u8; 65536]) = (256,256,[0;65536]);
        
        // let mut cp: Vec<Sprite> = vec![empty_glyph; 256];



        // let mut result = Vec::new();
        
        // for pixel in img.pixels() {

        //     // Read the raw pixel data
        //     // let mut buffer = Vec::new();
        //     // buffer.resize_with(width * 3, Default::default);
        //     // reader.next_row_rgb(&mut buffer[..]).unwrap();

        //     // Copy to result with an alpha component
        //     let mut buffer = vec![pixel.2[0], pixel.2[2], pixel.2[3], pixel.2[4]];
        //     let pixels = buffer
        //         .chunks(3)
        //         .flat_map(|rgb| {
        //             let mut rgb = rgb.to_vec();
        //             rgb.push(255);
        //             rgb
        //         })
        //         .collect::<Vec<u8>>();
        //     result.extend_from_slice(&pixels);







        //     // modify RGBA pixel
        //     let x: usize = pixel.0 as usize;
        //     let y: usize = pixel.0 as usize;
        //     let idx = x + y * 256;
            
        //     let glyph_num = x / glyph_size + (glyphs_per_row * (y / glyph_size));
        //     let xlocal = x % glyph_size;
        //     let ylocal = y % glyph_size;
        //     let idxlocal = xlocal + ylocal * glyphs_per_row;
    
        //     for i in 0..4 {
        //         // sheet.2[idx+i] = pixel.2[i];
        //         cp[glyph_num].2[idxlocal + i] = pixel.2[i];
        //     }
        // }

        // let mut cached: Vec<CachedSprite> = vec![];
        // for s in cp.iter() {
        //     cached.push((s.0, s.1, Rc::new(s.2)));
        // }

        // Assets { 
        //     cp437: cached,
        //     // sheet: (sheet.0, sheet.1, Rc::new(sheet.2))

        // }
    }

    // pub fn get_cp437(&self) -> Sprite {
    //     Sprite::new(self, 12)
    // }

    // pub fn get_sheet(&self) -> Sprite {
    //     Sprite::whole_sheet(self)
    // }
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