
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
            size,
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

    pub fn get_glyph(&self, p: (usize, usize)) -> char {
        match self.tiles[self.xy_idx(p)] {
            TileType::Water => '~',
            TileType::Sand => '.',
            TileType::Dirt => '.',
            TileType::Stone => '#',
        }
    }
}