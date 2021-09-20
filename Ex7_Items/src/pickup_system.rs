use specs::prelude::*;
use super::{CombatStats, SufferDamage};
use crate::gui::GameLog;
use crate::components::{Name, Player, WantsToDropOff, Position, InBackpack, WantsToPickUp};
use crate::map::Map;
use bracket_pathfinding::prelude::{field_of_view, Point};

pub struct PickUpSystem {}


impl<'a> System<'a> for PickUpSystem {
    type SystemData = (
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToPickUp>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, InBackpack>,
        Entities<'a>
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut logger,
            mut wants_to_pickups,
            mut positions,
            mut inBackpacks,
            entities
        ) = data;

        for wants_to_pickup in (&wants_to_pickups).join() {
            inBackpacks.insert(wants_to_pickup.target,InBackpack{
                owner: wants_to_pickup.owner
            }).expect("unable to pickup it");
            positions.remove(wants_to_pickup.target);
            logger.entries.push(format!("pick up a item"));
        }
        wants_to_pickups.clear();
    }
}