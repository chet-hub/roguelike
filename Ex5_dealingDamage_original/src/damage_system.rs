use specs::prelude::*;
use super::{CombatStats, SufferDamage};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (WriteStorage<'a, CombatStats>,
                       WriteStorage<'a, SufferDamage>,
                       Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage, entities) = data;

        for (mut stats, damage, en) in (&mut stats, &damage, &entities).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
            if stats.hp < 1 {
                entities.delete(en).expect("Unable to delete");
            }
        }

        damage.clear();
    }
}