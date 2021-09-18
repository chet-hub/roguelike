use specs::{System, WriteExpect, ReadStorage, Join, Entities};
use crate::map::Map;
use crate::components::{Position, Renderable, BlocksTile};
use bracket_pathfinding::prelude::Algorithm2D;


pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = (
        WriteExpect<'a, Box<Map>>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position, blocksTiles, entities) = data;
        map.populate_blocked();
        map.clear_content_index();
        for (ent, pos) in (&entities, &position).join() {
            let index = map.position_to_index(pos);
            let _p : Option<&BlocksTile> = blocksTiles.get(ent);
            if let Some(_) = _p {
                map.blocked[index] = true;
            }
            map.tile_content[index].push(ent);
        }
    }
}