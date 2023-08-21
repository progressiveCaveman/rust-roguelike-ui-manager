use rltk::Point;
use crate::{WIDTH, cp437::{converter::{string_to_cp437, to_cp437, FontCharType}, Assets, blit, sprites::Drawable}, HEIGHT, World, Image};

use self::console::{Console, ConsoleMode};

pub mod console;

pub enum ScreenMode {
    ScreenTypeMainMenu,
    ScreenTypeLocalView,
    ScreenTypeWorldView
}

pub struct Screen {
    pub size: (i32, i32),
    pub pos: (i32, i32),
    pub state: i32,
    pub input_blocking: bool,
    pub mode: ScreenMode,
    pub consoles: Vec<Console>
}

impl Screen {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(size: (i32, i32), pos: (i32, i32)) -> Self {
        Self {
            size,
            pos,
            state: 0,
            input_blocking: false,
            mode: ScreenMode::ScreenTypeMainMenu,
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
        let y = y + w;
        let w = w;
        let h = self.size.1 - h;
        self.consoles.push(Console::new((w, h), (x, y), ConsoleMode::WorldMap));
    }

    pub fn draw(&self, frame: &mut [u8], world: &World){
        for c in self.consoles.iter() {
            c.render(frame, world);
        }
        // let map = &world.map;
        // let screen = &world.screen;
        // let gsize = world.glyph_size;

        // match self.mode {
        //     ScreenMode::ScreenTypeMainMenu => {
        //         screen.draw_box(&world.assets, frame, Point { x: WIDTH * 1/3 - 8, y: HEIGHT/2 - 4 - 8 }, Point { x: 12 * 8, y: 2 * 8 });
        //         screen.print_string(&world.assets, frame, "Hello World", Point { x: WIDTH * 1/3, y: HEIGHT/2 - 4 });        
        //     },
        //     ScreenMode::ScreenTypeLocalView => {
        //         let width = cmp::min(self.size.0 / gsize, map.size.0 / gsize);
        //         let height = cmp::min(self.size.1 / gsize, map.size.1 / gsize);

        //         for x in 0 .. width {
        //             for y in 0 .. height {
        //                 let xm = x + self.pos.0;
        //                 let ym = y + self.pos.1;
        //                 if map.in_bounds((xm, ym)) {
        //                     let p = Point { x: xm, y: ym };
        //                     screen.print_char(&world.assets, frame, map.get_glyph(p), Point { x: x * gsize, y: y * gsize });
        //                 }
        //             }
        //         }
        //     },
        //     ScreenMode::ScreenTypeWorldView => {
        //         for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        //             let x = (i % WIDTH as usize) as i32;
        //             let y = (i / WIDTH as usize) as i32;
        
        //             let idx = map.xy_idx((x, y));
        //             let rgba = match map.tiles[idx] {
        //                 map::TileType::Water => [0x0f, 0x5e, 0x9c, 0xff],
        //                 map::TileType::Sand => [0xe1, 0xbf, 0x92, 0xff],
        //                 map::TileType::Dirt => [0x40, 0x29, 0x05, 0xff],
        //                 map::TileType::Stone => [0x39, 0x3d, 0x47, 0xff],
        //             };
        
        //             pixel.copy_from_slice(&rgba);
        //         }
        //     },
        // }
    }

    pub fn print_char(&self, assets: &Assets, frame: &mut [u8], ch: char, pos: Point) {
        let sprite = &assets.cp437[to_cp437(ch) as usize];
        blit(frame, &pos, sprite);
    }

    pub fn print_cp437(&self, assets: &Assets, frame: &mut [u8], ch: FontCharType, pos: Point) {
        let sprite = &assets.cp437[ch as usize];
        blit(frame, &pos, sprite);
    }

    pub fn print_string(&self, assets: &Assets, frame: &mut [u8], str: &str, pos: Point) {
        // let str = "Hello world!";
        let chars = string_to_cp437(str);

        for (idx, ch) in chars.iter().enumerate() {
            let sprite = &assets.cp437[*ch as usize];
            blit(frame, &Point{ x:pos.x + idx as i32 * 8, y:pos.y }, sprite);
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
        self.print_cp437(assets, frame, necorner, Point { x: pos.x + size.x, y: pos.y });
        self.print_cp437(assets, frame, swcorner, Point { x: pos.x + size.x, y: pos.y + size.y });
        self.print_cp437(assets, frame, secorner, Point { x: pos.x, y: pos.y + size.y });

        for x in pos.x + 1 .. pos.x + size.x {
            self.print_cp437(assets, frame, horizwall, Point { x: x, y: pos.y });
            self.print_cp437(assets, frame, horizwall, Point { x: x, y: pos.y + size.y });
        }

        for y in pos.y + 1 .. pos.y + size.y {
            self.print_cp437(assets, frame, vertwall, Point { x: pos.x + size.x, y: y });
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

            let xrange = self.pos.0 .. self.pos.0 + self.size.0;
            let yrange = self.pos.1 .. self.pos.1 + self.size.1;

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