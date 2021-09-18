use super::{Viewshed, Position, Map, Player};
use specs::{System, ReadStorage, join, Join, WriteStorage, WriteExpect, ReadExpect, Entities};
use bracket_terminal::prelude::Point;
use bracket_terminal::prelude::Console;
use crate::components::{Monster, Name, InputComponent};
use bracket_terminal::console::log;
use bracket_pathfinding::prelude::a_star_search;


pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Point>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InputComponent>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (point,
            viewShed,
            monster,
            name,
            mut inputComponent,
            Entities,
        ) = data;



        for (viewShed, _monster, name,input,en) in (&viewShed, &monster, &name,&mut inputComponent,&Entities).join() {
            // if input.accept_input == false {
            //     return
            // }
            //println!("system--AI->{:?}, {:?}",en,input);

            if viewShed.visible_tiles.contains(&point) {
                //chase the hero
                //todo
                //a_star_search((), (), &());
                log(&format!("{} shouts insults", name.name));
            }
            // input.accept_input = false
        }
    }
}