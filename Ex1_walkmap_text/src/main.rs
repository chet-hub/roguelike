use bracket_pathfinding::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
use specs_derive::*;
use specs::prelude::*;
use std::cmp::{max, min};


////component
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}


////map

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

fn xy_index(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn index_xy(index: usize) -> (usize, usize) {
    return (index % 80,  (index / 80) as usize);
}


fn new_map() -> Vec<TileType> {
    //init map vector
    let mut map = vec![TileType::Floor; 80 * 50];

    for i in 0..=400 {
        let mut rng = RandomNumberGenerator::new();
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let index = xy_index(x,y);
        println!("x= {}, y= {}, i={}",x,y,xy_index(x, y));
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


// systems
fn try_move_player(delta_x:i32,delta_y:i32,ecs:&mut World){
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for(_player,pos) in (&mut players, &mut positions).join(){
        let destination_index = xy_index(pos.x + delta_x,pos.y + delta_y);
        if map[destination_index] != TileType::Wall {
            pos.x = min(79,max(0,pos.x  +  delta_x));
            pos.y = min(49,max(0,pos.y + delta_y));
        }
    }
}

fn player_input(gs: &mut State, ctx:&mut BTerm){
    match ctx.key {
        None =>{}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        }
    }
}


fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    for (index, tile) in map.iter().enumerate() {
        let (x, y) = index_xy(index);
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0), to_cp437('.'))
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0.0, 0.0, 0.0), to_cp437('#'))
            }
        }
    }
}


////GameState


struct State {
    ecs: World,
}


impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self,ctx);
        self.ecs.maintain();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map,ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos,render) in (&positions, &renderables).join() {
            ctx.set(pos.x,pos.y,render.fg,render.bg,render.glyph)
        }
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map());

    gs.ecs.create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .build();

    main_loop(context, gs)
}