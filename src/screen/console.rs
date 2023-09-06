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

use crate::{colors::{self, Scale}, map, World, WIDTH, assets::cp437_converter::to_cp437};

use super::Glyph;

#[derive(Debug)]
pub enum ConsoleMode {
    MainMenu,
    WorldMap,
    Log,
}

#[derive(Debug)]
pub struct Console {
    pub size: (usize, usize),
    pub pos: (usize, usize),
    pub children: Vec<Console>,
    pub hidden: bool,
    pub z: i32, // not used yet
    pub mode: ConsoleMode, //fns: destroy (with children)
    pub zoom: usize, // Only used for map object
    pub map_pos: (usize, usize), // Only used for map object
}

impl Console {
    pub fn new(size: (usize, usize), pos: (usize, usize), mode: ConsoleMode) -> Console {
        Self {
            size: size,
            pos: pos,
            children: vec![],
            hidden: false,
            z: 1,
            mode: mode,
            zoom: 1,
            map_pos: (0, 0),
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
                    (self.pos.0 + self.size.0 * 1 / 3 - 8, self.pos.1 + self.size.1 / 2 - 4 - 8),
                    (12 * 8, 2 * 8),
                    colors::COLOR_WHITE,
                    colors::COLOR_CLEAR
                );
                screen.print_string(
                    &world.assets,
                    frame,
                    "Hello World",
                    (self.pos.0 + self.size.0 * 1 / 3, self.pos.1 + self.size.1 / 2 - 4),
                );
            }
            ConsoleMode::WorldMap => {
                let zoom = self.zoom; // each tile takes up zoom x zoom pixels

                if zoom < 8 {
                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let xscreen = i % WIDTH;
                        let yscreen = i / WIDTH;
    
                        let xrange = self.pos.0..self.pos.0 + self.size.0;
                        let yrange = self.pos.1..self.pos.1 + self.size.1;
    
                        if xrange.contains(&xscreen) && yrange.contains(&yscreen) {
                            let xmap = self.map_pos.0 + (xscreen - self.pos.0) / zoom;
                            let ymap = self.map_pos.1 + (yscreen - self.pos.1) / zoom;

                            if map.in_bounds((xmap, ymap)) { 

                                let rgba = match map.get_tile((xmap, ymap)) {
                                    map::TileType::Water => colors::COLOR_DARK_BLUE,
                                    map::TileType::Sand => colors::COLOR_DESATURATED_YELLOW,
                                    map::TileType::Dirt => colors::COLOR_DARKER_GREEN,
                                    map::TileType::Stone => colors::COLOR_GREY,
                                };
        
                                pixel.copy_from_slice(&rgba);
                            }
                        }
                    }
                } else {
                    let widthchars = self.size.0 / zoom;
                    let heightchars = self.size.1 / zoom;
    
                    for x in 0 .. widthchars {
                        for y in 0 .. heightchars {
                            let pos = (x + self.map_pos.0, y + self.map_pos.1);
                            // let idx = map.point_idx(point);
                            if x < self.pos.0 + self.size.0 + zoom && y < self.pos.1 + self.size.1 + zoom && map.in_bounds(pos){
                                let rgba = match map.get_tile(pos) {
                                    map::TileType::Water => colors::COLOR_DARK_BLUE,
                                    map::TileType::Sand => colors::COLOR_DESATURATED_YELLOW,
                                    map::TileType::Dirt => colors::COLOR_DARKER_GREEN,
                                    map::TileType::Stone => colors::COLOR_GREY,
                                };
                                screen.print_cp437(
                                    &world.assets,
                                    frame,
                                    Glyph {
                                        pos: (self.pos.0 + x * zoom, self.pos.1 + y * zoom),
                                        ch: to_cp437(map.get_glyph(pos)),
                                        fg: rgba,
                                        bg: rgba.scale(0.5),
                                    }
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
                    (self.pos.0, self.pos.1),
                    (self.size.0 - 8, self.size.1 - 8),
                    colors::COLOR_WHITE,
                    colors::COLOR_CLEAR
                );
            }
        }
    }
}
