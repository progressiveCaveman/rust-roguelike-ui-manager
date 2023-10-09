
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Water,
    Sand,
    Dirt,
    Stone,
}

#[derive(Default, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub size: (usize, usize),
}

impl Map {
    pub fn new(size: (usize, usize)) -> Map {
        let count = (size.0 * size.1) as usize;
        Map {
            tiles: vec![TileType::Water; count],
            size, // influence_maps:vec![vec![0.0; count]; 2],// todo magic numbers
        }
    }

    pub fn len(&self) -> usize {
        self.size.0 * self.size.1
    }

    pub fn get_tile(&self, xy: (usize, usize)) -> TileType {
        let idx = self.xy_idx(xy);
        self.tiles[idx]
    }

    pub fn set_tile(&mut self, xy: (usize, usize), value: TileType) {
        let idx = self.xy_idx(xy);
        self.tiles[idx] = value;
    }

    pub fn xy_idx(&self, xy: (usize, usize)) -> usize {
        (xy.1 as usize * self.size.0 as usize) + xy.0 as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (usize, usize) {
        (idx as usize % self.size.0, idx as usize / self.size.0)
    }

    pub fn in_bounds(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.size.0 && pos.1 < self.size.1
    }

    // fn is_exit_valid(&self, x: usize, y: usize) -> bool {
    //     if x < 1 || x >= self.size.0 || y < 1 || y >= self.size.1 {
    //         return false;
    //     }
    //     return true;
    // }

    pub fn get_glyph(&self, p: (usize, usize)) -> char {
        match self.tiles[self.xy_idx(p)] {
            TileType::Water => '~',
            TileType::Sand => '.',
            TileType::Dirt => '.',
            TileType::Stone => '#',
        }
    }
}

// impl Algorithm2D for Map {
//     fn dimensions(&self) -> Point {
//         Point::new(self.size.0, self.size.1)
//     }
// }

// impl BaseMap for Map {
//     // fn is_opaque(&self, idx: usize) -> bool {
//     //     false
//     //     // self.tiles[idx] == TileType::Wall
//     //     //     || self.tiles[idx] == TileType::Wheat
//     //     //     || self.tiles[idx] == TileType::WoodWall
//     //     //     || self.tiles[idx] == TileType::WoodDoor // TODO make fire block too?
//     // }

//     fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
//         let w = self.size.0 as usize;
//         let p1 = Point::new(idx1 % w, idx1 / w);
//         let p2 = Point::new(idx2 % w, idx2 / w);
//         rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
//     }

//     fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
//         let mut exits = rltk::SmallVec::new();
//         let (x, y) = self.idx_xy(idx);
//         let w = self.size.0 as usize;

//         if self.is_exit_valid(x - 1, y) {
//             exits.push((idx - 1, 1.0))
//         };
//         if self.is_exit_valid(x + 1, y) {
//             exits.push((idx + 1, 1.0))
//         };
//         if self.is_exit_valid(x, y - 1) {
//             exits.push((idx - w, 1.0))
//         };
//         if self.is_exit_valid(x, y + 1) {
//             exits.push((idx + w, 1.0))
//         };

//         if self.is_exit_valid(x - 1, y - 1) {
//             exits.push((idx - w - 1, 1.45))
//         };
//         if self.is_exit_valid(x + 1, y - 1) {
//             exits.push((idx - w + 1, 1.45))
//         };
//         if self.is_exit_valid(x - 1, y + 1) {
//             exits.push((idx + w - 1, 1.45))
//         };
//         if self.is_exit_valid(x + 1, y + 1) {
//             exits.push((idx + w + 1, 1.45))
//         };

//         exits
//     }
// }
