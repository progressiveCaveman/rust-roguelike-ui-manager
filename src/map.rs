use rltk::{Point, Algorithm2D, BaseMap};
use serde::{Serialize, Deserialize};
use shipyard::Unique;

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TileType {
    Water,
    Sand,
    Dirt,
    Stone
}

#[derive(Default, Serialize, Deserialize, Clone, Unique)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub size: (i32, i32)
}

impl Map {
    pub fn new(tile_type: TileType, size: (i32, i32)) -> Map {
        let count = (size.0 * size.1) as usize;
        Map {
            tiles: vec![tile_type; count],
            size
            // influence_maps:vec![vec![0.0; count]; 2],// todo magic numbers
        }
    }

    pub fn set_tile(&mut self, xy: (i32, i32), value: TileType) {
        let idx = self.xy_idx(xy);
        self.tiles[idx] = value;
    }

    pub fn xy_idx(&self, xy: (i32, i32)) -> usize {
        (xy.1 as usize * self.size.0 as usize) + xy.0 as usize
    }

    pub fn point_idx(&self, point: Point) -> usize {
        (point.y as usize * self.size.0 as usize) + point.x as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        (idx as i32 % self.size.0, idx as i32 / self.size.0)
    }

    pub fn idx_point(&self, idx: usize) -> Point {
        Point {
            x: (idx as i32 % self.size.0) as i32,
            y: (idx as i32 / self.size.0) as i32,
        }
    }

    pub fn in_bounds(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0 && pos.0 < self.size.0 && pos.1 >= 0 && pos.1 < self.size.1
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x >= self.size.0 || y < 1 || y >= self.size.1 {
            return false;
        }
        return true;
        // let idx = self.xy_idx(x, y);
        // !self.blocked[idx]
    }

    // pub fn distance(&self, vpos: &View<Position>, f: Target, t: Target) -> f32 {
    //     let idx1 = match f {
    //         Target::LOCATION(l) => vec![self.xy_idx(l.x, l.y)],
    //         Target::ENTITY(e) => {
    //             if let Ok(p) = vpos.get(e) {
    //                 p.idxes(self)
    //             } else {
    //                 vec![0]
    //             }
    //         }
    //     };

    //     let idx2 = match t {
    //         Target::LOCATION(l) => vec![self.xy_idx(l.x, l.y)],
    //         Target::ENTITY(e) => {
    //             if let Ok(p) = vpos.get(e) {
    //                 p.idxes(self)
    //             } else {
    //                 vec![0]
    //             }
    //         }
    //     };

    //     let mut min = f32::MAX;
    //     for i1 in idx1.iter() {
    //         for i2 in idx2.iter() {
    //             let dist = self.get_pathing_distance(*i1, *i2);
    //             if dist < min {
    //                 min = dist;
    //             }
    //         }
    //     }

    //     min
    // }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.size.0, self.size.1)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        false
        // self.tiles[idx] == TileType::Wall
        //     || self.tiles[idx] == TileType::Wheat
        //     || self.tiles[idx] == TileType::WoodWall
        //     || self.tiles[idx] == TileType::WoodDoor // TODO make fire block too?
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.size.0 as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let (x, y) = self.idx_xy(idx);
        let w = self.size.0 as usize;

        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + w, 1.0))
        };

        if self.is_exit_valid(x - 1, y - 1) {
            exits.push((idx - w - 1, 1.45))
        };
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push((idx - w + 1, 1.45))
        };
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push((idx + w - 1, 1.45))
        };
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push((idx + w + 1, 1.45))
        };

        exits
    }
}
