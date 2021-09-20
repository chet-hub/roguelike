use bracket_pathfinding::prelude::*;
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
use specs_derive::*;
use specs::prelude::*;
use std::cmp::{max, min};
use crate::{map::*, State, components::*, RunState};
use crate::gui::GameLog;

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Box<Map>>();
    let mut ppos = ecs.write_resource::<Point>();

    let mut combat_stats = ecs.write_storage::<CombatStats>();

    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();

    for (entity, _player, pos, viewshed) in (&entities, &players, &mut positions, &mut viewsheds).join() {
        let destination_index = xy_index(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_index].iter() {
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee.insert(entity, WantsToMelee { target: *potential_target }).expect("Add target failed");
                return;
            }
        }

        if map.blocked[destination_index] == false {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
            viewshed.dirty = true;
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    // Player movement
    match ctx.key {
        None => { return RunState::AwaitingInput; } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 |
            VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::C => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 |
            VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs),

            VirtualKeyCode::G => pickup_item(&mut gs.ecs),

            VirtualKeyCode::L => list_item(&mut gs.ecs),

            //VirtualKeyCode::F => use_item(-1, 1, &mut gs.ecs),

            _ => { return RunState::AwaitingInput; }
        },
    }
    RunState::PlayerTurn
}

fn pickup_item(ecs: &mut World) {
    let positions = ecs.read_storage::<Position>();
    let entities = ecs.entities();
    let items = ecs.read_storage::<Item>();
    let mut wants_to_pick_ups = ecs.write_storage::<WantsToPickUp>();
    let player_entity = ecs.fetch::<Entity>();
    let player_position = positions.get(*player_entity).unwrap();

    for (position, item, en) in (&positions, &items, &entities).join() {
        if position.x == player_position.x && position.y == player_position.y {
            let mut w = WantsToPickUp { owner: *player_entity, target: en };
            wants_to_pick_ups.insert(*player_entity, w).expect("Unable to insert want to pickup");
        } else {
            let mut log = ecs.fetch_mut::<GameLog>();
            log.entries.push("No item to pick up".parse().unwrap());
        }
    }
}


fn drop_off_item(ecs: &mut World) {

}

fn list_item(ecs: &mut World) {

}