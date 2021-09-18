use specs::{System, ReadStorage, join, Join, WriteStorage, WriteExpect, ReadExpect, Entity, Entities};
use crate::map::*;
use bracket_pathfinding::prelude::Point;
use crate::components::{Viewshed, Player, Position, InputComponent};
use std::cmp::{min, max};
use bracket_terminal::prelude::*;

pub struct PlayerMoveSystem {}

// fn getDelta() -> (i32, i32) {
//     let input = INPUT.lock();
//
//     let mut result: (i32, i32) = (0, 0);
//
//     if input.is_key_pressed(VirtualKeyCode::Left) |
//         input.is_key_pressed(VirtualKeyCode::Numpad4) |
//         input.is_key_pressed(VirtualKeyCode::A) {
//         result = (-1, 0)
//     }
//
//     if input.is_key_pressed(VirtualKeyCode::Right) |
//         input.is_key_pressed(VirtualKeyCode::Numpad6) |
//         input.is_key_pressed(VirtualKeyCode::D) {
//         result = (1, 0)
//     }
//
//     if input.is_key_pressed(VirtualKeyCode::Up) |
//         input.is_key_pressed(VirtualKeyCode::Numpad8) |
//         input.is_key_pressed(VirtualKeyCode::W) {
//         result = (0, -1)
//     }
//
//     if input.is_key_pressed(VirtualKeyCode::Down) |
//         input.is_key_pressed(VirtualKeyCode::Numpad2) |
//         input.is_key_pressed(VirtualKeyCode::S) {
//         result = (0, 1)
//     }
//
//
//     if input.is_key_pressed(VirtualKeyCode::Numpad9) |
//         input.is_key_pressed(VirtualKeyCode::E) {
//         result = (1, -1)
//     }
//
//     if input.is_key_pressed(VirtualKeyCode::Numpad7) |
//         input.is_key_pressed(VirtualKeyCode::Q) {
//         result = (-1, -1)
//     }
//
//     if input.is_key_pressed(VirtualKeyCode::Numpad3) |
//         input.is_key_pressed(VirtualKeyCode::C) {
//         result = (1, 1)
//     }
//
//     if input.is_key_pressed(VirtualKeyCode::Numpad1) |
//         input.is_key_pressed(VirtualKeyCode::Z) {
//         result = (-1, 1)
//     }
//     result
// }

fn getDelta(input: &mut InputComponent) -> (i32, i32) {
    let mut result: (i32, i32) = (0, 0);
    if input.left {
        result = (-1, 0)
    }else if input.right {
        result = (1, 0)
    }else if input.up {
        result = (0, -1)
    }else if input.down {
        result = (0, 1)
    }else if input.right_up {
        result = (1, -1)
    }else if input.left_up {
        result = (1, -1)
    }else if input.right_down {
        result = (1, 1)
    }else if input.left_down {
        result = (-1, 1)
    }
    result
}

impl<'a> System<'a> for PlayerMoveSystem {
    type SystemData = (
        WriteExpect<'a, Box<Map>>,
        WriteExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, InputComponent>,
        Entities<'a>,
    );


    fn run(&mut self, data: Self::SystemData) {
        let (map,
            mut ppos,
            mut viewsheds,
            mut players,
            mut positions,
            mut inputComponent,
            mut entity
        ) = data;


        // let (delta_x, delta_y) = getDelta();

        for (_player, pos, viewshed,input,en) in (&players, &mut positions, &mut viewsheds,&mut inputComponent,&entity).join() {
            println!("system--player->id = {:?}",en);
            if input.accept_input == false {
                continue
            }
            println!("system--player->{:?}, {:?}",en,input);

            let (delta_x, delta_y) = getDelta(input);
            if delta_x == 0 && delta_y ==0 {
                continue
            }
            let destination_index = xy_index(pos.x + delta_x, pos.y + delta_y);
            //todo go fight
            if map.blocked[destination_index] == false {
                pos.x = min(79, max(0, pos.x + delta_x));
                pos.y = min(49, max(0, pos.y + delta_y));
                viewshed.dirty = true;
                ppos.x = pos.x;
                ppos.y = pos.y;

                input.accept_input = false
            }
        }
    }
}