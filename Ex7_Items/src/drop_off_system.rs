use specs::prelude::*;
use super::{CombatStats, SufferDamage};
use crate::gui::GameLog;
use crate::components::{Name, Player, WantsToDropOff, Position, InBackpack};
use crate::map::Map;
use bracket_pathfinding::prelude::{field_of_view, Point};

pub struct DropOffSystem {}

fn find_position_for_placing(pos: &Position, map: &Box<Map>) -> Option<Position> {
    for range in 1..10 {
        let points = field_of_view(Point::new(pos.x, pos.y), range, &*map);
        for p in points {
            let position = Position { x: p.x, y: p.y };
            if !map.blocked[map.position_to_index(&position)] {
                return Some(position);
            }
        }
    }
    return None;
}


impl<'a> System<'a> for DropOffSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, Box<Map>>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToDropOff>,
        WriteStorage<'a, InBackpack>,
        Entities<'a>
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut logger,
            mut map,
            mut positions,
            mut wants_to_drop_offs,
            mut inBackpacks,
            entities
        ) = data;

        for (wantsToDropOff,inBackpack,en) in (&wants_to_drop_offs, &inBackpacks, &entities).join() {
            let owner_position = positions.get(inBackpack.owner);
            if let Some(owner_position) = owner_position {
                let place_position = find_position_for_placing(owner_position, &map);
                if let Some(place_position) = place_position {
                    positions.insert(wantsToDropOff.target, place_position).expect("Can't insert Position component");
                } else {
                    logger.entries.push("No place to drop off".parse().unwrap());
                }
            }

            // {
            //     wants_to_drop_offs.remove(en);
            // }
        }

        wants_to_drop_offs.clear();
    }
}