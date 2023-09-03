pub mod world;

use std::vec;

use rand::Rng;

use crate::{map::{Map, TileType}, Point};

pub fn basic_fill(map: &mut Map) {
    let size = map.size;

    for i in 0..map.tiles.len() {
        map.tiles[i] = TileType::Water;
    }

    for i in 0..240 {
        let start1 = map.xy_idx(rnd_point(size));
        if i % 3 == 0 {
            map.tiles[start1] = TileType::Dirt;
        } else if i % 3 == 1 {
            map.tiles[start1] = TileType::Sand;
        } else {
            map.tiles[start1] = TileType::Stone;
        }
    }

    fill_recursive(map, 0);
}

fn fill_recursive(map: &mut Map, depth: i32) {
    if depth > 200 {
        return;
    }

    let mut new: Vec<TileType> = vec![TileType::Water; map.tiles.len()];
    let mut water = 0;

    for index in 0..map.tiles.len() {
        let tile = map.tiles[index];
        new[index] = tile;

        if tile == TileType::Water {
            water += 1;

            let neighbors = get_neighbors(map.idx_point(index));

            for p in neighbors.iter() {
                if map.in_bounds((p.x.try_into().unwrap(), p.y.try_into().unwrap())) {
                    let idx = map.point_idx(*p);
                    let t = map.tiles[idx];
                    if t != TileType::Water {
                        new[index] = t;
                        break;
                    }
                }
            }
        }
    }

    map.tiles = new;

    if water > 100 {
        fill_recursive(map, depth + 1);
    }
}

fn rnd_point(size: (usize, usize)) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen();
    let y: f32 = rng.gen();
    ((x * size.0 as f32) as usize, (y * size.1 as f32) as usize)
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
