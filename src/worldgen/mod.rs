pub mod world;

use std::vec;

use rand::{Rng, thread_rng};
use rltk::Point;

use crate::map::{TileType, Map};

pub fn basic_fill(map: &mut Map) {
    let size = map.size;

    {
        let start1 = map.xy_idx(rnd_point(size));
        let start2 = map.xy_idx(rnd_point(size));
        let start3 = map.xy_idx(rnd_point(size));

        map.tiles[start1] = TileType::Dirt;
        map.tiles[start2] = TileType::Sand;
        map.tiles[start3] = TileType::Stone;
    }

    fill_recursive(map, 0);
}

fn fill_recursive(map: &mut Map, depth: i32) {
    if depth > 100 {
        return;
    }

    let mut new: Vec<TileType> = vec![TileType::Water; map.tiles.len()];
    let mut water = 0;
    let mut rng = thread_rng();

    for index in 0..map.tiles.len() {
        let tile = map.tiles[index];
        new[index] = tile;

        if tile == TileType::Water {
            water += 1;

            let neighbors = get_neighbors(map.idx_point(index));
            // let idx = Uniform::new(0, neighbors.len());//rng.next_u32() as usize * neighbors.len();
            let p = neighbors[rng.gen_range(0..neighbors.len())];
            // neighbors.shuffle(&mut rng);

            // for p in neighbors.iter() {
                if map.in_bounds((p.x.try_into().unwrap(), p.y.try_into().unwrap())) {
                    let idx = map.point_idx(p);
                    let t = map.tiles[idx];
                    if t != TileType::Water {
                        new[index] = t;
                        break;
                    }
                }
            // }
        }
    }

    map.tiles = new;

    if water > 100 {
        fill_recursive(map, depth + 1);
    }
}

fn rnd_point(size: (i32, i32)) -> (i32, i32) {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen();
    let y: f32 = rng.gen();
    ((x * size.0 as f32) as i32, (y * size.1 as f32) as i32)
}

pub fn get_neighbors(point: Point) -> Vec<Point> {
    vec![
        Point {
            x: point.x - 1,
            y: point.y - 1,
        },
        Point {
            x: point.x - 1,
            y: point.y,
        },
        Point {
            x: point.x - 1,
            y: point.y + 1,
        },
        Point {
            x: point.x,
            y: point.y - 1,
        },
        Point {
            x: point.x,
            y: point.y + 1,
        },
        Point {
            x: point.x + 1,
            y: point.y - 1,
        },
        Point {
            x: point.x + 1,
            y: point.y,
        },
        Point {
            x: point.x + 1,
            y: point.y + 1,
        },
    ]
}