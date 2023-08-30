use winit::event::{Event, VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

use crate::{World, screen::console::ConsoleMode};

pub enum Action {
    None,
    Exit
}

pub fn handle_input(input: &WinitInputHelper, world: &mut World) -> Action {

    // Esc
    if input.key_pressed(VirtualKeyCode::Escape) {
        return Action::Exit
    }

    // V
    if input.key_pressed(VirtualKeyCode::V) {
        world.screen.consoles[1].mode = match world.screen.consoles[1].mode {
            ConsoleMode::MainMenu => ConsoleMode::WorldMap,
            ConsoleMode::LocalMap => ConsoleMode::WorldMap,
            ConsoleMode::WorldMap => ConsoleMode::LocalMap,
            ConsoleMode::Log => ConsoleMode::Log,
        }
    }

    return Action::None
}
