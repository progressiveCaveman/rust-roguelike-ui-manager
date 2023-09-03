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

use crate::{colors, map, World, WIDTH};
use rltk::Point;

#[derive(Debug)]
pub enum ConsoleMode {
    MainMenu,
    WorldMap,
    Log,
}

#[derive(Debug)]
pub struct Console {
    pub size: (i32, i32),
    pub pos: (i32, i32),
    pub children: Vec<Console>,
    pub hidden: bool,
    pub z: i32, // not used yet
    pub mode: ConsoleMode, //fns: destroy (with children)
    pub zoom: usize, // Only used for map object
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
            zoom: 1,
        }
    }

    pub fn render(&self, frame: &mut [u8], world: &World) {
        let map = &world.map;
        let screen = &world.screen;
        // let gsize = world.glyph_size;

        // dbg!(&self.mode);

        match &self.mode {
            ConsoleMode::MainMenu => {
                screen.draw_box(
                    &world.assets,
                    frame,
                    Point {
                        x: self.pos.0 + self.size.0 * 1 / 3 - 8,
                        y: self.pos.1 + self.size.1 / 2 - 4 - 8,
                    },
                    Point {
                        x: 12 * 8,
                        y: 2 * 8,
                    },
                );
                screen.print_string(
                    &world.assets,
                    frame,
                    "Hello World",
                    Point {
                        x: self.pos.0 + self.size.0 * 1 / 3,
                        y: self.pos.1 + self.size.1 / 2 - 4,
                    },
                );
            }
            ConsoleMode::WorldMap => {
                let zoom = self.zoom; // each tile takes up zoom x zoom pixels

                if zoom < 8 {
                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let xscreen = (i % WIDTH as usize) as i32;
                        let yscreen = (i / WIDTH as usize) as i32;
    
                        let xrange = self.pos.0..self.pos.0 + self.size.0;
                        let yrange = self.pos.1..self.pos.1 + self.size.1;
    
                        if xrange.contains(&xscreen) && yrange.contains(&yscreen) {
                            let xmap = (xscreen - self.pos.0) / zoom as i32;
                            let ymap = (yscreen - self.pos.1) / zoom as i32;
    
                            let idx = map.xy_idx((xmap, ymap));
                            let rgba = match map.tiles[idx] {
                                map::TileType::Water => colors::COLOR_DARK_BLUE,
                                map::TileType::Sand => colors::COLOR_DESATURATED_YELLOW,
                                map::TileType::Dirt => colors::COLOR_DARKER_GREEN,
                                map::TileType::Stone => colors::COLOR_GREY,
                            };
    
                            pixel.copy_from_slice(&rgba);
                        }
                    }
                } else {
                    let widthchars = self.size.0 / zoom as i32;
                    let heightchars = self.size.1 / zoom as i32;
    
                    for x in 0..widthchars {
                        for y in 0..heightchars {
                            // todo check bounds
                            if x < self.pos.0 + self.size.0 + zoom as i32
                                && y < self.pos.1 + self.size.1 + zoom as i32
                            {
                                screen.print_char(
                                    &world.assets,
                                    frame,
                                    map.get_glyph(Point { x, y }),
                                    Point {
                                        x: self.pos.0 + x * zoom as i32,
                                        y: self.pos.1 + y * zoom as i32,
                                    },
                                );
                            }
                        }
                    }
                }
            }
            ConsoleMode::Log => {
                screen.draw_box(
                    &world.assets,
                    frame,
                    Point {
                        x: self.pos.0,
                        y: self.pos.1,
                    },
                    Point {
                        x: self.size.0 - 8,
                        y: self.size.1 - 8,
                    },
                );
            }
        }
    }
}
