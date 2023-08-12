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

*/

use crate::{WIDTH, map::{Map, self}};

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
    pub mode: ScreenMode
}

impl Screen {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(size: (i32, i32), pos: (i32, i32)) -> Self {
        Self {
            size,
            pos,
            state: 0,
            input_blocking: false,
            mode: ScreenMode::ScreenTypeMainMenu
        }
    }

    pub fn draw(&self, frame: &mut [u8], map: &Map){
        match self.mode {
            ScreenMode::ScreenTypeMainMenu => {

            },
            ScreenMode::ScreenTypeLocalView => {

            },
            ScreenMode::ScreenTypeWorldView => {
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as i32;
                    let y = (i / WIDTH as usize) as i32;
        
                    let idx = map.xy_idx((x, y));
                    let rgba = match map.tiles[idx] {
                        map::TileType::Water => [0x0f, 0x5e, 0x9c, 0xff],
                        map::TileType::Sand => [0xe1, 0xbf, 0x92, 0xff],
                        map::TileType::Dirt => [0x40, 0x29, 0x05, 0xff],
                        map::TileType::Stone => [0x39, 0x3d, 0x47, 0xff],
                    };
        
                    pixel.copy_from_slice(&rgba);
                }
            },
        }
    }
}