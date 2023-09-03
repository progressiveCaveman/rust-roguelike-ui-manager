// NOTE: CachedSprite and SpriteRef use Rc, probably worth using so leaving this here for now
/*
pub type CachedSprite = (usize, usize, Rc<[u8]>);

/// SpriteRefs can be drawn and animated.
///
/// They reference their pixel data (instead of owning it).
#[derive(Debug)]
pub struct SpriteRef {
    width: usize,
    height: usize,
    pixels: Rc<[u8]>,
    ch: usize,
    duration: Duration,
    dt: Duration,
}

impl Drawable for SpriteRef {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}
*/

use crate::colors::{Color, self};

/// Sprites can be drawn and procedurally generated.
///
/// A `Sprite` owns its pixel data, and cannot be animated. Use a `SpriteRef` if you need
/// animations.
#[derive(Debug, Clone)]
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // this probably shouldn't be pub
}

impl Sprite {
    // assumes bg is fuscia and fg is greyscale
    pub fn with_color(&self, bg: Color, fg: Color) -> Sprite {
        let mut source = self.clone();
        let mut s = self.clone();
        for (i, pixel) in source.pixels.chunks_exact_mut(4).enumerate() {
            assert!(pixel.len() == 4);
            for i2 in 0..4 {
                let idx = i * 4 + i2;
                if pixel == colors::COLOR_FUCHSIA { // background
                    s.pixels[idx] = bg[i2];
                } else { // foreground
                    s.pixels[idx] = (s.pixels[idx] as f32 * fg[i2] as f32 / 255 as f32) as u8;
                }
            }
        }

        s
    }
}

/// Drawables can be blitted to the pixel buffer and animated.
pub trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

impl Drawable for Sprite {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}
