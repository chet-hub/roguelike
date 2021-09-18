
use bracket_pathfinding::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
use specs_derive::*;
use specs::prelude::*;
use std::cmp::{max, min};
use crate::{map, State,components::*};


fn try_move_player(delta_x:i32,delta_y:i32,ecs:&mut World){
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<map::TileType>>();

    for(_player,pos) in (&players, &mut positions).join(){
        let destination_index = map::xy_index(pos.x + delta_x,pos.y + delta_y);
        if map[destination_index] != map::TileType::Wall {
            pos.x = min(79,max(0,pos.x  +  delta_x));
            pos.y = min(49,max(0,pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx:&mut BTerm){
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