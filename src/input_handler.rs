use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

use crate::World;

pub enum Action {
    None,
    Exit,
}

pub fn handle_input(input: &WinitInputHelper, world: &mut World) -> Action {
    // Esc
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

    // +
    if input.key_pressed(VirtualKeyCode::Equals) {
        world.screen.increment_zoom();
    }

    // -
    if input.key_pressed(VirtualKeyCode::Minus) {
        world.screen.decrement_zoom();
    }

    return Action::None;
}
