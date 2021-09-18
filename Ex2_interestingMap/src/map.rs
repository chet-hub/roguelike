use bracket_pathfinding::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
use specs_derive::*;
use specs::prelude::*;
use std::cmp::{max, min};
use crate::rect::Rect;
use crate::components::Position;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_index(x: i32, y: i32) -> usize {
    let result = (y as usize * 80) + x as usize;
    if result >= 50 * 80 {
        print!("===> error: {}", result)
    }
    result
}

pub fn index_xy(index: usize) -> (usize, usize) {
    return (index % 80, (index / 80) as usize);
}


pub fn new_map() -> Vec<TileType> {
    //init map vector
    let mut map = vec![TileType::Floor; 80 * 50];

    for i in 0..=400 {
        let mut rng = RandomNumberGenerator::new();
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let index = xy_index(x, y);
        println!("x= {}, y= {}, i={}", x, y, xy_index(x, y));
        map[xy_index(x, y)] = TileType::Wall;
    }

    //generate wall box for the map
    for x in 0..=79 {
        map[xy_index(x, 0)] = TileType::Wall;
        map[xy_index(x, 49)] = TileType::Wall;
    }
    for y in 0..=49 {
        map[xy_index(0, y)] = TileType::Wall;
        map[xy_index(79, y)] = TileType::Wall;
    }
    // set the born point to floor
    map[xy_index(40, 25)] = TileType::Floor;
    map
}


pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    for (index, tile) in map.iter().enumerate() {
        let (x, y) = index_xy(index);
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0), to_cp437(' '))
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0), to_cp437('#'))
            }
        }
    }
}


pub fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_index(x, y)] = TileType::Floor;
        }
    }
}


fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_index(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_index(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}


pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80 * 50];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;
    let mut rng = RandomNumberGenerator::new();
    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);

            if !rooms.is_empty() {
                let Position { x: new_x, y: new_y } = new_room.center();
                let Position { x: prev_x, y: prev_y } = rooms[rooms.len() - 1].center();
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }
    (rooms, map)
}
