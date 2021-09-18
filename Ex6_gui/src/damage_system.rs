use specs::prelude::*;
use super::{CombatStats, SufferDamage};
use crate::gui::GameLog;
use crate::components::{Name, Player};

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
        Entities<'a>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Player>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage, entities, names, mut log,players) = data;

        for (mut stats, damage, en) in (&mut stats, &damage, &entities).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
            if stats.hp < 1 {
                let player = players.get(en);
                match player {
                    None => {
                        let victim_name = names.get(en);
                        if let Some(victim_name) = victim_name {
                            log.entries.push(format!("{} is dead", &victim_name.name));
                        }
                    }
                    Some(_) => {
                        log.entries.push(format!("Hero was killed, game over!"));
                    }
                }

                entities.delete(en).expect("Unable to delete");
            }
        }

        damage.clear();
    }
}