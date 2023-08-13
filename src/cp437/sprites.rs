use core::time::Duration;
use std::rc::Rc;

pub type CachedSprite = (usize, usize, Rc<[u8]>);

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

/// Drawables can be blitted to the pixel buffer and animated.
pub trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pixels(&self) -> &[u8];
}

// pub(crate) trait Animation {
//     fn animate(&mut self, assets: &Assets);
// }

impl Sprite {
    // pub fn new(assets: &Assets, ch: usize) -> Sprite {
    //     let (width, height, pixels) = &assets.cp437[ch];//assets.sprites().get(&frame).unwrap();

    //     Sprite {
    //         width: *width,
    //         height: *height,
    //         pixels: pixels.to_vec(),
    //     }
    // }

    // pub fn whole_sheet(assets: &Assets) -> Sprite {
    //     let (width, height, pixels) = &assets.sheet;//assets.sprites().get(&frame).unwrap();

    //     Sprite {
    //         width: *width,
    //         height: *height,
    //         pixels: pixels.to_vec(),
    //     }
    // }
}

impl SpriteRef {
    // pub fn new(assets: &Assets, ch: usize, duration: Duration) -> SpriteRef {
    //     let (width, height, pixels) = &assets.cp437[ch];//assets.sprites().get(&frame).unwrap();

    //     SpriteRef {
    //         width: *width,
    //         height: *height,
    //         pixels: Rc::clone(&pixels),
    //         ch,
    //         duration,
    //         dt: Duration::default(),
    //     }
    // }

    // pub(crate) fn step_frame(&mut self, assets: &Assets) {
    //     use Frame::*;

    //     let assets = assets.sprites();
    //     let (pixels, frame) = match self.frame {
    //         Blipjoy1 => (Rc::clone(&assets.get(&Blipjoy2).unwrap().2), Blipjoy2),
    //         Blipjoy2 => (Rc::clone(&assets.get(&Blipjoy1).unwrap().2), Blipjoy1),

    //         Ferris1 => (Rc::clone(&assets.get(&Ferris2).unwrap().2), Ferris2),
    //         Ferris2 => (Rc::clone(&assets.get(&Ferris1).unwrap().2), Ferris1),

    //         Cthulhu1 => (Rc::clone(&assets.get(&Cthulhu2).unwrap().2), Cthulhu2),
    //         Cthulhu2 => (Rc::clone(&assets.get(&Cthulhu1).unwrap().2), Cthulhu1),

    //         Player1 => (Rc::clone(&assets.get(&Player2).unwrap().2), Player2),
    //         Player2 => (Rc::clone(&assets.get(&Player1).unwrap().2), Player1),

    //         Bullet1 => (Rc::clone(&assets.get(&Bullet2).unwrap().2), Bullet2),
    //         Bullet2 => (Rc::clone(&assets.get(&Bullet3).unwrap().2), Bullet3),
    //         Bullet3 => (Rc::clone(&assets.get(&Bullet4).unwrap().2), Bullet4),
    //         Bullet4 => (Rc::clone(&assets.get(&Bullet5).unwrap().2), Bullet5),
    //         Bullet5 => (Rc::clone(&assets.get(&Bullet1).unwrap().2), Bullet1),

    //         Laser1 => (Rc::clone(&assets.get(&Laser2).unwrap().2), Laser2),
    //         Laser2 => (Rc::clone(&assets.get(&Laser3).unwrap().2), Laser3),
    //         Laser3 => (Rc::clone(&assets.get(&Laser4).unwrap().2), Laser4),
    //         Laser4 => (Rc::clone(&assets.get(&Laser5).unwrap().2), Laser5),
    //         Laser5 => (Rc::clone(&assets.get(&Laser6).unwrap().2), Laser6),
    //         Laser6 => (Rc::clone(&assets.get(&Laser7).unwrap().2), Laser7),
    //         Laser7 => (Rc::clone(&assets.get(&Laser8).unwrap().2), Laser8),
    //         Laser8 => (Rc::clone(&assets.get(&Laser1).unwrap().2), Laser1),

    //         _ => unreachable!(),
    //     };

    //     self.pixels = pixels;
    //     self.frame = frame;
    // }
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

// impl Animation for SpriteRef {
//     fn animate(&mut self, assets: &Assets) {
//         if self.duration.subsec_nanos() == 0 {
//             self.step_frame(assets);
//         } else {
//             self.dt += TIME_STEP;

//             while self.dt >= self.duration {
//                 self.dt -= self.duration;
//                 self.step_frame(assets);
//             }
//         }
//     }
// }



// /// Draw a line to the pixel buffer using Bresenham's algorithm.
// pub(crate) fn line(screen: &mut [u8], p1: &Point, p2: &Point, color: [u8; 4]) {
//     let p1 = (p1.x as i64, p1.y as i64);
//     let p2 = (p2.x as i64, p2.y as i64);

//     for (x, y) in Bresenham::new(p1, p2) {
//         let x = min(x as usize, WIDTH - 1);
//         let y = min(y as usize, HEIGHT - 1);
//         let i = x * 4 + y * WIDTH * 4;

//         screen[i..i + 4].copy_from_slice(&color);
//     }
// }

// /// Draw a rectangle to the pixel buffer using two points in opposite corners.
// pub(crate) fn rect(screen: &mut [u8], p1: &Point, p2: &Point, color: [u8; 4]) {
//     let p2 = Point::new(p2.x - 1, p2.y - 1);
//     let p3 = Point::new(p1.x, p2.y);
//     let p4 = Point::new(p2.x, p1.y);

//     line(screen, p1, &p3, color);
//     line(screen, &p3, &p2, color);
//     line(screen, &p2, &p4, color);
//     line(screen, &p4, p1, color);
// }
