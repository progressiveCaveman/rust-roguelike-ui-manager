use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{World, worldgen::basic_fill, screen::UIState};

pub enum Action {
    None,
    Exit,
    // Up, Down, Left, Right,
    // Select,
    // ZoomIn, ZoomOut
}

pub fn handle_input(input: &WinitInputHelper, world: &mut World) -> Action {
    // Esc : Exit
    if input.key_pressed(VirtualKeyCode::Escape) {
        return Action::Exit;
    }

    // + : zoom in
    if input.key_pressed_os(VirtualKeyCode::Equals) {
        world.screen.increment_zoom();
    }

    // - : zoom out
    if input.key_pressed_os(VirtualKeyCode::Minus) {
        world.screen.decrement_zoom();
    }

    // R : refresh worldgen
    if input.key_pressed_os(VirtualKeyCode::R) {
        basic_fill(&mut world.map);
    }

    let movemod = if input.held_shift() {
        10
    } else {
        1
    };

    // Up
    if input.key_pressed_os(VirtualKeyCode::Up) {
        match world.screen.ui_state {
            UIState::Game => {
                world.screen.pan_map((0, -1 * movemod));
            },
            UIState::MainMenu { selection } => {
                world.screen.ui_state = UIState::MainMenu { selection: selection - 1 };
            },
        }
    }

    // Down
    if input.key_pressed_os(VirtualKeyCode::Down) {

        match world.screen.ui_state {
            UIState::Game => {
                world.screen.pan_map((0, 1 * movemod));
            },
            UIState::MainMenu { selection } => {
                world.screen.ui_state = UIState::MainMenu { selection: selection + 1 };
            },
        }
    }

    // Left
    if input.key_pressed_os(VirtualKeyCode::Left) {

        match world.screen.ui_state {
            UIState::Game => {
                world.screen.pan_map((-1 * movemod, 0));
            },
            _ => {
                
            },
        }
    }

    // Right
    if input.key_pressed_os(VirtualKeyCode::Right) {

        match world.screen.ui_state {
            UIState::Game => {
                world.screen.pan_map((1 * movemod, 0));
            },
            _ => {
                
            },
        }
    }

    // Enter
    if input.key_pressed_os(VirtualKeyCode::Return) {
        match world.screen.ui_state {
            UIState::MainMenu { selection } => {
                if selection == 0 { //play game
                    world.screen.ui_state = UIState::Game;
                }else if selection == 1 { // exit
                    return Action::Exit;
                }
            },
            _ => {}
        }
    }

    return Action::None;
}
