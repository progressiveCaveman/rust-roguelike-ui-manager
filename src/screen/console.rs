/*

Each screen will have a 
state
size
position
menustate index

any screen can have any number of screens embedded?
main screen has some named components?
Simplify render to use map and only render component

Have a screen mode and an active screen
Screen mode defines game mode behavior like controls

Representing the screen flow
One file is a state machine for the screen flow
Implementation is broken off into different file Eventually

Representing menu
Message
[Options]
how to pass control flow?

Screen always has an active menu
No mouse interaction to start


Initial use cases:
Main menu
inventory screen
item label


Targeting is a special function of a screen?


console types:
Main menu
Any submenus
local map
world map
log
stats
inventory
ais
overlays
label

*/

use rltk::Point;
use crate::{World, WIDTH, map, HEIGHT};

#[derive(Debug)]
pub enum ConsoleMode {
    MainMenu,
    LocalMap,
    WorldMap,
    Log
}

#[derive(Debug)]
pub struct Console {
    pub size: (i32, i32),
    pub pos: (i32, i32),
    pub children: Vec<Console>,
    pub hidden: bool,
    pub z: i32,
    pub mode: ConsoleMode
    //fns: destroy (with children)
}

impl Console {
    pub fn new(size: (i32, i32), pos: (i32, i32), mode: ConsoleMode) -> Console {
        Self { 
            size: size, 
            pos: pos, 
            children: vec![], 
            hidden: false, 
            z: 1, 
            mode: mode,
        }
    }

    pub fn render(&self, frame: &mut [u8], world: &World) {
        let map = &world.map;
        let screen = &world.screen;
        let gsize = world.glyph_size;

        // dbg!(&self.mode);

        match &self.mode {
            ConsoleMode::MainMenu => {

            },
            ConsoleMode::LocalMap => {
                let widthchars = self.size.0 / gsize;
                let heightchars = self.size.1 / gsize;

                for x in 0..widthchars {
                    for y in 0..heightchars {
                        // todo check bounds
                        dbg!(x,y);
                        dbg!(self.pos);
                        dbg!(self.size);
                        if x < self.pos.0 + self.size.0 + gsize && y < self.pos.1 + self.size.1 + gsize {
                            screen.print_char(&world.assets, frame, map.get_glyph(Point { x, y }), Point { x: self.pos.0 + x * gsize, y: self.pos.1 + y * gsize});                            
                        }
                    }
                }

                // for x in self.pos.0 .. self.pos.0 + (self.size.0 / gsize) {
                //     for y in self.pos.1 .. self.pos.1 + (self.size.1 / gsize) {
                //         // todo bounds check
                //         screen.print_char(&world.assets, frame, map.get_glyph(Point { x, y }), Point { x: x * gsize, y: y * gsize });
                //     }
                // }
            },
            ConsoleMode::WorldMap => {
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let xscreen = (i % WIDTH as usize) as i32;
                    let yscreen = (i / WIDTH as usize) as i32;

                    let xrange = self.pos.0 .. self.pos.0 + self.size.0;
                    let yrange = self.pos.1 .. self.pos.1 + self.size.1;

                    if xrange.contains(&xscreen) && yrange.contains(&yscreen) {
                        let xmap = (xscreen - self.pos.0) / gsize;
                        let ymap = (yscreen - self.pos.1) / gsize;

                        let idx = map.xy_idx((xmap, ymap));
                        let rgba = match map.tiles[idx] {
                            map::TileType::Water => [0x0f, 0x5e, 0x9c, 0xff],
                            map::TileType::Sand => [0xe1, 0xbf, 0x92, 0xff],
                            map::TileType::Dirt => [0x40, 0x29, 0x05, 0xff],
                            map::TileType::Stone => [0x39, 0x3d, 0x47, 0xff],
                        };

                        pixel.copy_from_slice(&rgba);
                    }        
                }
            },
            _ => {
                screen.draw_box(&world.assets, frame, Point { x: WIDTH * 1/3 - 8, y: HEIGHT/2 - 4 - 8 }, Point { x: 12 * 8, y: 2 * 8 });
                screen.print_string(&world.assets, frame, "Hello World", Point { x: WIDTH * 1/3, y: HEIGHT/2 - 4 });        
            }
        }
    }
}