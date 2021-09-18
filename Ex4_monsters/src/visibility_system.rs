use super::{Viewshed, Position, Map, Player};
use specs::prelude::*;
use bracket_pathfinding::prelude::{field_of_view, Algorithm2D};
use bracket_pathfinding::prelude::Point;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map,
            entities,
            mut viewshed,
            pos,
            player) = data;
        for (ent, viewshed, pos) in
        (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles = field_of_view(
                    Point::new(pos.x, pos.y),
                    viewshed.range,
                    &*map,
                );
                match player.get(ent) {
                    Some(p) => {
                        for t in map.visible_tiles.iter_mut() {
                            *t = false
                        };
                        for p in viewshed.visible_tiles.iter() {
                            let idx = map.point2d_to_index(*p);
                            map.revealed_tiles[idx] = true;
                            map.visible_tiles[idx] = true;
                        }
                    }
                    None => {}
                }
            }
        }
    }
}