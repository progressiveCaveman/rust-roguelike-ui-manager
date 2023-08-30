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
