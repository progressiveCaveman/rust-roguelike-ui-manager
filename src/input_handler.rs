use engine::worldgen;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{Game, screen::UIState};

pub enum Action {
    None,
    Exit,
}

pub fn handle_input(input: &WinitInputHelper, game: &mut Game) -> Action {
    // Esc : Exit
    if input.key_pressed(VirtualKeyCode::Escape) {
        match game.screen.ui_state {
            UIState::Game => {
                game.screen.ui_state = UIState::MainMenu { selection: 0 }
            },
            UIState::MainMenu { selection: _ } => {
                return Action::Exit;
            },
        }
    }

    // + : zoom in
    if input.key_pressed_os(VirtualKeyCode::Equals) {
        game.screen.increment_zoom();
    }

    // - : zoom out
    if input.key_pressed_os(VirtualKeyCode::Minus) {
        game.screen.decrement_zoom();
    }

    // R : refresh worldgen
    if input.key_pressed_os(VirtualKeyCode::R) {
        worldgen::basic_fill(&mut game.engine.map);
    }

    let movemod = if input.held_shift() {
        10
    } else {
        1
    };

    // Up
    if input.key_pressed_os(VirtualKeyCode::Up) {
        match game.screen.ui_state {
            UIState::Game => {
                game.screen.pan_map((0, -1 * movemod));
            },
            UIState::MainMenu { selection } => {
                game.screen.ui_state = UIState::MainMenu { selection: selection - 1 };
            },
        }
    }

    // Down
    if input.key_pressed_os(VirtualKeyCode::Down) {
        match game.screen.ui_state {
            UIState::Game => {
                game.screen.pan_map((0, 1 * movemod));
            },
            UIState::MainMenu { selection } => {
                game.screen.ui_state = UIState::MainMenu { selection: selection + 1 };
            },
        }
    }

    // Left
    if input.key_pressed_os(VirtualKeyCode::Left) {
        match game.screen.ui_state {
            UIState::Game => {
                game.screen.pan_map((-1 * movemod, 0));
            },
            _ => {},
        }
    }

    // Right
    if input.key_pressed_os(VirtualKeyCode::Right) {
        match game.screen.ui_state {
            UIState::Game => {
                game.screen.pan_map((1 * movemod, 0));
            },
            _ => {},
        }
    }

    // Enter
    if input.key_pressed_os(VirtualKeyCode::Return) {
        match game.screen.ui_state {
            UIState::MainMenu { selection } => {
                if selection == 0 { //play game
                    game.screen.ui_state = UIState::Game;
                }else if selection == 1 { // exit
                    return Action::Exit;
                }
            },
            _ => {}
        }
    }

    return Action::None;
}
