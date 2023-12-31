use engine::map::TileType;

use crate::{colors::{self, Scale}, Game, WIDTH, assets::cp437_converter::to_cp437};

use super::{Glyph, GLYPH_SIZE, DEBUG_OUTLINES, UIState};

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
    pub mode: ConsoleMode,
    pub zoom: usize, // Only used for map mode
    pub map_pos: (usize, usize), // Only used for map mode
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

    pub fn render(&self, frame: &mut [u8], game: &Game) {
        match self.mode {
            ConsoleMode::MainMenu => {
                self.render_main_menu(frame, game);
            }
            ConsoleMode::WorldMap => {
                self.render_map(frame, game);
            }
            ConsoleMode::Log => {
                self.render_log(frame, game);
            }
        }

        if DEBUG_OUTLINES {
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let xscreen = i % WIDTH;
                let yscreen = i / WIDTH;

                if self.in_bounds((xscreen, yscreen)) &&
                    (xscreen == self.pos.0 || 
                    xscreen == self.pos.0 + self.size.0 ||
                    yscreen == self.pos.1 ||
                    yscreen == self.pos.1 + self.size.1 )
                {
                    pixel.copy_from_slice(&colors::COLOR_PURPLE);               
                }
            }
        }
    }

    pub fn render_main_menu(&self, frame: &mut [u8], game: &Game) {
        let screen = &game.screen;

        if let UIState::MainMenu{selection} = screen.ui_state {
            screen.draw_box(
                &game.assets,
                frame,
                self.pos,
                self.size,
                colors::COLOR_UI_1,
                colors::COLOR_BLACK_SEMI_TRANS // todo transparancy doesn't work
            );

            let x = self.pos.0 + 3 * GLYPH_SIZE;
            let mut y = self.pos.1 + 2 * GLYPH_SIZE;

            screen.print_string(
                &game.assets,
                frame,
                "Main Menu",
                (x, y),
                colors::COLOR_UI_2
            );

            y += 2 * GLYPH_SIZE;

            screen.print_string(
                &game.assets,
                frame,
                "Play Game",
                (x, y),
                // colors::COLOR_UI_2
                if selection == 0 { colors::COLOR_UI_3 } else { colors::COLOR_UI_2 }
            );

            y += GLYPH_SIZE;

            screen.print_string(
                &game.assets,
                frame,
                "Quit",
                (x, y),
                // colors::COLOR_UI_2
                if selection == 1 { colors::COLOR_UI_3 } else { colors::COLOR_UI_2 }
            );
        }
    }

    pub fn render_map(&self, frame: &mut [u8], game: &Game) {
        let map = &game.engine.map;
        let screen = &game.screen;

        let zoom = self.zoom; // each tile takes up zoom x zoom pixels

        if zoom < GLYPH_SIZE {
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
                            TileType::Water => colors::COLOR_WATER,
                            TileType::Sand => colors::COLOR_SAND,
                            TileType::Dirt => colors::COLOR_DIRT,
                            TileType::Stone => colors::COLOR_STONE,
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
                            TileType::Water => colors::COLOR_WATER,
                            TileType::Sand => colors::COLOR_SAND,
                            TileType::Dirt => colors::COLOR_DIRT,
                            TileType::Stone => colors::COLOR_STONE,
                        };
                        screen.print_cp437(
                            &game.assets,
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

    pub fn render_log(&self, frame: &mut [u8], game: &Game) {
        let screen = &game.screen;

        screen.draw_box(
            &game.assets,
            frame,
            (self.pos.0, self.pos.1),
            (self.size.0, self.size.1),
            colors::COLOR_UI_1,
            colors::COLOR_CLEAR
        );

        let mut y = 1;
        for m in game.game_log.iter().rev() {
            for ms in m.chars().collect::<Vec<_>>().chunks(self.size.0 / GLYPH_SIZE - 2) {
                if y * GLYPH_SIZE < self.size.1 - GLYPH_SIZE {
                    let s: String = ms.into_iter().collect();
                    screen.print_string(
                        &game.assets,
                        frame,
                        &s,
                        (self.pos.0 + GLYPH_SIZE, self.pos.1 + y * GLYPH_SIZE),
                        colors::COLOR_UI_2
                    );
                    y += 1;
                } else {
                    return; // todo this will be a bug if more is added to this function
                }
            }
        }
    }

    pub fn in_bounds(&self, pos: (usize, usize)) -> bool {
        return pos.0 >= self.pos.0 && 
            pos.0 <= self.pos.0 + self.size.0 && 
            pos.1 >= self.pos.1 &&
            pos.1 <= self.pos.1 + self.size.1
    }
}
