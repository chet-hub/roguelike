//
// use bracket_pathfinding::prelude::*;
// use bracket_random::prelude::*;
// use bracket_terminal::prelude::*;
// use specs_derive::*;
// use specs::prelude::*;
// use std::cmp::{max, min};
// use crate::{map::*, State, components::*};
//
//
// fn try_move_player(delta_x:i32,delta_y:i32,ecs:&mut World){
//     let mut positions = ecs.write_storage::<Position>();
//     let mut players = ecs.write_storage::<Player>();
//     let mut viewsheds = ecs.write_storage::<Viewshed>();
//     let map = ecs.fetch::<Box<Map>>();
//     let mut ppos = ecs.write_resource::<Point>();
//
//     for(_player,pos,viewshed) in (&players, &mut positions,&mut viewsheds).join(){
//         let destination_index = xy_index(pos.x + delta_x,pos.y + delta_y);
//         //todo go fight
//         if map.blocked[destination_index] == false {
//             pos.x = min(79,max(0,pos.x  +  delta_x));
//             pos.y = min(49,max(0,pos.y + delta_y));
//             viewshed.dirty = true;
//             ppos.x = pos.x;
//             ppos.y = pos.y;
//         }
//     }
// }
//
// pub fn player_input(gs: &mut State, ctx:&mut BTerm) -> RunState {
//     // Player movement
//     match ctx.key {
//         None => { return RunState::AwaitingInput } // Nothing happened
//         Some(key) => match key {
//             VirtualKeyCode::Left |
//             VirtualKeyCode::Numpad4 |
//             VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
//
//             VirtualKeyCode::Right |
//             VirtualKeyCode::Numpad6 |
//             VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
//
//             VirtualKeyCode::Up |
//             VirtualKeyCode::Numpad8 |
//             VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
//
//             VirtualKeyCode::Down |
//             VirtualKeyCode::Numpad2 |
//             VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
//
//             VirtualKeyCode::Numpad9 |
//             VirtualKeyCode::E  => try_move_player(1,-1,&mut gs.ecs),
//
//             VirtualKeyCode::Numpad7 |
//             VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),
//
//             VirtualKeyCode::Numpad3 |
//             VirtualKeyCode::C => try_move_player(1, 1, &mut gs.ecs),
//
//             VirtualKeyCode::Numpad1 |
//             VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs),
//
//             _ => { return RunState::AwaitingInput }
//         },
//     }
//     RunState::PlayerTurn
// }