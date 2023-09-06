use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::{World, worldgen::basic_fill};

pub enum Action {
    None,
    Exit,
}

pub fn handle_input(input: &WinitInputHelper, world: &mut World) -> Action {
    // Esc : Exit
    if input.key_pressed(VirtualKeyCode::Escape) {
        return Action::Exit;
    }

    // V
    // if input.key_pressed(VirtualKeyCode::V) {
    //     world.screen.consoles[1].mode = match world.screen.consoles[1].mode {
    //         ConsoleMode::MainMenu => ConsoleMode::WorldMap,
    //         ConsoleMode::LocalMap => ConsoleMode::WorldMap,
    //         ConsoleMode::WorldMap => ConsoleMode::LocalMap,
    //         ConsoleMode::Log => ConsoleMode::Log,
    //     }
    // }

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

    // Up : pan map
    if input.key_pressed_os(VirtualKeyCode::Up) {
        world.screen.pan_map((0, -1 * movemod));
    }

    // Down : pan map
    if input.key_pressed_os(VirtualKeyCode::Down) {
        world.screen.pan_map((0, 1 * movemod));
    }

    // Left : pan map
    if input.key_pressed_os(VirtualKeyCode::Left) {
        world.screen.pan_map((-1 * movemod, 0));
    }

    // Right : pan map
    if input.key_pressed_os(VirtualKeyCode::Right) {
        world.screen.pan_map((1 * movemod, 0));
    }

    return Action::None;
}
