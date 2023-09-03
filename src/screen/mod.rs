use crate::{
    assets::{
        cp437_converter::{string_to_cp437, to_cp437, FontCharType},
        sprites::Drawable,
        Assets,
    },
    Image, World, HEIGHT, WIDTH, colors::Color,
};
use rltk::Point;

use self::console::{Console, ConsoleMode};

pub mod console;

const MAX_ZOOM: usize = 8;

pub struct Screen {
    pub size: (i32, i32),
    pub pos: (i32, i32),
    pub input_blocking: bool,
    consoles: Vec<Console>,
}

pub struct Glyph {
    pub pos: Point,
    pub fg: Color,
    pub bg: Color,
}

impl Screen {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(size: (i32, i32), pos: (i32, i32)) -> Self {
        Self {
            size,
            pos,
            input_blocking: false,
            consoles: Vec::new(),
        }
    }

    pub fn setup_consoles(&mut self) {
        let gsize = 8; // todo make this not magical

        // log
        let x = 0;
        let y = 0;
        let w = self.size.0;
        let h = 10 * gsize;
        self.consoles.push(Console::new((w, h), (x, y), ConsoleMode::Log));
        // let gsize = 8; // todo make this not magical

        // main window
        let x = 0;
        let y = h;
        let w = w;
        let h = self.size.1 - h;
        self.consoles.push(Console::new((w, h), (x, y), ConsoleMode::WorldMap));
    }

    pub fn set_main_console_mode(&mut self, mode: ConsoleMode) {
        self.consoles[1].mode = mode;
    }

    pub fn increment_zoom(&mut self) {
        if self.consoles[1].zoom < MAX_ZOOM {
            self.consoles[1].zoom += 1;
        }
    }

    pub fn decrement_zoom(&mut self) {
        if self.consoles[1].zoom > 1 {
            self.consoles[1].zoom -= 1;
        }
    }

    pub fn draw(&self, frame: &mut [u8], world: &World) {
        for c in self.consoles.iter() {
            c.render(frame, world);
        }
    }

    pub fn print_char(&self, assets: &Assets, frame: &mut [u8], ch: char, pos: Point) {
        let sprite = &assets.cp437[to_cp437(ch) as usize];
        Screen::blit(frame, &pos, sprite);
    }

    pub fn print_cp437(&self, assets: &Assets, frame: &mut [u8], ch: FontCharType, pos: Point) {
        let sprite = &assets.cp437[ch as usize];
        Screen::blit(frame, &pos, sprite);
    }

    pub fn print_string(&self, assets: &Assets, frame: &mut [u8], str: &str, pos: Point) {
        // let str = "Hello world!";
        let chars = string_to_cp437(str);

        for (idx, ch) in chars.iter().enumerate() {
            let sprite = &assets.cp437[*ch as usize];
            Screen::blit(
                frame,
                &Point {
                    x: pos.x + idx as i32 * 8,
                    y: pos.y,
                },
                sprite,
            );
        }
    }

    pub fn draw_box(&self, assets: &Assets, frame: &mut [u8], pos: Point, size: Point) {
        let vertwall = 186;
        let horizwall = 205;
        let nwcorner = 201;
        let necorner = 187;
        let secorner = 188;
        let swcorner = 200;

        self.print_cp437(assets, frame, nwcorner, pos);
        self.print_cp437(
            assets,
            frame,
            necorner,
            Point {
                x: pos.x + size.x,
                y: pos.y,
            },
        );
        self.print_cp437(
            assets,
            frame,
            swcorner,
            Point {
                x: pos.x + size.x,
                y: pos.y + size.y,
            },
        );
        self.print_cp437(
            assets,
            frame,
            secorner,
            Point {
                x: pos.x,
                y: pos.y + size.y,
            },
        );

        for x in pos.x + 1..pos.x + size.x {
            self.print_cp437(assets, frame, horizwall, Point { x: x, y: pos.y });
            self.print_cp437(
                assets,
                frame,
                horizwall,
                Point {
                    x: x,
                    y: pos.y + size.y,
                },
            );
        }

        for y in pos.y + 1..pos.y + size.y {
            self.print_cp437(
                assets,
                frame,
                vertwall,
                Point {
                    x: pos.x + size.x,
                    y: y,
                },
            );
            self.print_cp437(assets, frame, vertwall, Point { x: pos.x, y: y });
        }

        // if x < 1 || x > map.width-2 || y < 1 || y > map.height-2 as i32 { return 35; }
        // let mut mask : u8 = 0;

        // if is_revealed_and_wall(map, x, y - 1) { mask +=1; }
        // if is_revealed_and_wall(map, x, y + 1) { mask +=2; }
        // if is_revealed_and_wall(map, x - 1, y) { mask +=4; }
        // if is_revealed_and_wall(map, x + 1, y) { mask +=8; }

        // match mask {
        //     0 => { 9 } // Pillar because we can't see neighbors
        //     1 => { 186 } // Wall only to the north
        //     2 => { 186 } // Wall only to the south
        //     3 => { 186 } // Wall to the north and south
        //     4 => { 205 } // Wall only to the west
        //     5 => { 188 } // Wall to the north and west
        //     6 => { 187 } // Wall to the south and west
        //     7 => { 185 } // Wall to the north, south and west
        //     8 => { 205 } // Wall only to the east
        //     9 => { 200 } // Wall to the north and east
        //     10 => { 201 } // Wall to the south and east
        //     11 => { 204 } // Wall to the north, south and east
        //     12 => { 205 } // Wall to the east and west
        //     13 => { 202 } // Wall to the east, west, and south
        //     14 => { 203 } // Wall to the east, west, and north
        //     15 => { 206 }  // ╬ Wall on all sides
        //     _ => { 35 } // We missed one?
        // }
    }

    pub fn draw_image(&self, image: &Image, frame: &mut [u8], pos: Point) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let image_buf = &image.0;
            let size = image.1;

            let xscreen = (i % WIDTH as usize) as i32;
            let yscreen = (i / WIDTH as usize) as i32;

            let xrange = self.pos.0..self.pos.0 + self.size.0;
            let yrange = self.pos.1..self.pos.1 + self.size.1;

            if xrange.contains(&xscreen) && yrange.contains(&yscreen) {
                let ximg = xscreen - self.pos.0;
                let yimg = yscreen - self.pos.1;

                let idx = yimg * size.1 as i32 + ximg;
                let rgba = image_buf[idx as usize];

                pixel.copy_from_slice(&rgba);
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
            let zipped = screen[i as usize..i as usize + width]
                .iter_mut()
                .zip(&pixels[s..s + width]);
            for (left, &right) in zipped {
                if right > 0 {
                    *left = right;
                }
                // *left = right;
            }

            s += width;
        }
    }

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
}
