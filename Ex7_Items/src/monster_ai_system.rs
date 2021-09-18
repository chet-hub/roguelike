use super::{Viewshed, Position, Map, Player};
use specs::{System, ReadStorage, join, Join, WriteStorage, WriteExpect, ReadExpect, Entities, Entity};
use bracket_terminal::prelude::{Point, DistanceAlg};
use bracket_terminal::prelude::Console;
use crate::components::{Monster, Name, WantsToMelee};
use bracket_terminal::console::log;
use bracket_pathfinding::prelude::{a_star_search, Algorithm2D};
use crate::RunState;


pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = ( WriteExpect<'a, Box<Map>>,
                        ReadExpect<'a, Point>,
                        ReadExpect<'a, Entity>,
                        ReadExpect<'a, RunState>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, WantsToMelee>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, player_pos, player_entity, runstate, entities, mut viewshed, monster, mut position, mut wants_to_melee) = data;

        for (entity, mut viewshed,_monster,mut pos) in (&entities, &mut viewshed, &monster, &mut position).join() {
            let distance = DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                wants_to_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to insert attack");
            }
            else if viewshed.visible_tiles.contains(&*player_pos) {
                // Path to the player
                let path = a_star_search(
                    map.point2d_to_index(Point{x:pos.x, y:pos.y}),
                    map.point2d_to_index(Point{x:player_pos.x, y:player_pos.y}),
                    &mut *map
                );
                if path.success && path.steps.len()>1 {
                    let mut idx = map.point2d_to_index(Point{x:pos.x, y:pos.y});
                    map.blocked[idx] = false;
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    idx = map.point2d_to_index(Point{x:pos.x, y:pos.y});
                    map.blocked[idx] = true;
                    viewshed.dirty = true;
                }
            }
        }
    }
}