use std::collections::HashMap;

use engine::{
    components::Item,
    effects::{add_effect, EffectType},
    map::Map,
    player,
    uniques::{PPoint, PlayerID},
    utils::dir_to_point,
    GameMode, GameSettings,
};
use rltk::{Rltk, VirtualKeyCode};
use shipyard::{EntityId, Get, UniqueView, UniqueViewMut, View, World};

use crate::RunState;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InputCommand {
    None,
    Move { dir: i32 },
    ShowInventory,
    Wait,
    Escape,
    Get,
    Explore,
    RevealMap,
    Fireball,
    UseStairs,
}

impl InputCommand {
    fn execute(&self, world: &World, creator: Option<EntityId>) -> RunState {
        // let map = world.borrow::<UniqueView<Map>>().unwrap();

        // let player_pos = world.borrow::<UniqueView<PPoint>>().unwrap().0;
        // let player_pos_idx = map.point_idx(player_pos);

        // // return RunState::AwaitingInput to ignore input, RunState::PlayerTurn to advance engine
        // return match self {
        //     InputCommand::None => RunState::AwaitingInput,
        //     InputCommand::Move { dir } => {
        //         // hold shift to move by 10 squares at a time
        //         let movemod = 1;

        //         let mut dir_targets: HashMap<i32, usize> = HashMap::new();
        //         dir_targets.insert(1, map.point_idx(dir_to_point(player_pos, 1, movemod)));
        //         dir_targets.insert(2, map.point_idx(dir_to_point(player_pos, 2, movemod)));
        //         dir_targets.insert(3, map.point_idx(dir_to_point(player_pos, 3, movemod)));
        //         dir_targets.insert(4, map.point_idx(dir_to_point(player_pos, 4, movemod)));
        //         dir_targets.insert(6, map.point_idx(dir_to_point(player_pos, 6, movemod)));
        //         dir_targets.insert(7, map.point_idx(dir_to_point(player_pos, 7, movemod)));
        //         dir_targets.insert(8, map.point_idx(dir_to_point(player_pos, 8, movemod)));
        //         dir_targets.insert(9, map.point_idx(dir_to_point(player_pos, 9, movemod)));

        //         add_effect(
        //             creator,
        //             EffectType::MoveOrAttack {
        //                 tile_idx: dir_targets[dir],
        //             },
        //         );

        //         RunState::PlayerTurn
        //     }
        //     InputCommand::ShowInventory => RunState::ShowInventory,
        //     InputCommand::Wait => {
        //         add_effect(creator, EffectType::Wait {}); //todo is this weird on sim mode?
        //         RunState::PlayerTurn
        //     }
        //     InputCommand::Escape => RunState::EscPressed,
        //     InputCommand::Get => {
        //         world.run(|vitem: View<Item>| {
        //             for e in map.tile_content[player_pos_idx].iter() {
        //                 if let Ok(_) = vitem.get(*e) {
        //                     add_effect(creator, EffectType::PickUp { entity: *e });
        //                 }
        //             }
        //         });

        //         RunState::PlayerTurn
        //     }
        //     InputCommand::Explore => {
        //         add_effect(creator, EffectType::Explore {});

        //         RunState::PlayerTurn
        //     }
        //     InputCommand::RevealMap => {
        //         player::reveal_map(&world);

        //         RunState::PlayerTurn
        //     }
        //     InputCommand::Fireball => {
        //         dbg!("fireball is broken");
        //         RunState::AwaitingInput
        //         // RunState::ShowTargeting {
        //         //     range: 6,
        //         //     item: world.run(|mut store: AllStoragesViewMut| {
        //         //         entity_factory::tmp_fireball(&mut store)
        //         //     }),
        //         // }
        //     }
        //     InputCommand::UseStairs => {
        //         if player::try_next_level(&world) {
        //             RunState::NextLevel
        //         } else {
        //             RunState::AwaitingInput
        //         }
        //     }
        // };
    }
}

pub fn map_keys(ctx: &Rltk, mode: GameMode) -> InputCommand {
    match mode {
        GameMode::RL | GameMode::OrcHalls => match ctx.key {
            None => InputCommand::None,
            Some(key) => match key {
                VirtualKeyCode::Left => InputCommand::Move { dir: 4 },
                VirtualKeyCode::Right => InputCommand::Move { dir: 6 },
                VirtualKeyCode::Up => InputCommand::Move { dir: 8 },
                VirtualKeyCode::Down => InputCommand::Move { dir: 2 },
                VirtualKeyCode::Y => InputCommand::Move { dir: 7 },
                VirtualKeyCode::U => InputCommand::Move { dir: 9 },
                VirtualKeyCode::N => InputCommand::Move { dir: 3 },
                VirtualKeyCode::B => InputCommand::Move { dir: 1 },
                VirtualKeyCode::G => InputCommand::Get,
                VirtualKeyCode::X => InputCommand::Explore,
                VirtualKeyCode::R => InputCommand::RevealMap,
                VirtualKeyCode::F => InputCommand::Fireball,
                VirtualKeyCode::I => InputCommand::ShowInventory,
                VirtualKeyCode::W => InputCommand::Wait,
                VirtualKeyCode::Escape => InputCommand::Escape,
                VirtualKeyCode::Period => InputCommand::UseStairs,
                _ => InputCommand::None,
            },
        },
        GameMode::VillageSim => match ctx.key {
            None => InputCommand::None,
            Some(key) => match key {
                VirtualKeyCode::Left => InputCommand::Move { dir: 4 },
                VirtualKeyCode::Right => InputCommand::Move { dir: 6 },
                VirtualKeyCode::Up => InputCommand::Move { dir: 8 },
                VirtualKeyCode::Down => InputCommand::Move { dir: 2 },
                VirtualKeyCode::Y => InputCommand::Move { dir: 7 },
                VirtualKeyCode::U => InputCommand::Move { dir: 9 },
                VirtualKeyCode::N => InputCommand::Move { dir: 3 },
                VirtualKeyCode::B => InputCommand::Move { dir: 1 },
                VirtualKeyCode::F => InputCommand::Fireball,
                VirtualKeyCode::W => InputCommand::Wait,
                VirtualKeyCode::Escape => InputCommand::Escape,
                _ => InputCommand::None,
            },
        },
    }
}

pub fn handle_input(world: &World, ctx: &Rltk) -> RunState {
    let settings = world.borrow::<UniqueView<GameSettings>>().unwrap();
    let player_id = world.borrow::<UniqueViewMut<PlayerID>>().unwrap().0;

    let command = map_keys(ctx, settings.mode);

    return command.execute(world, Some(player_id));
}
