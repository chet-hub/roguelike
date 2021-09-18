use bracket_pathfinding::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
use specs_derive::*;
use specs::prelude::*;
use std::cmp::{max, min};
use crate::components::Position;

const MAP_WIDTH : usize = 80;
const MAP_HEIGHT : usize = 43;
const MAP_COUNT : usize = MAP_HEIGHT * MAP_WIDTH;



#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_index(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH as i32) + x) as usize
}

pub fn index_xy(index: usize) -> (usize, usize) {
    (index % MAP_WIDTH, (index / MAP_WIDTH) as usize)
}

//
// pub fn new_map() -> Vec<TileType> {
//     //init map vector
//     let mut map = vec![TileType::Floor; MAP_WIDTH * MAP_HEIGHT];
//
//     for i in 0..=400 {
//         let mut rng = RandomNumberGenerator::new();
//         let x = rng.roll_dice(1, 79);
//         let y = rng.roll_dice(1, 49);
//         let index = xy_index(x, y);
//         println!("x= {}, y= {}, i={}", x, y, xy_index(x, y));
//         map[xy_index(x, y)] = TileType::Wall;
//     }
//
//     //generate wall box for the map
//     for x in 0..=79 {
//         map[xy_index(x, 0)] = TileType::Wall;
//         map[xy_index(x, 49)] = TileType::Wall;
//     }
//     for y in 0..=49 {
//         map[xy_index(0, y)] = TileType::Wall;
//         map[xy_index(79, y)] = TileType::Wall;
//     }
//     // set the born point to floor
//     map[xy_index(40, 25)] = TileType::Floor;
//     map
// }
//
//
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
        if idx > 0 && idx < MAP_WIDTH * MAP_HEIGHT {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_index(x, y);
        if idx > 0 && idx < MAP_WIDTH * MAP_HEIGHT {
            map[idx as usize] = TileType::Floor;
        }
    }
}


pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked : Vec<bool>,
    pub tile_content : Vec<Vec<Entity>>
}


impl BaseMap for Box<Map> {

    fn is_opaque(&self, _idx: usize) -> bool {
        self.tiles[_idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut result:SmallVec<[(usize, f32); 10]> = SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;
        if self.is_exit_valid(x - 1, y) { result.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x+1, y) { result.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { result.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y+1) { result.push((idx+w, 1.0)) };
        result
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

}

impl Algorithm2D for Box<Map> {


    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = self.dimensions();
        ((pt.y * bounds.x) + pt.x) as usize
    }

    fn index_to_point2d(&self, idx: usize) -> Point {
        let Point { x: w, y: h } = self.dimensions();
        Point::new((idx % w as usize) , (idx / w as usize) )
    }

    fn dimensions(&self) -> Point {
        Point { x: self.width, y: self.height }
    }


}



impl Map {
    pub fn position_to_index(&self, pt: &Position) -> usize {
        ((pt.y * self.width) + pt.x) as usize
    }

    fn is_exit_valid(&self,x:i32,y:i32) -> bool {
        if x < 1 || x > self.width - 1 || y <1 || y > self.height - 1 {
            return false;
        }
        let idx = xy_index(x,y);
        return self.blocked[idx] == false;
    }

    pub fn draw_map(&self, ctx: &mut BTerm) {
        for (index, revealed) in self.revealed_tiles.iter().enumerate() {
            if *revealed {
                let tile = self.tiles[index];
                let (x, y) = index_xy(index);
                if self.visible_tiles[index] {
                    match tile {
                        TileType::Floor => {
                            ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), to_cp437('.'))
                        }
                        TileType::Wall => {
                            ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.0, 0.0, 0.0), to_cp437('#'))
                        }
                    }
                } else {
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
        }
    }

    // pub fn set_visible_revealed_tiles(&mut self, p: Position) {
    //     let range = 8;
    //     self.visible_tiles.clear();
    //     let result = field_of_view(Point { x: p.x, y: p.y }, range, self);
    //     for p in result {
    //         let index = xy_index(p.x, p.y);
    //         self.visible_tiles[index] = true;
    //         self.revealed_tiles[index] = true;
    //     }
    // }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map: Map = Map {
            tiles: vec![TileType::Wall; MAP_WIDTH * MAP_HEIGHT],
            rooms: vec![],
            width: MAP_WIDTH as i32,
            height: MAP_HEIGHT as i32,
            revealed_tiles: vec![false; MAP_WIDTH * MAP_HEIGHT],
            visible_tiles: vec![false; MAP_WIDTH * MAP_HEIGHT],
            blocked:vec![false;MAP_WIDTH*MAP_HEIGHT],
            tile_content : vec![Vec::new(); MAP_WIDTH*MAP_HEIGHT]
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, (MAP_WIDTH - w as usize - 1) as i32) - 1;
            let y = rng.roll_dice(1, (MAP_HEIGHT - h as usize - 1) as i32) - 1;
            let new_room = Rect { x1: x, y1: y, x2: x + w, y2: y + h };
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                apply_room_to_map(&new_room, &mut map.tiles);

                if !map.rooms.is_empty() {
                    let Point { x: new_x, y: new_y } = new_room.center();
                    let Point { x: prev_x, y: prev_y } = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(&mut map.tiles, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut map.tiles, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut map.tiles, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut map.tiles, prev_x, new_x, new_y);
                    }
                }
                map.rooms.push(new_room);
            }
        }
        map.populate_blocked();
        map
    }

    pub fn populate_blocked(&mut self) {
        for (i,tile) in self.tiles.iter_mut().enumerate(){
            self.blocked[i] = (*tile==TileType::Wall)
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}



