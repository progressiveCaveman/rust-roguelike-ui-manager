use map::Map;

pub mod map;
pub mod worldgen;

/*
This is where the main game engine stuff goes. Basically the ECS and whatever else makes sense
*/

pub struct Engine {
    // pub world: World,
    pub map: Map,
    pub first_run: bool,
}

impl Engine {
    pub fn new(map_size: (usize, usize)) -> Self {
        Self {
            map: Map::new(map_size),
            first_run: false,
        }
    }
    pub fn run_systems() {

    }

    pub fn reset() {

    }
}