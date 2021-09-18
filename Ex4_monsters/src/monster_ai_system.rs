use super::{Viewshed, Position, Map, Player};
use specs::prelude::*;
use bracket_terminal::prelude::Point;
use bracket_terminal::prelude::Console;
use crate::components::{Monster, Name};
use bracket_terminal::console::log;


pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (point,
            viewShed,
            monster,
            name
        ) = data;
        for (viewShed, _monster, name) in
        (&viewShed, &monster, &name).join() {
            if viewShed.visible_tiles.contains(&*point) {
                log(&format!("{} shouts insults", name.name));
            }
        }
    }
}